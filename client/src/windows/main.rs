use crate::prelude::*;

pub static TAB: Lazy<state::State<Tab>> = Lazy::new(default);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SmartDefault)]
pub enum Tab {
    #[default]
    Main,
}

pub struct App;
impl App {
    pub fn new_element() -> SomeElement {
        let element = cmp::div().class((css::Display::Flex, css::min_height!(100 vh))).erase();

        let main_container = cmp::div().class((css::Display::Flex, css::FlexDirection::Column, css::width!(100%))).with(|&element| {
            element.add_component(TAB.subscribe(move |_| {
                let child = SomeElement(hobo::Children::descendants(element)[1]);
                child.replace_with(Page::new_element().erase());
            }));
        });

        element.child(main_container.child(Page::new_element()))
    }
}

pub struct Page;
impl Page {
    pub fn new_element() -> cmp::Div {
        match *TAB.view() {
            Tab::Main => cmp::div()
                .class((css::Display::Flex, css::JustifyContent::Center, css::size!(100%), css::background_color!(css::color::PINK)))
                .child(cmp::div().text("Bonanza Extravaganza").class(css::style!(
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
                ))),
        }
    }
}
