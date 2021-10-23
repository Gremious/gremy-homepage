mod fonts;

use crate::prelude::*;
use fonts::fonts;

#[allow(non_upper_case_globals, dead_code)]
pub mod colors {
    pub use super::*;

    macro_rules! def_colors {
        ($($name:ident => $color:expr);+$(;)?) => {$(pub static $name: css::Color = css::Color::from_hex($color);)+};
    }

    def_colors! {
        debug_blue => 0x00_87_FF_47;
    }
}

#[allow(dead_code)]
pub mod text {
    use super::*;

    pub mod alfa_slab_one {
        use super::*;

        pub static BODY: Lazy<Vec<css::Property>> = Lazy::new(|| {
            css::properties! {
                css::font_family!("Alfa Slab One"),
                css::font_size!(64 px),
                // css::line_height!(1.369 px),
                css::font_weight!(400),
            }
        });
    }
}

// #[allow(dead_code)]
// pub mod text {
// 	use super::*;
//
// 	pub mod rubik {
// 		use super::*;
//
// 		pub static H1: Lazy<Vec<css::Property>> = Lazy::new(|| css::properties! {
// 			css::font_family!("Rubik"),
// 			css::font_size!(30 px),
// 			css::line_height!(35.55 px),
// 			css::font_weight!(500),
// 		});
//
// 		pub static H2: Lazy<Vec<css::Property>> = Lazy::new(|| css::properties! {
// 			css::font_family!("Rubik"),
// 			css::font_size!(22 px),
// 			css::line_height!(26.07 px),
// 			css::font_weight!(500),
// 		});
//
// 		pub static BODY: Lazy<Vec<css::Property>> = Lazy::new(|| css::properties! {
// 			css::font_family!("Rubik"),
// 			css::font_size!(15 px),
// 			css::line_height!(17.78 px),
// 			css::font_weight!(500),
// 		});
// 	}
//
//
// 	pub mod roboto {
// 		use super::*;
//
// 		pub static TEXT: Lazy<Vec<css::Property>> = Lazy::new(|| css::properties! {
// 			css::font_family!("Roboto"),
// 			css::font_size!(18 px),
// 			css::line_height!(23.04 px),
// 			css::font_weight!(400),
// 		});
//
// 		pub static SMALL_TEXT: Lazy<Vec<css::Property>> = Lazy::new(|| css::properties! {
// 			css::font_family!("Roboto"),
// 			css::font_size!(15 px),
// 			css::line_height!(20.55 px),
// 			css::font_weight!(400),
// 		});
//
// 		pub static TINY_TEXT: Lazy<Vec<css::Property>> = Lazy::new(|| css::properties! {
// 			css::font_family!("Roboto"),
// 			css::font_size!(13 px),
// 			css::line_height!(17.81 px),
// 			css::font_weight!(400),
// 		});
// 	}
//
// 	pub mod lato {
// 		use super::*;
//
// 		pub static CTA: Lazy<Vec<css::Property>> = Lazy::new(|| css::properties! {
// 			css::font_family!("Lato"),
// 			css::font_size!(20 px),
// 			css::line_height!(24 px),
// 			css::font_weight!(400),
// 		});
//
// 		pub static CTA_SMALL: Lazy<Vec<css::Property>> = Lazy::new(|| css::properties! {
// 			css::font_family!("Lato"),
// 			css::font_size!(14 px),
// 			css::line_height!(16.8 px),
// 			css::font_weight!(400),
// 		});
// 	}
//
// }

#[allow(dead_code)]
pub mod element_style {
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
