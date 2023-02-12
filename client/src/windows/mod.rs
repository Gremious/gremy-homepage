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
            Tab::Debug => cmp::div(),
        }
        .class_typed::<Page>(css::style!(
            .& {
                css::size!(100%),
            }
        ))
    }
}
