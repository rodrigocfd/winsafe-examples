#![cfg_attr(any(), rustfmt::skip)]

use winsafe::{self as w, gui, co};

/// Main application window.
#[derive(Clone)]
pub struct MyWindow {
	wnd: gui::WindowMain,
	lst: gui::ListView,
	txt: gui::Edit,
	btn: gui::Button,
}

impl MyWindow {
	pub fn new() -> Self {
		let wnd = gui::WindowMain::new(
			gui::WindowMainOpts {
				title:      "Resizable layout",
				class_icon: gui::Icon::Id(101),
				size:       gui::dpi(300, 150),
				style:      gui::WindowMainOpts::default().style |
					co::WS::MINIMIZEBOX | co::WS::MAXIMIZEBOX | co::WS::SIZEBOX, // window can be resized
				..Default::default()
			},
		);

		let lst = gui::ListView::new(
			&wnd,
			gui::ListViewOpts {
				position: gui::dpi(10, 10),
				size:     gui::dpi(280, 100),
				// Resize horizontally and vertically together with parent window.
				resize_behavior: (gui::Horz::Resize, gui::Vert::Resize),
				..Default::default()
			},
		);

		let txt = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: gui::dpi(10, 120),
				width:    gui::dpi_x(180),
				// Resize horizontally together with parent window.
				// Move anchored at bottom as parent window resizes.
				resize_behavior: (gui::Horz::Resize, gui::Vert::Repos),
				..Default::default()
			},
		);

		let btn = gui::Button::new(
			&wnd,
			gui::ButtonOpts {
				text:     "&Button",
				position: gui::dpi(200, 120),
				// Move anchored at right/bottom as parent window resizes.
				resize_behavior: (gui::Horz::Repos, gui::Vert::Repos),
				..Default::default()
			},
		);

		let new_self = Self { wnd, lst, txt, btn };
		new_self.events();
		new_self
	}

	pub fn run(&self) -> w::AnyResult<i32> {
		self.wnd.run_main(None)
	}

	fn events(&self) {
		// no events being handled
	}
}
