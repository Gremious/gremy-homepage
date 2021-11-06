use super::*;

pub fn marked_page() -> cmp::Div {
    use text::text_test::*;

    let mut test_font_style = css::Style(Vec::new());

    macro_rules! fonts {
            ($($name:ident),*) => {paste::item!{
                $(
                   test_font_style = test_font_style + css::style!(
                        .& >> .[[<Tag $name>]] {
                            text::text_test::[<$name:lower _properties>]()
                        }
                    );
                )+
            }}
    }

    fonts!{ A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z }
    cmp::div()
        .class_typed::<Page>(css::class!(css::size!(100%)) + test_font_style)
        .child(debug::new_element_marked())
}

pub fn classes_page() -> cmp::Div {
    cmp::div()
        .class_typed::<Page>(css::class!(css::size!(100%)))
        .child(debug::new_element_classes())
}

pub fn new_element_marked() -> cmp::Div {
    let mut children = Vec::new();

    macro_rules! fonts {
        ($($name:ident),*) => {paste::item!{
            $(
                children.push(cmp::div()
                    .class((css::Display::Flex, css::FlexDirection::Column))
                    .children(
                        (0..100).into_iter().map(|i| {
                            cmp::div()
                            .text(i.to_string())
                            .mark::<text::text_test::[<Tag $name>]>()
                            // .class_typed::<text::FontTag>(text::text_test::[<$name:upper>].clone())
                        })
                    ));
            )+
        }};
    }

    fonts!{ A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z }

    cmp::div()
        .class((css::background_color!(colors::bg_black), css::Display::Flex, css::column_gap!(15 px), css::color!(css::color::WHITE)))
        .children(children)
}

pub fn new_element_classes() -> cmp::Div {
    let mut children = Vec::new();

    macro_rules! fonts {
        ($($name:ident),*) => {paste::item!{
            $(
                children.push(cmp::div()
                    .class((css::Display::Flex, css::FlexDirection::Column))
                    .children(
                        (0..100).into_iter().map(|i| {
                            cmp::div()
                            .text(i.to_string())
                            // .mark::<text::text_test::[<Tag $name>]>()
                            .class_typed::<text::FontTag>(text::text_test::[<$name:upper>].clone())
                        })
                    ));
            )+
        }};
    }

    fonts!{ A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z }

    cmp::div()
        .class((css::background_color!(colors::bg_black), css::Display::Flex, css::column_gap!(15 px), css::color!(css::color::WHITE)))
        .children(children)
}
