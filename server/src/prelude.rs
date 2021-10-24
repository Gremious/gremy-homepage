pub use actix_web::rt::spawn;
pub use anyhow::Context;
pub use futures::prelude::*;
pub use once_cell::sync::Lazy;
pub use serde::{Deserialize, Serialize};
pub use shared::IntoResOpt as _;

pub static CONFIG: Lazy<Config> = Lazy::new(|| toml::from_str(include_str!("../Config.toml")).expect("failed to parse config"));

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub http_port: u16,
    pub https_port: u16,
    pub https: bool,
    pub salt: String,
    pub ssl: SslConfig,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SslConfig {
    pub key: String,
    pub cert: String,
}

#[allow(dead_code)]
pub fn spawn_complain<T>(x: impl Future<Output = anyhow::Result<T>> + 'static) {
    spawn(async move {
        if let Err(e) = x.await {
            log::error!("{:?}", e);
        }
    });
}
