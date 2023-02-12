use hobo::css::font_face::{Style, Weight, UnicodeRange, unicode_range};
use crate::prelude::*;

/// Makes font faces in a more compact manner
///
/// * `family` - Font family
/// * `local` - A font name for local search, e.g. "Font Regular"
/// * `style` - Font style
/// * `weight` - Font weight.
/// * `srcs` - Source urls
	// normally you can't have 1 @font-face decl with multiple url sources pointing to different unicode ranges
	// So this function basically makes it so u don't have to have
	// 5 seperate declarations that are identically outside of the source url and range
pub fn create_font(family: &str, local: &str, style: Style, weight: Weight, srcs: &[(&str, Vec<UnicodeRange>)]) -> css::Style {
	let mut ret = css::Style(Vec::new());

	for src in srcs {
		ret = ret + css::style!{
			@font-face {
				family: family.into(),
				style: style,
				weight: (weight, None),
				src: vec![Source::Local(family.into()), Source::Local(local.into()), Source::Url(src.0.into(), Some(Format::Woff2))],
				unicode_range: src.1.clone(),
				display: Display::Block,
			}
		};
	}

	ret
}

pub fn fonts() -> css::Style {
	create_font("Space Mono", "Space-Mono", Style::Normal, Weight::Number(400), &[
		("https://fonts.gstatic.com/s/spacemono/v12/i7dPIFZifjKcF5UAWdDRYE58RWq7.woff2",
		unicode_range![0x0102..0x0103, 0x0110..0x0111, 0x0128..0x0129, 0x0168..0x0169, 0x01A0..0x01A1, 0x01AF..0x01B0, 0x1EA0..0x1EF9, 0x20AB]),
		("https://fonts.gstatic.com/s/spacemono/v12/i7dPIFZifjKcF5UAWdDRYE98RWq7.woff2",
		unicode_range!(0x0100..0x024F, 0x0259, 0x1E00..0x1EFF, 0x2020, 0x20A0..0x20AB, 0x20AD..0x20CF, 0x2113, 0x2C60..0x2C7F, 0xA720..0xA7FF)),
		("https://fonts.gstatic.com/s/spacemono/v12/i7dPIFZifjKcF5UAWdDRYEF8RQ.woff2",
		unicode_range!(0x0000..0x00FF, 0x0131, 0x0152..0x0153, 0x02BB..0x02BC, 0x02C6, 0x02DA, 0x02DC, 0x2000..0x206F, 0x2074, 0x20AC, 0x2122, 0x2191, 0x2193, 0x2212, 0x2215, 0xFEFF, 0xFFFD)),
	])
	+ create_font("Space Mono", "Space-Mono", Style::Normal, Weight::Number(700), &[
		("https://fonts.gstatic.com/s/spacemono/v12/i7dMIFZifjKcF5UAWdDRaPpZUFqaHjyV.woff2",
		unicode_range!(0x0102..0x0103, 0x0110..0x0111, 0x0128..0x0129, 0x0168..0x0169, 0x01A0..0x01A1, 0x01AF..0x01B0, 0x1EA0..0x1EF9, 0x20AB)),
		("https://fonts.gstatic.com/s/spacemono/v12/i7dMIFZifjKcF5UAWdDRaPpZUFuaHjyV.woff2",
		unicode_range!(0x0100..0x024F, 0x0259, 0x1E00..0x1EFF, 0x2020, 0x20A0..0x20AB, 0x20AD..0x20CF, 0x2113, 0x2C60..0x2C7F, 0xA720..0xA7FF)),
		("https://fonts.gstatic.com/s/spacemono/v12/i7dMIFZifjKcF5UAWdDRaPpZUFWaHg.woff2",
		unicode_range!(0x0000..0x00FF, 0x0131, 0x0152..0x0153, 0x02BB..0x02BC, 0x02C6, 0x02DA, 0x02DC, 0x2000..0x206F, 0x2074, 0x20AC, 0x2122, 0x2191, 0x2193, 0x2212, 0x2215, 0xFEFF, 0xFFFD)),
	])
	+ create_font("Space Mono", "Space Mono", Style::Italic, Weight::Number(400), &[
		("https://fonts.gstatic.com/s/spacemono/v12/i7dNIFZifjKcF5UAWdDRYERMSHK_IwU.woff2",
		unicode_range!(0x0102..0x0103, 0x0110..0x0111, 0x0128..0x0129, 0x0168..0x0169, 0x01A0..0x01A1, 0x01AF..0x01B0, 0x1EA0..0x1EF9, 0x20AB)),
		("https://fonts.gstatic.com/s/spacemono/v12/i7dNIFZifjKcF5UAWdDRYERMSXK_IwU.woff2",
		unicode_range!(0x0100..0x024F, 0x0259, 0x1E00..0x1EFF, 0x2020, 0x20A0..0x20AB, 0x20AD..0x20CF, 0x2113, 0x2C60..0x2C7F, 0xA720..0xA7FF)),
		("https://fonts.gstatic.com/s/spacemono/v12/i7dNIFZifjKcF5UAWdDRYERMR3K_.woff2",
		unicode_range!(0x0000..0x00FF, 0x0131, 0x0152..0x0153, 0x02BB..0x02BC, 0x02C6, 0x02DA, 0x02DC, 0x2000..0x206F, 0x2074, 0x20AC, 0x2122, 0x2191, 0x2193, 0x2212, 0x2215, 0xFEFF, 0xFFFD)),
	])
	+ create_font("Space Mono", "Space Mono", Style::Italic, Weight::Number(700), &[
		("https://fonts.gstatic.com/s/spacemono/v12/i7dSIFZifjKcF5UAWdDRYERE_FeqEySRV3U.woff2",
		unicode_range!(0x0102..0x0103, 0x0110..0x0111, 0x0128..0x0129, 0x0168..0x0169, 0x01A0..0x01A1, 0x01AF..0x01B0, 0x1EA0..0x1EF9, 0x20AB)),
		("https://fonts.gstatic.com/s/spacemono/v12/i7dSIFZifjKcF5UAWdDRYERE_FeqEiSRV3U.woff2",
		unicode_range!(0x0100..0x024F, 0x0259, 0x1E00..0x1EFF, 0x2020, 0x20A0..0x20AB, 0x20AD..0x20CF, 0x2113, 0x2C60..0x2C7F, 0xA720..0xA7FF)),
		("https://fonts.gstatic.com/s/spacemono/v12/i7dSIFZifjKcF5UAWdDRYERE_FeqHCSR.woff2",
		unicode_range!(0x0000..0x00FF, 0x0131, 0x0152..0x0153, 0x02BB..0x02BC, 0x02C6, 0x02DA, 0x02DC, 0x2000..0x206F, 0x2074, 0x20AC, 0x2122, 0x2191, 0x2193, 0x2212, 0x2215, 0xFEFF, 0xFFFD)),
	])
	+ create_font("Alfa Slab One", "Alfa-Slab-One", Style::Normal, Weight::Number(400), &[
		("https://fonts.gstatic.com/s/alfaslabone/v17/6NUQ8FmMKwSEKjnm5-4v-4Jh2d1he-Wv.woff2",
		unicode_range!(0x0102..0x0103, 0x0110..0x0111, 0x0128..0x0129, 0x0168..0x0169, 0x01A0..0x01A1, 0x01AF..0x01B0, 0x1EA0..0x1EF9, 0x20AB)),
		("https://fonts.gstatic.com/s/alfaslabone/v17/6NUQ8FmMKwSEKjnm5-4v-4Jh2dxhe-Wv.woff2",
		unicode_range!(0x0100..0x024F, 0x0259, 0x1E00..0x1EFF, 0x2020, 0x20A0..0x20AB, 0x20AD..0x20CF, 0x2113, 0x2C60..0x2C7F, 0xA720..0xA7FF)),
		("https://fonts.gstatic.com/s/alfaslabone/v17/6NUQ8FmMKwSEKjnm5-4v-4Jh2dJhew.woff2",
		unicode_range!(0x0000..0x00FF, 0x0131, 0x0152..0x0153, 0x02BB..0x02BC, 0x02C6, 0x02DA, 0x02DC, 0x2000..0x206F, 0x2074, 0x20AC, 0x2122, 0x2191, 0x2193, 0x2212, 0x2215, 0xFEFF, 0xFFFD)),
	])
}
