// TODO: Separate, maybe more global mobile style, potentially based on just global tags, or on styles like the font?

use super::*;

struct Clicked(bool);

fn container_style() -> css::Style {
    css::style!(
        .& {
            css::Position::Relative,
            css::Display::Flex,
            css::FlexDirection::Column,
            css::JustifyContent::FlexStart,
            css::BoxSizing::BorderBox,
            css::padding!(2.5%),
            css::background_color!(colors::bg_black),
        }

       @media All && MaxWidth(css::unit!(500 px)) {
            .& {
                "background-size: cover;",
            }
        }

        raw("@keyframes writing") {
            "from{ opacity:0 } to { opacity: 1 }"
        }

        raw("@keyframes caret") {
            "0% { opacity:0 } 50% { opacity: 1 }"
        }
    )
}

fn text_style() -> css::Style {
    css::class!((
        css::color!(css::color::WHITE),
        css::z_index!(100),
    ))
}

fn image_style() -> css::Style {
    css::class!((
        css::Position::Absolute,
        css::top!(50% - 300 px),
        css::left!(50% - 336 px),
        css::width!(672 px),
        css::height!(600 px),
        css::BackgroundImage::Some(vec![css::Image::Url("../public/img/lain/small.webp".to_string())]),
        css::BackgroundRepeat::NoRepeat,
        css::background_color!(colors::bg_black),
    ))
}

pub fn new_element() -> cmp::Div {
    cmp::div()
        .class(container_style())
        .child(cmp::div()
            .class(text_style())
            // .mark::<text::space_mono::TagBig>()
            .class_typed::<text::FontTag>(text::space_mono::BIG.clone())
            .child(cmp::div()
                .class(css::Display::Flex)
                .child(fade_in_typewriter_animated_text("Hello.".to_owned()))
                .child(blinking_caret())
            )
        )
        .child(cmp::div()
            .class(image_style())
            .with(|&element| element.add_on_mouse_down(move |mouse_event| {
                element.set_class_tagged("Cursor", css::class!(css::Cursor::Grabbing));
                *element.get_cmp_mut::<Clicked>() = Clicked(true);
            }))
            .with(|&element| element.add_on_mouse_up(move |_| {
                element.set_class_tagged("Cursor", css::class!(css::Cursor::Auto));
                element.get_cmp_mut::<Clicked>().0 = false;
            }))
            .with(|&element| element.add_on_mouse_leave(move |_| {
                element.set_class_tagged("Cursor", css::class!(css::Cursor::Auto));
                element.get_cmp_mut::<Clicked>().0 = false;
            }))
            .with(|&element| {
                element.add_on_mouse_move(move |mouse_event| {
                    if element.get_cmp::<Clicked>().0 {
                        let image_rect = element.get_cmp::<web_sys::HtmlElement>().get_bounding_client_rect();
                        let (image_top, image_left) = (image_rect.y(), image_rect.x());
                        let (move_y, move_x) = (mouse_event.movement_y(), mouse_event.movement_x());

                        let top = image_top + move_y as f64;
                        let left = image_left + move_x as f64;

                        element.set_style((
                            css::top!(top),
                            css::left!(left),
                        ));
                    }
                });
            })
            .component(Clicked(false))
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

//=

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
