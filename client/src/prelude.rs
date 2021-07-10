pub use crate::{/*svgs, images, widgets::*,*/ global_style::*};
pub use core::future::Future;
pub use std::{
	cell::RefCell,
	rc::Rc
};
pub use anyhow::Context;
pub use hobo::{create::components as cmp, enclose as e, events, prelude::*, state};
pub use once_cell::sync::{Lazy, OnceCell};
pub use serde::Deserialize;
pub use smart_default::SmartDefault;
pub use wasm_bindgen_futures::spawn_local as spawn;
use extend as ext;
// pub use crate::hobo_plus::*;

pub fn window() -> web_sys::Window { web_sys::window().expect("no window") }
pub fn document() -> web_sys::Document { window().document().expect("no document") }

pub static CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);
pub static EVENTS: Lazy<events::Events> = Lazy::new(default);

pub fn spawn_complain<T>(x: impl Future<Output = anyhow::Result<T>> + 'static) {
	spawn(async move { if let Err(e) = x.await { log::error!("{:?}", e); } });
}

#[async_trait::async_trait(?Send)]
trait ReqwestWasmResponseExt {
	async fn json<T: serde::de::DeserializeOwned>(self) -> anyhow::Result<T>;
}

#[async_trait::async_trait(?Send)]
impl ReqwestWasmResponseExt for reqwest::Response {
	async fn json<T: serde::de::DeserializeOwned>(self) -> anyhow::Result<T> {
		Ok(serde_json::from_str(&self.text().await?)?)
	}
}
