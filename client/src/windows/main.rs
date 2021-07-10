use crate::prelude::*;

pub static TAB: Lazy<state::State<Tab>> = Lazy::new(default);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SmartDefault)]
pub enum Tab {
	#[default]
	Main,
}

pub struct App;
impl App {
	pub fn new() -> SomeElement {
		let element = cmp::div().class((
			css::Display::Flex,
			css::min_height!(100 vh),
		)).erase();

		let main_container = cmp::div().class((
			css::Display::Flex,
			css::FlexDirection::Column,
			css::width!(100%),
		))
		.with(|&element| element.add_component(TAB.subscribe(move |_| {
			let child = SomeElement(hobo::Children::get(element)[1]);
			child.replace_with(MainWindow::new().erase());
		})));

		element
			.child(main_container
				.child(MainWindow::new()))
	}
}

pub struct MainWindow;
impl MainWindow {
	pub fn new() -> cmp::Div {
		match *TAB.view() {
			Tab::Main => { cmp::div().class((css::size!(100%), css::background_color!(css::color::PINK))) },
		}
	}
}
