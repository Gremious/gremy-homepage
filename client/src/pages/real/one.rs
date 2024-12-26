use crate::prelude::*;

pub fn new() -> e::Div {
	e::div()
		.class((
			css::border_style::solid,
			css::border_color::rgba(css::colors::RED),
			css::border_width::px(2),
		))
}
