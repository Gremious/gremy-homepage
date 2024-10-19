mod fonts;
#[allow(non_upper_case_globals, dead_code)]
pub mod colors;
#[allow(dead_code)]
pub mod text;

use crate::prelude::*;
use fonts::fonts;

#[allow(dead_code)]
pub mod element {
    use super::*;

    pub static OVERFLOW_ELLIPSIS: Lazy<Vec<css::Property>> = Lazy::new(|| {
        css::properties! {
            css::text_overflow::ellipsis,
            css::white_space::nowrap,
            css::overflow::hidden,
        }
    });
}

pub fn style() -> css::Style {
    fonts()
        + css::style! {
            html, body {
                css::height::vh(100),
            }

            body {
                // css::color!(0),
                // css::font_size!(100%),
                // css::font_family!("serif"),
                // css::FontWeight::Normal,
                // css::line_height!(1),
                // css::TextDecorationLine::None,
                // css::Cursor::Auto,
            }

			* {
				css::margin::zero,
				css::padding::zero,
				css::border_width::zero,
				css::border_style::none,
				css::border_color::none,
				css::outline_width::zero,
				css::outline_color::none,

				css::color::inherit,
				css::font_size::inherit,
				css::font_family::inherit,
				css::font_weight::inherit,
				css::font_style::inherit,
				css::line_height::inherit,
				css::text_decoration_line::inherit,
				css::white_space::inherit,

				// css::cursor::inherit,
				css::user_select::inherit,

				css::box_sizing::border_box,
				css::align_items::initial,
				css::background_color::initial,
			}
        }
}
