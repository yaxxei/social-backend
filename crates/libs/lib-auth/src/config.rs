use std::sync::OnceLock;

pub fn auth_config() -> &'static AuthConfig {
    static AUTH_CONFIG: OnceLock<AuthConfig> = OnceLock::new();
    AUTH_CONFIG.get_or_init(|| {
        AuthConfig::load_from_env()
            .unwrap_or_else(|err| panic!("PANIC WHILE LOADING AUTH CONFIG: {}", err))
    })
}

pub struct AuthConfig {
    jwt_access_secret: String,
    jwt_refresh_secret: String,
    jwt_reset_password_secret: String,
}

impl AuthConfig {
    fn load_from_env() -> lib_utils::env::Result<Self> {
        Ok(Self {
            jwt_access_secret: lib_utils::env::get_env("JWT_ACCESS_SECRET")?,
            jwt_refresh_secret: lib_utils::env::get_env("JWT_REFRESH_SECRET")?,
            jwt_reset_password_secret: lib_utils::env::get_env("JWT_RESET_PASSWORD_SECRET")?,
        })
    }

    pub fn jwt_access_secret(&self) -> &str {
        &self.jwt_access_secret
    }

    pub fn jwt_refresh_secret(&self) -> &str {
        &self.jwt_refresh_secret
    }

    pub fn jwt_reset_password_secret(&self) -> &str {
        &self.jwt_reset_password_secret
    }
}
