//! Stub — shared session settings (kept for settings framework compatibility).

use std::time::Duration;

use settings::{
    macros::define_settings_group, RespectUserSyncSetting, Setting, SupportedPlatforms, SyncToCloud,
};

define_settings_group!(SharedSessionSettings, settings: [
    viewer_driven_sizing_enabled: ViewerDrivenSizingEnabled {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        private: true,
    },
]);

impl SharedSessionSettings {
    pub fn inactivity_period_between_warning_and_ending_session(&self) -> Duration {
        Duration::from_secs(300)
    }
}
