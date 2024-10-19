#![feature(test)]
#![warn(clippy::pedantic, clippy::todo)]
#![cfg_attr(not(debug_assertions), warn(
    clippy::dbg_macro,
    clippy::use_debug,
    clippy::print_stdout,
    clippy::unimplemented,
))]
#![allow(
    clippy::too_many_lines,
    clippy::missing_panics_doc,
    clippy::wildcard_imports,
)]
#![recursion_limit = "1024"]

mod prelude;
mod style;
mod pages;
mod hobo_plus;
// mod benches;

use prelude::*;

#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
#[derive(Debug, Default)]
pub struct OvenPlayerLoaded(pub Mutable<bool>);

#[wasm_bindgen(start)]
pub fn main() {
	wasm_logger::init(wasm_logger::Config::default());
	console_error_panic_hook::set_once();

	wasm_bindgen_futures::spawn_local(async {
		log::info!("Main Thread Spawned");
		Resource::register_resource(Mutable::new(pages::Tab::from_url()));

		document().body().unwrap()
			.tap(|body| body.set_inner_html(""))
			.pipe(|body| e::Body(e::html_element(&body)))
			.allow_no_parent()
			.class(style::style())
			.class(css::background_color::rgba(colors::bg_black))
			.child(e::script()
				.attr(web_str::r#type(), "text/javascript")
				.src("../public/ovenplayer.js")
				.bool_attr(web_str::r#async(), true)
				.on_load(move |_| OvenPlayerLoaded::resource_or_default().0.set_neq(true))
			)
			.add_child(pages::body());
	});
}
