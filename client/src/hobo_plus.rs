use super::*;

/// Creates a fn that merges the closures for every event:
///
///```
/// fn with_on_click(self, mut f: impl FnMut(&Self, hobo::web_sys::MouseEvent) + 'static) -> Self where Self: Sized + 'static + Copy {
/// 	self.add_on_click(move |e| f(&self, e));
/// 	self
/// }
/// ```
///
/// So that you can do:
///
/// ```
/// cmp::div()
/// 	.with_on_click(|&element, event| { /* ... */ })
/// ```
///
macro_rules! with_events {
	($($event_kind:ident, $f:ident);+$(;)*) => {paste::item!{
		$(
			fn [<with_ $f>](self, mut f: impl FnMut(&Self, hobo::web_sys::$event_kind) + 'static) -> Self where Self: 'static + Copy {
				self.[<add_ $f>](move |e| f(&self, e));
				self
			}
		)+
	}}
}

pub trait ElementExt: Element {
	with_events!{
		MouseEvent,           on_click;
		MouseEvent,    on_context_menu;
		MouseEvent,       on_dbl_click;
		MouseEvent,      on_mouse_down;
		MouseEvent,     on_mouse_enter;
		MouseEvent,     on_mouse_leave;
		MouseEvent,      on_mouse_move;
		MouseEvent,      on_mouse_over;
		MouseEvent,       on_mouse_out;
		MouseEvent,        on_mouse_up;
		KeyboardEvent,     on_key_down;
		KeyboardEvent,       on_key_up;
		Event,               on_change;
		UiEvent,             on_scroll;
		FocusEvent,            on_blur;
		FocusEvent,           on_focus;
		TouchEvent,     on_touch_start;
		TouchEvent,       on_touch_end;
		TouchEvent,      on_touch_move;
		TouchEvent,    on_touch_cancel;
		WheelEvent,           on_wheel;
		Event,                 on_load;
	}

	/// Equivalent to .with(|&element| element.add_child( ... ))
	fn with_child<T: Element>(self, f: impl FnOnce(&Self) -> T) -> Self where Self: Sized + 'static + Copy {
		self.add_child(f(&self));
		self
	}
}

impl<T: Element> ElementExt for T {}
