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
	pub fn is_local(&self) -> bool {
		match &self {
			Tab::Homepage => true,
			Tab::Stream => false,
		}
	}

	#[must_use]
	pub fn to_url(self) -> String {
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

	#[must_use]
	pub fn from_url() -> Self {
		let url = window().location().href().ok().and_then(|x| url::Url::parse(&x).ok());
		let Some(segments) = url.as_ref()
			.and_then(url::Url::path_segments)
			.map(|iter| iter.map(|x| url::form_urlencoded::parse(x.as_bytes()).next().map(|x| x.0.into_owned()).unwrap_or(x.to_owned())).collect::<Vec<_>>())
			else { return Tab::default(); };
		let mut segments = segments.iter().map(|x| x as &str);

		match segments.next() {
			Some("stream") => Tab::Stream,
			_ => Tab::Homepage,
		}
	}
}

pub fn body() -> e::Div {
	let page = e::div();

	e::div()
		.class((css::Display::Flex, css::min_height!(100 vh)))
		.child(e::div()
			.class((css::Display::Flex, css::FlexDirection::Column, css::width!(100%)))
			.component(TabState::resource().signal().dedupe().subscribe(move |current_tab| {
				let new_url = current_tab.to_url();
				if window().location().href().unwrap_or_default() != new_url {
					window().location().assign(&new_url).ok();
				}

				if current_tab.is_local() {
					page.replace_with(Page::tab_page(current_tab));
				}
			}))
			.child(page)
		)
}

pub struct Page;
impl Page {
    pub fn tab_page(tab: Tab) -> Element {
        match tab {
            Tab::Homepage => homepage::new().as_element(),
            Tab::Stream => unreachable!(),
            // Tab::Debug => e::div(),
        }
        .class_typed::<Page>(css::style!(
            .& {
                css::size!(100%),
            }
        ))
    }
}
