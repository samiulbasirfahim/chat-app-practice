use dotenvy::dotenv;
use std::path::PathBuf;
use std::sync::OnceLock;

pub struct Config {
    pub port: u16,
    pub db_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub upload_dir: PathBuf,
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
        db_url: Config::env("DATABASE_URL"),
        redis_url: Config::env("REDIS_URL"),
        jwt_secret: Config::env("JWT_SECRET"),
        upload_dir: PathBuf::from(Config::env("UPLOAD_DIR")),
    })
}
