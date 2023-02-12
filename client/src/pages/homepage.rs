use super::*;

struct Clicked(bool);

pub fn new() -> e::Div {
	container()
		.child(hello_header())
		.child(lain_image())
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
				css::padding!(2.5%),
				css::background_color!(colors::bg_black),
			}

			@media All && MaxWidth(css::unit!(500 px)) {
				.& { "background-size: cover;" }
			}
		))
}

fn hello_header() -> e::Div {
	e::div()
		.class(css::style!(
			.& {
				css::color!(css::color::WHITE),
				css::z_index!(100),
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
			.class(css::Display::Flex)
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
				let top = element.top() + f64::from(move_y);
				let left = element.left() + f64::from(move_x);

				element.set_style((css::top!(top), css::left!(left)));
			}
		}))
		.component(Clicked(false))
}

