use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub project_name: String,
    pub name: String,
    pub hostname: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| toml::from_str(include_str!("../../Config.toml")).expect("failed to parse config"));

pub trait IntoResOpt: Sized {
    fn into_ok<E>(self) -> Result<Self, E>;
    fn into_some(self) -> Option<Self>;
}

impl<T> IntoResOpt for T {
    fn into_ok<E>(self) -> Result<Self, E> { Ok(self) }
    fn into_some(self) -> Option<Self> { Some(self) }
}
