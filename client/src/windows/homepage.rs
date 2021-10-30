use super::*;

fn container_style() -> css::Style {
    css::class!((
        css::Display::Flex,
        css::FlexDirection::Column,
        css::JustifyContent::FlexStart,
        css::BoxSizing::BorderBox,
        css::padding!(2.5%),
        css::background_color!(colors::bg_black),
        css::BackgroundImage::Some(vec![css::Image::Url("../public/img/lain/lain57_t.gif".to_string())]),
        "background-position: center;",
    ))
}

fn element_style() -> css::Style {
    css::style!(
        .& {
            css::color!(css::color::WHITE),
            // css::AlignSelf::FlexEnd,
        }

        @media All && MaxWidth(css::unit!(500 px)) {
            .& {
                css::font_size!(12 vw),
            }
        }
    )
}

pub fn new_element() -> cmp::Div {
    cmp::div()
        .class(container_style())
        .child(cmp::div()
            .text("Hello.")
            .class(element_style())
            .class_typed::<text::FontTag>(text::space_mono::BIG.clone())
        )
        // .child(cmp::img().attr(web_str::src(), "../public/img/lain/lain57_t.gif"))
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

