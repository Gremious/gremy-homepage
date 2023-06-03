use shared::{spawn_complain, REQWEST_CLIENT};
use serde::{Deserialize, Serialize};
use super::*;

// TODO: Re-write animation using hobo macros

// TODO: Easter eggs
// - Hide something behind lain image.
// - Type "hi" - type out smiley face to hello text e.g. "Hello :)_"t
// - Type "hello" - secret item fly-in?
// - Template page lol

struct Clicked(bool);

pub fn new() -> e::Div {
	container()
		.child(lain_image())
		.child(hello_header())
		.child(homepage_nav())
		.child(media_player())
}

#[derive(Serialize, Deserialize)]
struct Options {
	title: String,
	sources: Vec<Source>,
}

impl Default for Options {
	fn default() -> Self {
		let config = String::from_utf8_lossy(include_bytes!("/home/gremious/.config/gremy-stream/config.conf"));

		let (stream_name, stream_key) = config.lines().next().unwrap().split_once(';')
			.expect("Put a 'stream name;stream key' as a config file in .config/gremy-stream/config.conf");

		log::debug!("wss://stream.gremy.co.uk:3334/{stream_name}/{stream_key}");

		Self {
			title: "gremy player".to_owned(),
			sources: vec![
				Source {
					label: String::from("bypass_stream"),
					stream_type: String::from("webrtc"),
					file: format!("wss://stream.gremy.co.uk:3334/{stream_name}/{stream_key}"),
				}
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

fn media_player() -> e::Div {
	let load_state = crate::OvenPlayerLoaded::resource_or_default();

	e::div().class((css::width!(1280 px), css::height!(720 px)))
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
fn container() -> e::Div {
	e::div()
		.class(css::style!(
			.& {
				css::Position::Relative,
				css::Display::Flex,
				css::FlexDirection::Column,
				css::JustifyContent::FlexStart,
				css::BoxSizing::BorderBox,
				css::overflow!(hidden),
				css::padding!(2.5%),
				css::background_color!(colors::bg_black),
			}

			@media All && MaxWidth(css::unit!(500 px)) {
				.& { "background-size: cover;" }
			}
		))
}

fn lain_image() -> e::Div {
	e::div()
		.class((
			css::Position::Absolute,
			css::top!(50% - 300 px),
			css::left!(50% - 336 px),
			css::width!(672 px),
			css::height!(600 px),
			css::BackgroundImage::Some(vec![css::Image::Url("../public/img/lain/small.webp".to_string())]),
			css::BackgroundRepeat::NoRepeat,
			css::background_color!(colors::bg_black),
		))
		.tap(|&element| element.add_on_mouse_down(move |_| {
			element.set_class_tagged("Cursor", css::class!(css::Cursor::Grabbing));
			*element.get_cmp_mut::<Clicked>() = Clicked(true);
		}))
		.tap(|&element| element.add_on_mouse_up(move |_| {
			element.set_class_tagged("Cursor", css::class!(css::Cursor::Auto));
			element.get_cmp_mut::<Clicked>().0 = false;
		}))
		.tap(|&element| element.add_on_mouse_leave(move |_| {
			element.set_class_tagged("Cursor", css::class!(css::Cursor::Auto));
			element.get_cmp_mut::<Clicked>().0 = false;
		}))
		.tap(|&element| element.add_on_mouse_move(move |mouse_event| {
			if element.get_cmp::<Clicked>().0 {
				let (move_y, move_x) = (mouse_event.movement_y(), mouse_event.movement_x());
				let left = element.left() + f64::from(move_x);
				let top = element.top() + f64::from(move_y);

				element.set_style((css::top!(top), css::left!(left)));
			}
		}))
		.component(Clicked(false))
}


fn hello_header() -> e::Div {
	e::div()
		.class(css::style!(
			.& {
				css::color!(css::color::WHITE),
				"text-shadow: 2px 2px 1px black;",
			}

			raw("@keyframes writing") {
				"from{ opacity:0 } to { opacity: 1 }"
			}

			raw("@keyframes caret") {
				"0% { opacity:0 } 50% { opacity: 1 }"
			}
		))
		.child(e::div()
			.font(&text::space_mono::BIG)
			.class((css::Display::Flex, css::user_select!(none)))
			.child(fade_in_typewriter_animated_text("Hello."))
			.child(blinking_caret())
		)
}

#[allow(clippy::cast_precision_loss)]
pub fn fade_in_typewriter_animated_text(text: &str) -> e::Div {
	e::div()
		.children(text.chars().enumerate().map(|(i, c)| -> e::Span {
			e::span()
				.text(c.to_string())
				.class((
					css::opacity!(0),
					format!("animation: writing 0.1s linear forwards {}s;", i as f32 * 0.1)
				))
		}))
}

pub fn blinking_caret() -> e::Div {
	e::div()
		.child(e::span()
			.text("_")
			.class("animation: caret 1.33s steps(1, end) infinite;")
		)
}

fn homepage_nav() -> e::Ul {
	e::ul()
		.class(css::style!(
			.& {
				css::z_index!(2),
				css::list_style_position!(inside),
				css::line_height!(2.),
				css::width!(max-content),
				"list-style-type: \"– \";",
			}

			raw("@keyframes fade-in-left") {"
				0% {
					-webkit-transform: translateX(-100px);
				}

				100% {
		            -webkit-transform: translateX(0);
				}
			"}
		))
		.child(e::a().href("https://home.gremy.co.uk")
			.child(button("home").style(css::animation_delay!(100 ms)))
		)
		.child(e::a().href("https://cloud.gremy.co.uk")
			.child(button("cloud").style(css::animation_delay!(200 ms)))
		)
		.child(e::a().href("https://github.com/Gremious")
			.child(button("github").style(css::animation_delay!(300 ms)))
		)
		.child(button("item").style(css::animation_delay!(400 ms)))
		.child(button("item").style(css::animation_delay!(500 ms)))
}


fn button(text: &str) -> e::Li {
	e::li()
		.font(&text::space_mono::BODY)
		.class(css::style!(
			.& {
				css::user_select!(none),
				css::color!(css::color::WHITE),
				css::animation_name!("fade-in-left"),
				css::animation_duration!(500 ms),
				css::animation_timing_function!("cubic-bezier(.22,.61,.36,1)"),
				css::animation_fill_mode!(both),
				// You make a text shadow stronger... by setting it twice...
				css::TextShadow::Some(vec![
					css::TextShadowEffect {
						color: css::color::BLACK,
						offset_x: css::unit!(2 px),
						offset_y: css::unit!(2 px),
						blur_radius: css::unit!(1 px),
					},
					css::TextShadowEffect {
						color: css::color::BLACK,
						offset_x: css::unit!(2 px),
						offset_y: css::unit!(2 px),
						blur_radius: css::unit!(1 px),
					},
				]),
			}

			.&:hover {
				css::cursor!(pointer),
				css::color!(css::color::WHEAT),
			}

			.&:active {
				css::color!(css::color::WHITE),
			}
		))
		.child(e::span()
			.text(text)
			.class((
				css::text_decoration_line!(underline),
				"text-underline-offset: 0.25em;",
			))
		)
}

