// You could just .href("/wherever")
// however, that forces an actual page load,
// which is slower than just swapping out an element (page) for a differet one
//
// And also, you get flashbanged on the first load of every page, which is not nice.
//
// "with our SPA-style approach you shouldn't actually be hrefing across your site - only pushing location to history stack to pretend like you are
// if you want to href around without getting flashbanged - we need SSR" - @awpteamoose
//
// (though	also
// "if you have some body style you can just .to_string() and slam it into a <style> tag in your reply, or as a build step save it in public and link to it like a plain css file"
// so then you can give it background-color: black; every reply, which could work to prevent flashbang)
use super::*;

pub mod homepage;
pub mod real;
pub mod stream;
pub mod debug;

pub struct Clicked(bool);

#[derive(Default, Clone, Copy, Debug)]
pub struct Navigation {
	pub cur: Tab,
	pub prev: Option<Tab>,
	pub popped: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SmartDefault)]
pub enum Tab {
	#[default]
	Homepage,
	Stream,
	Real,
	Debug,
}

impl Tab {
	#[must_use]
	pub fn to_url(self) -> String {
		let base = if cfg!(debug_assertions) {
			format!("https://localhost:{}", shared::CONFIG.port)
		} else {
			format!("https://{}", shared::CONFIG.hostname)
		};
		let Ok(mut base_url) = url::Url::parse(&base) else { return base; };
		let Ok(mut segments) = base_url.path_segments_mut() else { return base; };

		match self {
			Tab::Homepage => {},
			Tab::Stream => { segments.push("stream"); },
			Tab::Real => { segments.push("real"); },
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
			Some("real") => Tab::Real,
			Some("debug") => Tab::Debug,
			_ => Tab::Homepage,
		}
	}
}

pub fn navigate(tab: impl Into<Tab>) {
	fn navigate_(mut tab: Tab) {
		let tab_state = Mutable::<Navigation>::resource();
		let &mut pages::Navigation { ref mut cur, ref mut prev, .. } = &mut tab_state.lock_mut() as &mut _;
		if *cur == tab { return; }

		std::mem::swap(cur, &mut tab);
		*prev = Some(tab);
	}

	navigate_(tab.into());
}

pub fn body() -> e::Div {
	e::div()
		.class(css::display::flex)
		.child_signal(Mutable::<Navigation>::resource().signal().filter_map(|Navigation { cur, prev, popped }| {
			// only push state if not going back/forward in history and if not same url
			if popped {
				Mutable::<Navigation>::resource().lock_mut().popped = false;
			} else {
				let new_url = cur.to_url();
				if window().location().href().unwrap_or_default() != new_url {
					window().history().ok().and_then(|x| x.push_state_with_url(&JsValue::NULL, "", Some(&new_url)).ok());
				}
			}

			// If we somehow try to navigate from the same page onto itself, do nothing
			match cur {
				Tab::Homepage => if matches!(prev, Some(Tab::Homepage)) { None } else {
					Some(Page::tab_page(Tab::Homepage))
				},
				Tab::Stream => if matches!(prev, Some(Tab::Stream)) { None } else {
					Some(Page::tab_page(Tab::Stream))
				},
				Tab::Real => if matches!(prev, Some(Tab::Real)) { None } else {
					Some(Page::tab_page(Tab::Real))
				},
				Tab::Debug => if matches!(prev, Some(Tab::Debug)) { None } else {
					Some(Page::tab_page(Tab::Debug))
				}
			}
		}).map(Option::unwrap))
}

pub struct Page;
impl Page {
	pub fn tab_page(tab: Tab) -> Element {
		match tab {
			Tab::Homepage => homepage::new().as_element(),
			Tab::Debug => debug::new().as_element(),
			Tab::Stream => stream::new().as_element(),
			Tab::Real => real::new().as_element(),
		}
		.class_typed::<Page>(css::style!(
			.& {
				css::color::rgba(css::colors::WHITE),
			}
		))
	}
}
