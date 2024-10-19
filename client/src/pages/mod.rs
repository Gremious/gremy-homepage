use super::*;

pub mod homepage;
pub mod stream;
pub mod debug;

pub type TabState = Mutable<Tab>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SmartDefault)]
pub enum Tab {
	#[default]
	Homepage,
	Stream,
	Debug,
}

impl Tab {
	#[must_use]
	pub fn to_url(self) -> String {
		let base = format!("https://{}", shared::CONFIG.hostname);
		let Ok(mut base_url) = url::Url::parse(&base) else { return base; };
		let Ok(mut segments) = base_url.path_segments_mut() else { return base; };

		match self {
			Tab::Homepage => {},
			Tab::Stream => { segments.push("stream"); },
			Tab::Debug => { segments.push("debug"); },
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
			Some("debug") => Tab::Debug,
			_ => Tab::Homepage,
		}
	}
}

pub fn body() -> e::Div {
	e::div()
		.class((css::display::flex, css::min_height::vh(100)))
		.child(e::div()
			.class((css::display::flex, css::flex_direction::column, css::width::pct(100.)))
			.child_signal(TabState::resource().signal().map(|current_tab| {
				Page::tab_page(current_tab)
			}))
		)
}

pub struct Page;
impl Page {
    pub fn tab_page(tab: Tab) -> Element {
        match tab {
            Tab::Homepage => homepage::new().as_element(),
			Tab::Debug => debug::new().as_element(),
			Tab::Stream => stream::new().as_element(),
        }
        .class_typed::<Page>(css::style!(
            .& {
                css::size::pct(100),
            }
        ))
    }
}
