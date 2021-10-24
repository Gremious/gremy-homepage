#![warn(clippy::pedantic, clippy::todo)]
#![cfg_attr(not(debug_assertions), warn(
	clippy::dbg_macro,
	clippy::use_debug,
	clippy::print_stdout,
	clippy::unimplemented,
))]
#![allow(
	clippy::missing_errors_doc,
	clippy::too_many_lines,
	clippy::missing_panics_doc,
	clippy::wildcard_imports,
)]

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
