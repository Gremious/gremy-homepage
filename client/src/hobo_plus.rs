use super::*;

pub trait AsElementExt: AsElement {
	// client_rect.width()/.height() are with padding + border
	// use client_width() for with padding but no borders/margins/etc
	fn width(&self) -> f64 {
		let element_rect = self.get_cmp::<web_sys::Element>().get_bounding_client_rect();
		element_rect.right() - element_rect.left()
	}

	fn height(&self) -> f64 {
		let element_rect = self.get_cmp::<web_sys::Element>().get_bounding_client_rect();
		element_rect.bottom() - element_rect.top()
	}

	#[inline]
	fn top(&self) -> f64 {
		self.get_cmp::<web_sys::Element>().get_bounding_client_rect().top()
	}

	#[inline]
	fn right(&self) -> f64 {
		self.get_cmp::<web_sys::Element>().get_bounding_client_rect().right()
	}

	#[inline]
	fn bottom(&self) -> f64 {
		self.get_cmp::<web_sys::Element>().get_bounding_client_rect().bottom()
	}

	#[inline]
	fn left(&self) -> f64 {
		self.get_cmp::<web_sys::Element>().get_bounding_client_rect().left()
	}
}

impl<T: AsElement> AsElementExt for T {}
