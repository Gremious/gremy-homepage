use super::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Options {
	title: String,
	sources: Vec<Source>,
}

impl Default for Options {
	fn default() -> Self {
		let shared::Key {
			name: stream_name,
			pass: stream_key,
		} = shared::CONFIG.stream_keys.iter().find(|x| &x.name == "gremy-default").expect("no gremy stream key?");

		Self {
			title: "gremy player".to_owned(),
			sources: vec![
				Source {
					label: String::from("bypass_stream"),
					stream_type: String::from("webrtc"),
					// file: format!("wss://stream.gremy.co.uk:3333/app/stream"),
					file: format!("wss://stream.gremy.co.uk:3333/{stream_name}/{stream_key}"),
				},
			],
		}
	}
}

#[derive(Serialize, Deserialize)]
struct Source {
	label: String,
	#[serde(rename = "type")]
	stream_type: String,
	file: String,
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_name = OvenPlayer)]
	pub type Player;

	#[wasm_bindgen(constructor, js_class = create, js_namespace = OvenPlayer)]
	fn create(player_div_id: &str, options: JsValue) -> Player;
}

pub fn new() -> e::Div {
	container()
		// .child(media_player())
}

fn container() -> e::Div {
	e::div()
		// .class(css::style!(
		//     .& {
		//         css::Position::Relative,
		//         css::Display::Flex,
		//         css::FlexDirection::Column,
		//         css::JustifyContent::FlexStart,
		//         css::BoxSizing::BorderBox,
		//         css::overflow!(hidden),
		//         css::padding!(2.5%),
		//     }
        //
		//     @media All && MaxWidth(css::unit!(500 px)) {
		//         .& { "background-size: cover;" }
		//     }
		// ))
}

fn media_player() -> e::Div {
	let load_state = crate::OvenPlayerLoaded::resource_or_default();

	e::div()
		.class((
			css::width::px(1280),
			css::height::px(720),
			css::background_image::Some(vec![
				css::Image::RepeatingLinearGradient(css::LinearGradient {
					angle: F32::new(-45.0).unwrap(),
					stop_list: vec![
						(css::Color::new(0, 0, 0, 0), css::Unit::px(0)),
						(css::Color::new(0, 0, 0, 0), css::Unit::px(20)),
						(css::Color::new(190, 25, 90, 127), css::Unit::px(20)),
						(css::Color::new(190, 25, 90, 127), css::Unit::px(40)),
					],

				}),
			]),
			css::background_color::rgba(css::colors::BLACK),
			css::resize::both,
			css::overflow::auto,
		))
		.child(e::div()
			.id("main_player")
		)
		.with_component(move |&element| load_state.signal().subscribe(move |loaded| {
 			if !loaded { return; }
			element
				.tap(|_| {
					Player::create("main_player", serde_wasm_bindgen::to_value(&Options::default()).unwrap());
				});
		}))
}
