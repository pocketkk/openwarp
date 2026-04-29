use std::sync::Arc;

use futures::channel::oneshot;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp_multi_agent_api as api;

use super::super::{AIAgentInput, MCPContext};
use crate::ai::agent::api::{ConvertToAPITypeError, RequestParams, ResponseStream};
use crate::server::server_api::AIApiError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalLLMConfig {
    pub enabled: bool,
    pub endpoint_url: String,
    pub model_name: String,
    pub api_key: Option<String>,
}

impl Default for LocalLLMConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint_url: "http://localhost:11434/v1".into(),
            model_name: "llama3.1".into(),
            api_key: None,
        }
    }
}

impl LocalLLMConfig {
    pub fn load() -> Self {
        let config_path = dirs::config_dir()
            .map(|d| d.join("warp").join("local_llm.json"))
            .unwrap_or_default();
        std::fs::read_to_string(&config_path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    #[allow(dead_code)]
    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = dirs::config_dir()
            .map(|d| d.join("warp").join("local_llm.json"))
            .ok_or_else(|| anyhow::anyhow!("No config dir"))?;
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&config_path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<serde_json::Value>>,
}

#[derive(Deserialize)]
struct ChatCompletionChunk {
    choices: Vec<ChunkChoice>,
}

#[derive(Deserialize)]
struct ChunkChoice {
    delta: ChunkDelta,
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct ChunkDelta {
    #[serde(default)]
    content: Option<String>,
}

fn extract_user_query(inputs: &[AIAgentInput]) -> String {
    for input in inputs.iter().rev() {
        match input {
            AIAgentInput::UserQuery { query, .. } => return query.clone(),
            AIAgentInput::AutoCodeDiffQuery { query, .. } => return query.clone(),
            AIAgentInput::ResumeConversation { .. } => {
                return "Please continue where you left off.".into()
            }
            AIAgentInput::CreateNewProject { query, .. } => return query.clone(),
            _ => continue,
        }
    }
    String::new()
}

fn extract_system_context(inputs: &[AIAgentInput]) -> Option<String> {
    for input in inputs {
        let context = match input {
            AIAgentInput::UserQuery { context, .. } => context,
            AIAgentInput::AutoCodeDiffQuery { context, .. } => context,
            AIAgentInput::ResumeConversation { context, .. } => context,
            _ => continue,
        };
        let mut parts = Vec::new();
        for ctx in context.iter() {
            match ctx {
                super::super::AIAgentContext::Directory {
                    pwd, home_dir, ..
                } => {
                    if let Some(pwd) = pwd {
                        parts.push(format!("Working directory: {pwd}"));
                    }
                    if let Some(home) = home_dir {
                        parts.push(format!("Home directory: {home}"));
                    }
                }
                super::super::AIAgentContext::ExecutionEnvironment(env) => {
                    if let Some(cat) = &env.os.category {
                        parts.push(format!("OS: {cat}"));
                    }
                    parts.push(format!("Shell: {}", env.shell_name));
                }
                _ => {}
            }
        }
        if !parts.is_empty() {
            return Some(parts.join("\n"));
        }
    }
    None
}

fn build_tool_definitions(mcp_context: &Option<MCPContext>) -> Option<Vec<serde_json::Value>> {
    let mcp = mcp_context.as_ref()?;
    let mut tools = Vec::new();
    for server in &mcp.servers {
        for tool in &server.tools {
            let params = serde_json::Value::Object((*tool.input_schema).clone());
            tools.push(serde_json::json!({
                "type": "function",
                "function": {
                    "name": &*tool.name,
                    "description": tool.description.as_deref().unwrap_or(""),
                    "parameters": params,
                }
            }));
        }
    }
    if tools.is_empty() { None } else { Some(tools) }
}

pub async fn generate_local_llm_output(
    config: LocalLLMConfig,
    params: RequestParams,
    cancellation_rx: oneshot::Receiver<()>,
) -> Result<ResponseStream, ConvertToAPITypeError> {
    let conversation_id = params
        .conversation_token
        .as_ref()
        .map(|t| t.as_str().to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    let request_id = Uuid::new_v4().to_string();
    let task_id = Uuid::new_v4().to_string();

    let user_query = extract_user_query(&params.input);
    let system_context = extract_system_context(&params.input);
    let tools = build_tool_definitions(&params.mcp_context);

    let mut messages = Vec::new();

    if let Some(sys) = system_context {
        messages.push(ChatMessage {
            role: "system".into(),
            content: format!(
                "You are an AI assistant embedded in a terminal. Be concise and helpful.\n\n{sys}"
            ),
        });
    } else {
        messages.push(ChatMessage {
            role: "system".into(),
            content: "You are an AI assistant embedded in a terminal. Be concise and helpful."
                .into(),
        });
    }

    messages.push(ChatMessage {
        role: "user".into(),
        content: user_query,
    });

    let chat_request = ChatCompletionRequest {
        model: config.model_name.clone(),
        messages,
        stream: true,
        tools,
    };

    let client = reqwest::Client::new();
    let url = format!("{}/chat/completions", config.endpoint_url.trim_end_matches('/'));
    let mut req = client.post(&url).json(&chat_request);
    if let Some(key) = &config.api_key {
        if !key.is_empty() {
            req = req.bearer_auth(key);
        }
    }

    let response = req.send().await.map_err(|e| {
        ConvertToAPITypeError::from(anyhow::anyhow!("Local LLM request failed: {e}"))
    })?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        let (tx, rx) = async_channel::unbounded();
        let _ = tx
            .send(Err(Arc::new(AIApiError::Other(anyhow::anyhow!(
                "Local LLM returned {status}: {body}"
            )))))
            .await;
        return Ok(Box::pin(rx));
    }

    let conv_id = conversation_id.clone();
    let req_id = request_id.clone();
    let t_id = task_id.clone();

    let (tx, rx) = async_channel::unbounded::<super::Event>();

    let init_event = api::ResponseEvent {
        r#type: Some(api::response_event::Type::Init(
            api::response_event::StreamInit {
                conversation_id: conv_id.clone(),
                request_id: req_id.clone(),
                run_id: String::new(),
            },
        )),
    };
    let _ = tx.send(Ok(init_event)).await;

    let initial_task = api::Task {
        id: t_id.clone(),
        description: String::new(),
        dependencies: None,
        messages: vec![],
        summary: String::new(),
        server_data: String::new(),
    };
    let create_task_action = api::ClientAction {
        action: Some(api::client_action::Action::CreateTask(
            api::client_action::CreateTask {
                task: Some(initial_task),
            },
        )),
    };
    let _ = tx
        .send(Ok(api::ResponseEvent {
            r#type: Some(api::response_event::Type::ClientActions(
                api::response_event::ClientActions {
                    actions: vec![create_task_action],
                },
            )),
        }))
        .await;

    tokio::spawn(async move {
        let mut raw_text = String::new();
        let mut visible_text = String::new();
        let message_id = Uuid::new_v4().to_string();
        let mut first_content = true;
        let mut inside_think = false;

        use futures::StreamExt;
        let mut byte_stream = response.bytes_stream();
        let mut buffer = String::new();

        let mut cancelled = cancellation_rx;

        loop {
            tokio::select! {
                _ = &mut cancelled => {
                    break;
                }
                chunk = byte_stream.next() => {
                    let Some(chunk) = chunk else { break; };
                    let Ok(bytes) = chunk else { break; };
                    buffer.push_str(&String::from_utf8_lossy(&bytes));

                    while let Some(line_end) = buffer.find('\n') {
                        let line = buffer[..line_end].trim().to_string();
                        buffer = buffer[line_end + 1..].to_string();

                        if line.is_empty() || line == "data: [DONE]" {
                            continue;
                        }
                        let json_str = line.strip_prefix("data: ").unwrap_or(&line);
                        let Ok(chunk) = serde_json::from_str::<ChatCompletionChunk>(json_str) else {
                            continue;
                        };

                        for choice in &chunk.choices {
                            if let Some(content) = &choice.delta.content {
                                raw_text.push_str(content);

                                // Strip <think>...</think> blocks (qwen3, deepseek, etc.)
                                if !inside_think {
                                    if let Some(start) = content.find("<think>") {
                                        visible_text.push_str(&content[..start]);
                                        inside_think = true;
                                    } else {
                                        visible_text.push_str(content);
                                    }
                                }
                                if inside_think {
                                    if raw_text.contains("</think>") {
                                        inside_think = false;
                                        if let Some(end_pos) = raw_text.rfind("</think>") {
                                            let after = &raw_text[end_pos + 8..];
                                            visible_text.clear();
                                            visible_text.push_str(after.trim_start());
                                        }
                                    } else {
                                        continue;
                                    }
                                }

                                if visible_text.is_empty() {
                                    continue;
                                }

                                let message = api::Message {
                                    id: message_id.clone(),
                                    task_id: t_id.clone(),
                                    request_id: req_id.clone(),
                                    timestamp: None,
                                    server_message_data: String::new(),
                                    citations: vec![],
                                    message: Some(api::message::Message::AgentOutput(
                                        api::message::AgentOutput {
                                            text: visible_text.clone(),
                                        },
                                    )),
                                };

                                let action = if first_content {
                                    first_content = false;
                                    api::client_action::Action::AddMessagesToTask(
                                        api::client_action::AddMessagesToTask {
                                            task_id: t_id.clone(),
                                            messages: vec![message],
                                        },
                                    )
                                } else {
                                    api::client_action::Action::UpdateTaskMessage(
                                        api::client_action::UpdateTaskMessage {
                                            task_id: t_id.clone(),
                                            message: Some(message),
                                            mask: None,
                                        },
                                    )
                                };

                                let event = api::ResponseEvent {
                                    r#type: Some(api::response_event::Type::ClientActions(
                                        api::response_event::ClientActions {
                                            actions: vec![api::ClientAction {
                                                action: Some(action),
                                            }],
                                        },
                                    )),
                                };
                                if tx.send(Ok(event)).await.is_err() {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }

        let finished_event = api::ResponseEvent {
            r#type: Some(api::response_event::Type::Finished(
                api::response_event::StreamFinished {
                    token_usage: vec![],
                    should_refresh_model_config: false,
                    request_cost: None,
                    conversation_usage_metadata: None,
                    reason: Some(api::response_event::stream_finished::Reason::Done(
                        api::response_event::stream_finished::Done {},
                    )),
                },
            )),
        };
        let _ = tx.send(Ok(finished_event)).await;
    });

    Ok(Box::pin(rx))
}
