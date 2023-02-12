use super::*;

pub mod homepage;
pub mod debug;

pub type TabState = Mutable<Tab>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SmartDefault)]
pub enum Tab {
	#[default]
    Homepage,
	// #[default]
    Debug,
}

pub struct Root;
impl Root {
    pub fn new_element() -> e::Div {
        e::div()
            .class((css::Display::Flex, css::min_height!(100 vh)))
            .child(e::div()
                .class((css::Display::Flex, css::FlexDirection::Column, css::width!(100%)))
                .child_signal(TabState::resource().signal().dedupe().map(move |tab| {
                    Page::new_element(tab)
                }))
            )
    }
}

pub struct Page;
impl Page {
    pub fn new_element(tab: Tab) -> e::Div {
        match tab {
            Tab::Homepage => homepage::new_element(),
            Tab::Debug => e::div(),
        }
        .class_typed::<Page>(css::style!(
            .& {
                css::size!(100%),
            }
        ))
    }
}
