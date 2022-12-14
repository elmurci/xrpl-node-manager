use config::{Config, ConfigError, Environment, File};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::{env, fmt};

lazy_static! {
  static ref SETTINGS: Settings = {
    match Settings::new() {
      Ok(settings) => settings,
      Err(err) => panic!("Failed to setup settings: {}", err),
    }
  };
}

pub fn get_settings() -> &'static Settings {
  &SETTINGS
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
  pub port: u16,
  pub refresh: u64
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logger {
  pub level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
  pub environment: String,
  pub config: String,
  pub server: Server,
  pub rpc_endpoint: String,
  pub logger: Logger,
}

impl Settings {
  pub fn new() -> Result<Self, ConfigError> {
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

    let mut builder = Config::builder()
      .add_source(File::with_name("config/default"))
      .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
      .add_source(File::with_name("config/local").required(false))
      .add_source(Environment::default().separator("__"));

    if let Ok(port) = env::var("PORT") {
      builder = builder.set_override("server.port", port)?;
    }

    builder
      .build()?
      .try_deserialize()
  }
}

impl fmt::Display for Server {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "http://localhost:{}", &self.port)
  }
}
