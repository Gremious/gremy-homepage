use shared::shared_prelude::*;
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
}

fn container() -> e::Div {
	e::div()
		.class(css::style!(
			.& {
				css::position::relative,
				css::display::flex,
				css::flex_direction::column,
				css::justify_content::flex_start,
				css::box_sizing::border_box,
				css::overflow::hidden,
				css::padding::pct(2.5),
			}

			@media All && MaxWidth(css::unit!(500 px)) {
				.& { "background-size: cover;" }
			}
		))
}

fn lain_image() -> e::Div {
	e::div()
		.class((
			css::position::absolute,
			css::top::unit(css::Unit::pct(50) - css::Unit::px(300)),
			css::left::unit(css::Unit::pct(50) - css::Unit::px(336)),
			css::width::px(672),
			css::height::px(600),
			css::background_image::Some(vec![css::Image::Url("../public/img/lain/small.webp".to_string())]),
			css::background_repeat::no_repeat,
		))
		.tap(|&element| element.add_on_mouse_down(move |_| {
			element.set_class_tagged("Cursor", css::class!(css::cursor::grabbing));
			*element.get_cmp_mut::<Clicked>() = Clicked(true);
		}))
		.tap(|&element| element.add_on_mouse_up(move |_| {
			element.set_class_tagged("Cursor", css::class!(css::cursor::auto));
			element.get_cmp_mut::<Clicked>().0 = false;
		}))
		.tap(|&element| element.add_on_mouse_leave(move |_| {
			element.set_class_tagged("Cursor", css::class!(css::cursor::auto));
			element.get_cmp_mut::<Clicked>().0 = false;
		}))
		.tap(|&element| element.add_on_mouse_move(move |mouse_event| {
			if element.get_cmp::<Clicked>().0 {
				let (move_y, move_x) = (mouse_event.movement_y(), mouse_event.movement_x());
				let left = element.left() + f64::from(move_x);
				let top = element.top() + f64::from(move_y);

				element.set_style((css::top::px(top), css::left::px(left)));
			}
		}))
		.component(Clicked(false))
}


fn hello_header() -> e::Div {
	e::div()
		.class(css::style!(
			.& {
				css::color::rgba(css::colors::WHITE),
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
			.class((css::display::flex, css::user_select::none))
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
					css::opacity::val(0),
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
				css::z_index::val(2),
				css::list_style_position::inside,
				css::line_height::val(2.),
				css::width::max_content,
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
			.child(button("home").style(css::animation_delay::dur(100)))
		)
		.child(e::a().href("https://data.gremy.co.uk")
			.child(button("data").style(css::animation_delay::dur(200)))
		)
		.child(e::a().href("/stream")
			.child(button("stream").style(css::animation_delay::dur(200)))
		)
		.child(e::a().href("https://github.com/Gremious")
			.child(button("github").style(css::animation_delay::dur(300)))
		)
}


fn button(text: &str) -> e::Li {
	e::li()
		.font(&text::space_mono::BODY)
		.class(css::style!(
			.& {
				css::user_select::none,
				css::color::rgba(css::colors::WHITE),
				css::animation_name::String(String::from("fade-in-left")),
				css::animation_duration::dur(500),
				css::animation_timing_function::raw("cubic-bezier(.22,.61,.36,1)"),
				css::animation_fill_mode::both,
				// You make a text shadow stronger... by setting it twice...
				css::text_shadow::Some(vec![
					css::TextShadowEffect {
						color: css::colors::BLACK,
						offset_x: css::unit!(2 px),
						offset_y: css::unit!(2 px),
						blur_radius: css::unit!(1 px),
					},
					css::TextShadowEffect {
						color: css::colors::BLACK,
						offset_x: css::unit!(2 px),
						offset_y: css::unit!(2 px),
						blur_radius: css::unit!(1 px),
					},
				]),
			}

			.&:hover {
				css::cursor::pointer,
				css::color::rgba(css::colors::WHEAT),
			}

			.&:active {
				css::color::rgba(css::colors::WHITE),
			}
		))
		.child(e::span()
			.text(text)
			.class((
				css::text_decoration_line::underline,
				"text-underline-offset: 0.25em;",
			))
		)
}
