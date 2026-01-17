use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_path: String,
    pub default_limit: u32,
    pub max_limit: u32,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            database_path: env::var("DATABASE_PATH")
                .unwrap_or_else(|_| "./data/settings.db".to_string()),
            default_limit: env::var("DEFAULT_LIMIT")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .expect("DEFAULT_LIMIT must be a valid u32"),
            max_limit: env::var("MAX_LIMIT")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .expect("MAX_LIMIT must be a valid u32"),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a valid u16"),
        })
    }
}
