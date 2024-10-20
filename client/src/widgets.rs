use crate::prelude::*;

pub fn column(row_gap: u8) -> e::Div {
	e::div().class((
		css::display::flex,
		css::flex_direction::column,
		css::row_gap::px(row_gap),
	))
}

pub fn row(column_gap: u8) -> e::Div {
	e::div().class((
		css::display::flex,
		// css::flex_direction::row,
		css::align_items::center,
		css::column_gap::px(column_gap),
	))
}
