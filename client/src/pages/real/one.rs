use crate::prelude::*;

pub fn new() -> e::Div {
	e::div()
		.class((
			css::aspect_ratio::val(1.77),
			css::border_style::solid,
			css::border_color::rgba(css::colors::RED),
			css::border_width::px(2),
		))
}
