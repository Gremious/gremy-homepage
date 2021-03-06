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
        if let Tab::Debug = tab {
            return debug::classes_page()
                .with(|&element| spawn(async move {
                    web_sys::console::time_with_label("benchmark");

                    for i in 0..15 {
                        let ele = SomeElement(element.get_cmp::<Children>()[0]);
                        ele.replace_with(debug::new_element_classes());
                        log::debug!("{}", i);
                    }

                    web_sys::console::time_end_with_label("benchmark");
                    let perf = web_sys::window().expect("should have a window").performance().expect("should have performance").now();
                    log::debug!("perf: {:#?}", perf);
                }));
        }

        match tab {
            Tab::Homepage => homepage::new_element(),
            Tab::Debug => unreachable!(),
        }
        .class_typed::<Page>(css::style!(
            .& {
                css::size!(100%),
            }
        ))
    }
}
