use once_cell::sync::Lazy;
use std::{env, fmt::Display};

const DB: &str = "DB";
const DB_PORT: &str = "DB_PORT";
const HOST: &str = "HOST";
const HOST_PORT: &str = "HOST_PORT";

const PRODUCTION_STR: &str = "production";
const DEVELOPMENT_STR: &str = "development";

pub struct Config {
    pub db: String,
    pub db_port: String,
    pub host: String,
    pub host_port: String,
    pub env: ENV,
}

pub static ENV_CONFIG: Lazy<Config> = Lazy::new(|| Config {
    db: env::var("DB").expect("DATABASE_URL must be set"),
    db_port: env::var("DB_PORT").expect("DATABASE_URL must be set"),
    host: env::var("HOST").expect("DATABASE_URL must be set"),
    host_port: env::var("HOST_PORT").expect("DATABASE_URL must be set"),
    env: ENV::from(env::var("ENV").expect("ENV must be set")),
});

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let db = format!("{}:{}", DB, self.db);
        let db_port = format!("{}:{}", DB_PORT, self.db_port);
        let host = format!("{}:{}", HOST, self.host);
        let host_port = format!("{}:{}", HOST_PORT, self.host_port);
        let env = format!("{}:{}", "ENV", self.env);

        write!(f, "{}\n{}\n{}\n{}\n{}", db, db_port, host, host_port, env)
    }
}

pub fn set_environment() -> Result<String, String> {
    let environment = env::args()
        .nth(1)
        .unwrap_or_else(|| ENV::Development.to_string());

    if let Err(e) = dotenv::from_filename(format!(".env.{}", environment)) {
        panic!("Failed to load .env.{} file: {}", environment, e)
    };

    println!("Environment: {}", environment);
    println!("Configs: {}", ENV_CONFIG.to_string());

    Ok(String::from("Environment Successfully Set!"))
}
pub enum ENV {
    Development,
    Production,
}

impl From<String> for ENV {
    fn from(value: String) -> Self {
        match value.as_str() {
            PRODUCTION_STR => ENV::Production,
            DEVELOPMENT_STR => ENV::Development,
            _ => ENV::Development,
        }
    }
}

impl Display for ENV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let env = match self {
            ENV::Development => DEVELOPMENT_STR,
            ENV::Production => PRODUCTION_STR,
        };

        write!(f, "{}", env)
    }
}
