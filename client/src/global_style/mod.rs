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

		dark => 0x16_16_16_FF;
		dark01 => 0x16_16_16_25;

		gray01 => 0x3A_3A_3A_FF;
		gray02 => 0x5D_5D_5D_FF;
		gray03 => 0xBD_BD_BD_FF;
		gray04 => 0x89_89_89_FF;
		gray05 => 0xE5_E5_E5_FF;
		mid_gray => 0x77_77_7A_FF;
		dark_gray => 0x2B_2A_30_FF;

		blue => 0x2B_19_FA_FF;
		blue_hover => 0x41_31_FE_FF;
		yellow => 0xFF_B8_26_FF;
		red => 0xE2_60_54_FF;
	}
}
//
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

	pub static OVERFLOW_ELLIPSIS: Lazy<Vec<css::Property>> = Lazy::new(|| css::properties! {
		css::text_overflow!(ellipsis),
		css::white_space!(nowrap),
		css::overflow!(hidden),
	});
}

pub fn style() -> css::Style { fonts() + css::style! {
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
} }
