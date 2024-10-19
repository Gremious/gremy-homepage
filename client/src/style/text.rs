use super::*;

pub struct FontTag;

pub mod alfa_slab_one {
    use super::*;

    pub static BODY: Lazy<css::Style> = Lazy::new(|| {
        css::style!(
			.& {
				css::font_family::Some(vec![String::from("Alfa Slab One")]),
				css::font_weight::val(400),
			}
        )
    });
}

pub mod space_mono {
    use super::*;

    pub static BODY: Lazy<css::Style> = Lazy::new(|| {
		css::style!(
			.& {
				css::font_family::Some(vec![String::from("Space Mono")]),
				css::font_weight::val(400),
			}
		)
    });

    pub static BIG: Lazy<css::Style> = Lazy::new(|| {
        css::style!(
            .& {
                css::font_family::Some(vec![String::from("Space Mono")]),
                css::font_weight::val(400),
                css::font_size::px(64),
            }

            // @media All && MaxWidth(css::unit!(500 px)) {
            //     .& {
            //         css::font_size!(12 vw),
            //     }
            // }
        )
    });

}
