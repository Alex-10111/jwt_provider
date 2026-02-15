use std::env;
use std::sync::LazyLock;
use dotenv::dotenv;
use crate::utils::get_exp;


pub struct Config {
    host: String,
    port: String,
    jwt_secret: String,
    jwt_lifetime_seconds: usize,
}

impl Config {
    pub fn new () -> Config {
        dotenv().ok();

        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");
        let jwt_lifetime_minutes = env::var("JWT_LIFETIME_MINUTES").expect("JWT_LIFETIME_MINUTES not set")
            .parse::<u8>().expect("JWT_LIFETIME_MINUTES not a number");
        let jwt_lifetime_seconds = get_exp(jwt_lifetime_minutes);

        Self {
            host,
            port,
            jwt_secret,
            jwt_lifetime_seconds,
        }
    }

    pub fn host (&self) -> &str { &self.host }
    pub fn port (&self) -> &str { &self.port }
    pub fn jwt_secret (&self) -> &str { &self.jwt_secret }
    pub fn jwt_lifetime_seconds(&self) -> usize { self.jwt_lifetime_seconds }
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::new());

pub fn get_config() -> &'static Config {
    &CONFIG
}
