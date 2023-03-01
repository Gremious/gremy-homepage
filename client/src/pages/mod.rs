use super::*;

pub mod homepage;
pub mod debug;

pub type TabState = Mutable<Tab>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SmartDefault)]
pub enum Tab {
	#[default]
    Homepage,
	Stream,
	// #[default]
    // Debug,
}

impl Tab {
	#[must_use]
	pub fn to_url(&self) -> String {
		let base = format!("https://{}", shared::CONFIG.hostname);
		let Ok(mut base_url) = url::Url::parse(&base) else { return base; };
		let Ok(mut segments) = base_url.path_segments_mut() else { return base; };

		match self {
			Tab::Homepage => {},
			Tab::Stream => { segments.push("stream"); },
		};

		drop(segments);
		base_url.to_string()
	}
}

pub fn body() -> e::Div {
	e::div()
		.class((css::Display::Flex, css::min_height!(100 vh)))
		.child(e::div()
			.class((css::Display::Flex, css::FlexDirection::Column, css::width!(100%)))
			.child_signal(TabState::resource().signal().dedupe().map(|current_tab| {
				let new_url = current_tab.to_url();
				if window().location().href().unwrap_or_default() != new_url {
					window().history().ok().and_then(|x| x.push_state_with_url(&JsValue::NULL, "", Some(&new_url)).ok());
				}

				Page::tab_page(current_tab)
			}))
		)
}

pub struct Page;
impl Page {
    pub fn tab_page(tab: Tab) -> Element {
        match tab {
            Tab::Homepage => homepage::new().as_element(),
            Tab::Stream => e::div().as_element(),
            // Tab::Debug => e::div(),
        }
        .class_typed::<Page>(css::style!(
            .& {
                css::size!(100%),
            }
        ))
    }
}
