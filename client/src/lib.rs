#![recursion_limit = "1024"]

mod style;
mod prelude;
mod windows;

use prelude::*;

fn root(element: &web_sys::HtmlElement) {
	let element = cmp::Body(hobo::create::html_element(element));
	element.child(windows::main::App::new_element());
}

#[wasm_bindgen(start)]
pub fn main() {
	wasm_logger::init(wasm_logger::Config::default());
	console_error_panic_hook::set_once();

	wasm_bindgen_futures::spawn_local(async {
		log::info!("Main Thread Spawned");

		let body = web_sys::window().unwrap().document().unwrap().body().unwrap();
		body.set_inner_html("");
		body.set_attribute(web_str::class(), &hobo::fetch_classname(style::style())).unwrap();

		root(&body);
	});
}
