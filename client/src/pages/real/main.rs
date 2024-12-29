use crate::prelude::*;
use crate::widgets as w;

pub fn new() -> e::Div {
	w::column(32)
		.class((
			css::size::pct(100.),
			css::padding::px(12),
		))
		.child(w::row(32)
			.class((
				css::size::pct(100.),
			))
			.child(w::column(32)
				.class((
					css::width::pct(40.),
					css::height::pct(100.),
				))
				.child(wip_box())
				.child(wip_box())
			)
			.child(w::column(32)
				.class((
					css::width::pct(30.),
					css::height::pct(100.),
				))
				.child(wip_box())
			)
			.child(w::column(32)
				.class((
					css::width::pct(30.),
					css::height::pct(100.),
				))
				.child(black_box()
					.class((
						css::padding::px(12),
						css::max_height::pct(33.33),
						css::white_space::break_spaces,
					))
					.child(w::row(12)
						.class((
							css::align_items::start,
							css::height::pct(40.),
						))
						.child(e::img()
							.class((
								css::height::pct(100.),
								css::border_style::solid,
								css::border_color::rgba(colors::pink_magenta),
								css::border_width::px(2),
								css::pointer_events::none,
							))
							.src("../public/img/avi.png")
						)
						.child(w::column(0)
							.class((
								css::height::pct(100.),
								css::justify_content::space_between,
							))
							.child(e::p()
								.child(e::span().text("I'm Gremious.\n"))
								.child(e::span().class(css::color::rgba(colors::pink_magenta)).text("He/Him"))
								.child(e::span().text(" unless I'm wearing eyeliner.\n"))
							)
							.child(e::p()
								.child(e::span().text("I hope you have a nice day."))
							)

						)
					)
					.child(w::row(0)
						.child(e::p().child(e::span().text("Status: ")))
						.child(e::p().class(css::color::rgba(css::colors::GREEN)).child(e::span().text("OK.")))
					)
					// Stop if you need the mouse to not move
					// .on_mouse_down(|e| e.stop_propagation())
				)
				.child(wip_box().class(css::max_height::pct(33.33)))
				.child(wip_box().class(css::max_height::pct(33.33)))
				// .child(wip_box())
		)
	)
	.child(w::row(8)
		.child(wip_box().class((
			css::width::pct(100.),
			css::height::px(31),
		)))
	)
}

fn black_box() -> e::Div {
	w::column(8)
		.class((
			css::size::pct(100.),
			css::cursor::auto,
			css::background_color::rgba(colors::bg_black),
			css::border_style::solid,
			css::border_color::rgba(colors::pink_magenta),
			css::border_width::px(2),
		))
}

fn wip_box() -> e::Div {
	e::div()
		.class((
			css::size::pct(100.),
			css::background_color::rgba(colors::debug_blue),
		))
		.child(e::div()
			.class((
				css::size::pct(100.),
				css::background_image::url("../public/img/vhs_noise.webp"),
				css::opacity::val(0.7),
			))
		)
}
