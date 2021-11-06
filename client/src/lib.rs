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
mod windows;
pub mod hobo_plus;
mod benches;

use prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(async {
        log::info!("Main Thread Spawned");
        Resource::register_resource(Mutable::new(windows::Tab::default()));

        let body = web_sys::window().unwrap().document().unwrap().body().unwrap();
        body.set_inner_html("");
        body.set_attribute(web_str::class(), &hobo::fetch_classname(style::style())).unwrap();

        let hobo_body = cmp::Body(hobo::create::html_element(&body));
        hobo_body.add_child(windows::Root::new_element());
    });
}
