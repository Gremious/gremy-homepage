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

macro_rules! fonts {
    ($($name:ident => $size:literal);+$(;)?) => {paste::item!{
        $(
            pub struct [<Tag $name>];

            pub fn [<$name:lower _properties>]() -> Vec<css::Property> {
                css::properties!(
                    css::font_family!("Space Mono"),
                    css::font_weight!(400),
                    css::font_size!($size px),
                    css::color!(css::color::AQUA.a(255 - $size * 3)),
                )
            }

            pub static [<$name:upper>]: Lazy<css::Style> = Lazy::new(|| {
                css::style!(
                    .& {
                        css::font_family!("Space Mono"),
                        css::font_weight!(400),
                        css::font_size!($size px),
                        css::color!(css::color::AQUA.a(255 - $size * 3)),
                    }
                )
            });
        )+
    }};
}

pub mod text_test {
    use super::*;

    fonts!{
        A => 1;
        B => 2;
        C => 3;
        D => 4;
        E => 5;
        F => 6;
        G => 6;
        H => 7;
        I => 8;
        J => 9;
        K => 10;
        L => 11;
        M => 12;
        N => 13;
        O => 14;
        P => 15;
        Q => 16;
        R => 17;
        S => 18;
        T => 19;
        U => 21;
        V => 22;
        W => 23;
        X => 24;
        Y => 25;
        Z => 26;
    }
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

            // @media All && MaxWidth(css::unit!(500 px)) {
            //     .& {
            //         css::font_size!(12 vw),
            //     }
            // }
        )
    });

}
