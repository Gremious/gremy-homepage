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
            css::text_overflow!(ellipsis),
            css::white_space!(nowrap),
            css::overflow!(hidden),
        }
    });
}

pub fn style() -> css::Style {
    fonts()
        + css::style! {
            html, body {
                css::height!(100 vh),
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
                css::margin!(0),
                css::padding!(0),
                css::border_width!(0),
                css::border_style!(none),
                css::border_color!(0),
                css::outline_width!(0),
                css::outline_color!(0),

                css::color!(inherit),
                css::FontSize::Inherit,
                css::FontFamily::Inherit,
                css::FontWeight::Inherit,
                css::LineHeight::Inherit,
                css::TextDecorationLine::Inherit,
                css::WhiteSpace::Inherit,

                // css::Cursor::Inherit,
                css::UserSelect::Inherit,

                css::BoxSizing::Initial,
                css::AlignItems::Initial,
                css::TextAlign::Inherit,
                css::background_color!(initial),
            }
        }
}
