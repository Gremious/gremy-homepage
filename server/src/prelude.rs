pub use anyhow::Context;
pub use once_cell::sync::Lazy;
pub use serde::{Deserialize, Serialize};

pub static CONFIG: Lazy<Config> = Lazy::new(|| toml::from_str(include_str!("../Config.toml")).expect("failed to parse config"));

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub http_port: u16,
    pub https_port: u16,
    pub https: bool,
    pub ssl: SslConfig,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SslConfig {
    pub key: String,
    pub cert: String,
}
