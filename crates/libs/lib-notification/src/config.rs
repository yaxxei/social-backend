use std::sync::OnceLock;

pub fn notification_config() -> &'static NotificationConfig {
    static NOTIFICATION_CONFIG: OnceLock<NotificationConfig> = OnceLock::new();
    NOTIFICATION_CONFIG.get_or_init(|| {
        NotificationConfig::load_from_env()
            .unwrap_or_else(|err| panic!("PANIC WHILE LOADING NOTIFICATION CONFIG: {}", err))
    })
}

pub struct NotificationConfig {
    smtp_username: String,
    smtp_password: String,
}

impl NotificationConfig {
    fn load_from_env() -> lib_utils::env::Result<Self> {
        Ok(Self {
            smtp_username: lib_utils::env::get_env("JWT_ACCESS_SECRET")?,
            smtp_password: lib_utils::env::get_env("JWT_REFRESH_SECRET")?,
        })
    }

    pub fn smtp_username(&self) -> &str {
        &self.smtp_username
    }

    pub fn smtp_password(&self) -> &str {
        &self.smtp_password
    }
}
