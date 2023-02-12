pub use crate::{style::*, hobo_plus::*};
pub use anyhow::Context;
pub use core::future::Future;
pub use hobo::{
	prelude::*,
    create as e,
    signals::signal::{Mutable, SignalExt}
};
pub use tap::prelude::*;
pub use once_cell::sync::{Lazy, OnceCell};
pub use serde::Deserialize;
pub use smart_default::SmartDefault;
pub use std::cell::RefCell;
pub use std::rc::Rc;
pub use wasm_bindgen_futures::spawn_local as spawn;

pub fn window() -> web_sys::Window { web_sys::window().expect("no window") }
pub fn document() -> web_sys::Document { window().document().expect("no document") }
