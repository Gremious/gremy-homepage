use super::*;

pub mod homepage;

pub type TabState = Mutable<Tab>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SmartDefault)]
pub enum Tab {
    #[default]
    Homepage,
}

pub struct Root;
impl Root {
    pub fn new_element() -> cmp::Div {
        cmp::div()
            .class((css::Display::Flex, css::min_height!(100 vh)))
            .child(cmp::div()
                .class((css::Display::Flex, css::FlexDirection::Column, css::width!(100%)))
                .child_signal(TabState::resource().signal().dedupe().map(move |tab| {
                    Page::new_element(tab)
                }))
            )
    }
}

pub struct Page;
impl Page {
    pub fn new_element(tab: Tab) -> cmp::Div {
        match tab {
            Tab::Homepage => homepage::new_element(),
        }
        .class_typed::<Page>(css::class!(css::size!(100%)))
    }
}
