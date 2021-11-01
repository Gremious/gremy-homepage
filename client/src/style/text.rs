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

    pub struct TagBody;
    pub static BODY: Lazy<Vec<css::Property>> = Lazy::new(|| {
        css::properties! {
            css::font_family!("Space Mono"),
            css::font_weight!(400),
        }
    });

    pub struct TagBig;
    impl TagBig {
        pub fn properties() -> Vec<css::Property> {
            css::properties!(
                css::font_family!("Space Mono"),
                css::font_weight!(400),
                css::font_size!(64 px),
            )
        }
    }

    pub static BIG: Lazy<css::Style> = Lazy::new(|| {
        css::style!(
            .& {
                css::font_family!("Space Mono"),
                css::font_weight!(400),
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
