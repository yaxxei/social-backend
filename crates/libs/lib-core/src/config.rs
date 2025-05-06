use std::sync::OnceLock;

pub fn core_config() -> &'static CoreConfig {
    static CORE_CONFIG: OnceLock<CoreConfig> = OnceLock::new();
    CORE_CONFIG.get_or_init(|| {
        CoreConfig::load_from_env()
            .unwrap_or_else(|err| panic!("PANIC WHILE LOADING CORE CONFIG: {}", err))
    })
}

pub struct CoreConfig {
    db_url: String,
    db_max_conn: u32,
}

impl CoreConfig {
    fn load_from_env() -> lib_utils::env::Result<Self> {
        Ok(Self {
            db_url: lib_utils::env::get_env("DATABASE_URL")?,
            db_max_conn: lib_utils::env::get_parsed_env("DATABASE_MAX_CONNECTIONS")?,
        })
    }

    pub fn db_url(&self) -> &str {
        &self.db_url
    }

    pub fn db_max_conn(&self) -> &u32 {
        &self.db_max_conn
    }
}
