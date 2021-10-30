use super::*;

pub struct Homepage;
impl Homepage {
    pub fn new_element() -> cmp::Div {
        cmp::div()
            .class((
                css::Display::Flex,
                css::JustifyContent::Center,
                css::size!(100%),
                css::background_color!(css::Color { r: 00, g: 15, b: 20, a: 255 }),
            ))
            .child(cmp::div()
                .text("Hello. Ă U Ũ")
                .class(css::style!(
                    .& {
                        css::padding_top!(2.5%),
                        css::color!(css::color::WHITE),
                    }

                    @media All && MaxWidth(css::unit!(500 px)) {
                        .& {
                            css::font_size!(12 vw),
                        }
                    }
                ))
                .class_typed::<text::FontTag>(css::class!(text::space_mono::BODY.clone()))
            )
    }

    pub fn old_homepage() -> cmp::Div {
        cmp::div()
            .class((css::Display::Flex, css::JustifyContent::Center, css::size!(100%), css::background_color!(css::color::PINK)))
            .child(cmp::div()
                .text("Bonanza Extravaganza")
                .class(css::style!(
                    .& {
                        css::padding_top!(2.5%),
                        "-webkit-text-stroke: 3px hotpink;",
                        css::color!(css::color::WHITE),
                        text::alfa_slab_one::BODY.clone(),
                    }

                    @media All && MaxWidth(css::unit!(500 px)) {
                        .& {
                            css::font_size!(12 vw),
                        }
                    }
                ))
            )
            .child(cmp::a().attr(web_str::href(), "https://www.cat-bounce.com").class(css::size!(100%))
                .child(cmp::img().attr(web_str::src(), "../public/img/cat.gif").class(css::size!(100%)))
            )
    }
}
