#![cfg_attr(any(), rustfmt::skip)]

use winsafe::{self as w, gui, co, prelude::*};

use crate::tab_container1::TabContainer1;
use crate::tab_container2::TabContainer2;

/// Main application window.
#[derive(Clone)]
pub struct MyWindow {
	wnd:      gui::WindowMain,
	tab_ctrl: gui::Tab,
}

impl MyWindow {
	pub fn create_and_run() -> w::AnyResult<i32> {
		let wnd = gui::WindowMain::new( // create the main window
			gui::WindowMainOpts {
				title:      "Tabs",
				class_icon: gui::Icon::Id(101),
				size:       gui::dpi(300, 150),
				style:      gui::WindowMainOpts::default().style | co::WS::MINIMIZEBOX, // add a minimize button
				..Default::default()
			},
		);

		let tab_ctrl = gui::Tab::new( // create the container tab control
			&wnd,
			gui::TabOpts {
				position: gui::dpi(10, 10),
				size:     gui::dpi(280, 130),
				pages:    &[
					("First",  TabContainer1::new(&wnd).into()), // create the 2 tab pages
					("Second", TabContainer2::new(&wnd).into()),
				],
				..Default::default()
			},
		);

		let new_self = Self { wnd, tab_ctrl };
		new_self.events();
		new_self.wnd.run_main(None)
	}

	fn events(&self) {

	}
}
