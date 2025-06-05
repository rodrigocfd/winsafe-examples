#![cfg_attr(any(), rustfmt::skip)]

use winsafe::{self as w, prelude::*, gui, co};

use crate::click_board::ClickBoard;

/// Main application window.
#[derive(Clone)]
pub struct MyWindow {
	wnd:         gui::WindowMain,
	click_board: ClickBoard,
}

impl MyWindow {
	pub fn new() -> Self {
		let wnd = gui::WindowMain::new(
			gui::WindowMainOpts {
				title:      "Custom control".to_owned(),
				class_icon: gui::Icon::Id(101),
				size:       gui::dpi(300, 150),
				style:      gui::WindowMainOpts::default().style | co::WS::MINIMIZEBOX, // add a minimize button
				..Default::default()
			},
		);

		let click_board = ClickBoard::new(
			&wnd,
			gui::dpi(10, 10),
			gui::dpi(280, 130),
		);

		let mut new_self = Self { wnd, click_board };
		new_self.events();
		new_self
	}

	pub fn run(&self) -> w::AnyResult<i32> {
		self.wnd.run_main(None)
	}

	fn events(&mut self) {
		let wnd = self.wnd.clone();
		self.click_board.on_click(move |num_points| { // click event of our custom control
			wnd.hwnd().SetWindowText(&format!("Points: {}", num_points))?;
			Ok(())
		});
	}
}
