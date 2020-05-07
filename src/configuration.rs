use config;
use serde::Deserialize;
use std::env;
use std::path;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
}

impl Settings {
    // Creates new settings from compiled in default values, possibly overwritten by default values
    // in the config directory, then by more specific values matching the runtime environment given
    // with the `APP_ENVIRONMENT` environment variable, and finally overwritten by any environment
    // variables.
    pub fn new(base_path: path::PathBuf) -> Result<Self, config::ConfigError> {
        // running environment (optional)
        let environment = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".into());
        let path = base_path.join(&format!("config/{}", environment));

        let mut cfg = config::Config::new();
        cfg.set_default("server.address", "0.0.0.0")
            .and_then(|s| s.set_default("server.http.port", "3000"))
            .and_then(|s| s.merge(config::File::from(path).required(false)))
            .and_then(|s| s.merge(config::Environment::with_prefix("app").separator("_")))?;
        cfg.try_into()
    }
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub address: String,
    pub http: Http,
}

#[derive(Debug, Deserialize)]
pub struct Http {
    pub port: usize,
}
