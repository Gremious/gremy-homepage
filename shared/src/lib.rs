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

pub mod prelude;
use prelude::*;

use core::future::Future;
use wasm_bindgen_futures::spawn_local as spawn;

#[derive(Serialize, Deserialize)]
pub struct Config {
	pub dev: bool,
    pub hostname: String,
	pub stream_keys: HashMap<String, String>,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| toml::from_str(include_str!("../../Config.toml")).expect("failed to parse config"));
pub static REQWEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);

#[allow(dead_code)]
pub fn spawn_complain<T>(x: impl Future<Output = anyhow::Result<T>> + 'static) {
	spawn(async move { if let Err(e) = x.await { log::error!("{:?}", e); } });
}
