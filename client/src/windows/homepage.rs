use super::*;

pub struct Homepage;
impl Homepage {
    pub fn new_element() -> cmp::Div {
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
    }
}
