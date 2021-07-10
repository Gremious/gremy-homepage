pub use crate::{
	// db::{self, DB, heedRwTxnepTExt as _, heedRoTxneTExt as _},
	// oauth::CredentialsSecret as _,
};
pub use actix::prelude::*;
pub use actix_web::web;
pub use anyhow::Context;
// pub use async_trait::async_trait;
// pub use cascade::cascade;
// pub use itertools::Itertools as _;
pub use once_cell::sync::Lazy;
// pub use rand::Rng;
pub use reqwest::header;
pub use serde::{Deserialize, Serialize};
pub use shared::{/*enone, serde_jsonValueExt as _, default, */IntoResOpt as _};
pub use std::{
	collections::{BTreeMap, HashMap, HashSet},
	convert::TryFrom,
};
// pub use sugars::*;
// pub use typed_builder::TypedBuilder;
pub use futures::prelude::*;
// pub use timed::timed;
pub use actix_web::rt::spawn;
// pub use fehler::{throw, throws};
// pub use smart_default::SmartDefault;
// pub use chrono::{Datelike as _, TimeZone as _};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SslConfig {
	pub key: String,
	pub cert: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
	pub port: String,
	pub salt: String,
	pub ssl: SslConfig,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| toml::from_str(include_str!("../Config.toml")).expect("failed to parse config"));
pub static CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);

pub fn spawn_complain<T>(x: impl Future<Output = anyhow::Result<T>> + 'static) {
	spawn(async move { if let Err(e) = x.await { log::error!("{:?}", e); } });
}
