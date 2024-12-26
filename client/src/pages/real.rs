use crate::prelude::*;
mod one;
use crate::pages::Clicked;
// use crate::widgets as w;

pub fn new() -> e::Div {
	let parent = e::div();

	let grid_items = [
		one::new().text("one"),
		one::new().text("two"),
		one::new().text("three"),
		one::new().text("four"),
		one::new().text("five"),
		one::new().text("six"),
		one::new().text("seven"),
		one::new().text("eight"),
		one::new().text("nine"),
	];
	let center = grid_items[4];

	parent
		.tap(|&ele| {
			ele.add_on_dom_attach(move || {
				let viewport_height = document().document_element().unwrap().client_height() as f64;
				let viewport_width = document().document_element().unwrap().client_width() as f64;

				if viewport_width < viewport_height {
					for child in &grid_items {
						child.set_style((
							css::width::vw(100),
						));
					}
				} else {
					for child in &grid_items {
						child.set_style(css::height::vh(100));
					}
				}


				let body_left = document().body().unwrap().get_bounding_client_rect().left();
				let body_top = document().body().unwrap().get_bounding_client_rect().top();

				let ele_top = center.top();
				let ele_left = center.left();

				let viewport_height = document().document_element().unwrap().client_height();
				let viewport_width = document().document_element().unwrap().client_width();

				let extra_height = viewport_height as f64 - center.height();
				let extra_width = viewport_width as f64 - center.width();

				let top = (ele_top - body_top) - (extra_height / 2.0);
				let left = ele_left - body_left - (extra_width / 2.0);

				log::debug!("top: {}, left: {}", top,  left);
				window().scroll_to_with_x_and_y(left, top);
			})
		})
		.class((
			css::display::grid,
			"grid-template-columns: repeat(3, 1fr);",
			"grid-template-rows: repeat(3, 1fr);",
			css::color::rgba(css::colors::WHITE)
		))
		.component({
			let closure = move || {
				let child_height = grid_items[0].height();
				let child_width = grid_items[0].width();

				let viewport_height = document().document_element().unwrap().client_height() as f64;
				let viewport_width = document().document_element().unwrap().client_width() as f64;

				if viewport_width < child_width {
					for child in &grid_items {
						child.set_style(css::width::vw(100));
					}
				} else if viewport_height < child_height {
					for child in &grid_items {
						child.set_style(css::height::vh(100));
					}
				}
			};

			let f = Closure::wrap(Box::new(closure) as Box<dyn FnMut()>);
			window().set_onresize(Some(f.as_ref().unchecked_ref()));
			f
		})
		.children(grid_items)
		.tap(|&element| element.add_on_mouse_down(move |_| {
			element.set_class_tagged("Cursor", css::class!(css::cursor::grabbing));
			*element.get_cmp_mut::<Clicked>() = Clicked(true);
		}))
		.tap(|&element| element.add_on_mouse_up(move |_| {
			element.set_class_tagged("Cursor", css::class!(css::cursor::auto));
			element.get_cmp_mut::<Clicked>().0 = false;
		}))
		.tap(|&element| element.add_on_mouse_leave(move |_| {
			element.set_class_tagged("Cursor", css::class!(css::cursor::auto));
			element.get_cmp_mut::<Clicked>().0 = false;
		}))
		.tap(|&element| element.add_on_mouse_move(move |mouse_event| {
			if element.get_cmp::<Clicked>().0 {
				let (move_x, move_y) = (mouse_event.movement_x(), mouse_event.movement_y());
				let move_x = move_x as f64;
				let move_y = move_y as f64;
				web_sys::window().unwrap().scroll_by_with_x_and_y(-move_x, -move_y);
			}
		}))
		.component(Clicked(false))
}
