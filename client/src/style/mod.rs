mod fonts;
pub mod colors;

use crate::prelude::*;
use fonts::fonts;

#[allow(dead_code)]
pub mod text {
    use super::*;
    pub struct FontTag;

    pub mod alfa_slab_one {
        use super::*;

        pub static BODY: Lazy<Vec<css::Property>> = Lazy::new(|| {
            css::properties! {
                css::font_family!("Alfa Slab One"),
                css::font_weight!(400),
            }
        });
    }

    pub mod degrassi {
        use super::*;

        pub static BODY: Lazy<Vec<css::Property>> = Lazy::new(|| {
            css::properties! {
                css::font_family!("Degrassi"),
                css::font_weight!(400),
            }
        });
    }

    pub mod space_mono {
        use super::*;

        pub static BODY: Lazy<Vec<css::Property>> = Lazy::new(|| {
            css::properties! {
                css::font_family!("Space Mono"),
                css::font_weight!(normal),
            }
        });

        pub static BIG: Lazy<css::Style> = Lazy::new(|| {
            css::style!(
                .& {
                    css::font_family!("Space Mono"),
                    css::font_weight!(normal),
                    css::font_size!(64 px),
                }

                @media All && MaxWidth(css::unit!(500 px)) {
                    .& {
                        css::font_size!(12 vw),
                    }
                }
            )
        });
    }
}

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
