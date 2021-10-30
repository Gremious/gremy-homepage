use super::*;

fn container_style() -> css::Style {
    css::style!(
        .& {
            css::Display::Flex,
            css::FlexDirection::Column,
            css::JustifyContent::FlexStart,
            css::BoxSizing::BorderBox,
            css::padding!(2.5%),
            css::background_color!(colors::bg_black),
            css::BackgroundImage::Some(vec![css::Image::Url("../public/img/lain/lain57_t.gif".to_string())]),
            "background-position: center;",
        }

       @media All && MaxWidth(css::unit!(500 px)) {
            .& {
                css::font_size!(12 vw),
                "background-size: cover;",
            }
        }
    )
}

fn element_style() -> css::Style {
    css::style!(
        .& {
            css::color!(css::color::WHITE),
            // css::AlignSelf::FlexEnd,
        }

        raw("@keyframes writing") {
            "from{ opacity:0 } to { opacity: 1 }"
        }

        raw("@keyframes caret") {
            "0% { opacity:0 } 50% { opacity: 1 }"
        }
    )
}

pub fn new_element() -> cmp::Div {
    cmp::div()
        .class(container_style())
        .child(cmp::div()
            .child(cmp::div()
                .class(css::Display::Flex)
                .child(fade_in_typewriter_animated_text("Hello.".to_owned()))
                .child(blinking_caret())
            )
            // .text("Hello.")
            .class(element_style())
            .class_typed::<text::FontTag>(text::space_mono::BIG.clone())
        )
}

pub fn fade_in_typewriter_animated_text(text: String) -> cmp::Div {
    cmp::div()
        .children(text.chars().enumerate().map(|(i, c)| {
            cmp::span()
                .text(c.to_string())
                .class((
                    css::opacity!(0),
                    format!("animation: writing 0.1s linear forwards {}s;", i as f32 * 0.1)
                ))
        }))
}

pub fn blinking_caret() -> cmp::Div {
    cmp::div()
        .child(cmp::span()
            .text("_")
            .class("animation: caret 1.33s steps(1, end) infinite;")
        )
}

#[allow(dead_code)]
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

