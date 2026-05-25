use dotenvy::dotenv;
use std::sync::OnceLock;

pub struct Config {
    pub port: u16,
    pub db_uri: String,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    fn env(key: &str) -> String {
        dotenv().ok();
        std::env::var(key).unwrap_or_else(|_| panic!("Environment variable {} not found", key))
    }
}

pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| Config {
        port: Config::env("PORT")
            .parse()
            .expect("PORT must be a valid number"),
        db_uri: Config::env("DATABASE_URI"),
    })
}
