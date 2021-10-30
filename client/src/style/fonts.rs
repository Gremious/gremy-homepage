use hobo::css::font_face::{Style, Weight};
use crate::prelude::*;

/// Makes font faces
///
/// * `family` - Font family
/// * `local` - A font name for local search, e.g. "Font Regular"
/// * `style` - Font style
/// * `weight` - Font weight.
/// * `srcs` - Source urls
///
pub fn font(family: &str, local: &str, style: Style, weight: Weight, srcs: &[&str]) -> css::Style {
	let mut ret = css::Style(Vec::new());

	for &src in srcs {
		ret = ret + css::style!{
			@font-face {
				family: family.into(),
				style: style,
				weight: (weight, None),
				src: vec![Source::Local(family.into()), Source::Local(local.into()), Source::Url(src.into(), Some(Format::Woff2))],
				display: Display::Block,
			}
		};
	}

	ret
}

pub fn fonts() -> css::Style {
	font("Alfa Slab One", "Alfa-Slab-One", Style::Normal, Weight::Normal, &["https://fonts.gstatic.com/s/alfaslabone/v10/6NUQ8FmMKwSEKjnm5-4v-4Jh2dJhew.woff2"])
	+ font("Degrassi", "Degrassi Regular", Style::Normal, Weight::Normal, &["../public/fonts/degrassi/degrassi.ttf"])
	+ font("Space Mono", "Space Mono Regular", Style::Normal, Weight::Normal, &[
			"https://fonts.gstatic.com/s/spacemono/v6/i7dPIFZifjKcF5UAWdDRYE58RWq7.woff2",
			"https://fonts.gstatic.com/s/spacemono/v6/i7dPIFZifjKcF5UAWdDRYE98RWq7.woff2",
			"https://fonts.gstatic.com/s/spacemono/v6/i7dPIFZifjKcF5UAWdDRYEF8RQ.woff2",
		]
	)
	+ font("Space Mono", "Space Mono Bold", Style::Normal, Weight::Bold, &[
			"https://fonts.gstatic.com/s/spacemono/v6/i7dMIFZifjKcF5UAWdDRaPpZUFqaHjyV.woff2",
			"https://fonts.gstatic.com/s/spacemono/v6/i7dMIFZifjKcF5UAWdDRaPpZUFuaHjyV.woff2",
			"https://fonts.gstatic.com/s/spacemono/v6/i7dMIFZifjKcF5UAWdDRaPpZUFWaHg.woff2",
		]
	)
}
