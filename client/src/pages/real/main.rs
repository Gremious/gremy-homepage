use crate::prelude::*;

pub fn new() -> e::Div {
	e::div()
		.child(e::div()
			.class((
				css::size::px(100.),
				css::background_color::rgba(css::colors::LIGHTPINK),
				css::cursor::auto,
			))
			// Stop if you need the mouse to not move
			// .on_mouse_down(|e| e.stop_propagation())
		)

}
