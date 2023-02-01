use winsafe::{prelude::*, self as w, gui, co};

use crate::tab_container1::TabContainer1;
use crate::tab_container2::TabContainer2;

#[derive(Clone)]
pub struct MyWindow {
	wnd:      gui::WindowMain,
	tab_ctrl: gui::Tab,
}

impl MyWindow {
	pub fn new() -> Self {
		let wnd = gui::WindowMain::new(
			gui::WindowMainOpts {
				title: "Tabs".to_owned(),
				class_icon: gui::Icon::Id(101),
				size: (300, 150),
				style: gui::WindowMainOpts::default().style | co::WS::MINIMIZEBOX, // add a minimize button
				..Default::default()
			},
		);

		let tab_ctrl = gui::Tab::new(
			&wnd,
			gui::TabOpts {
				position: (10, 10),
				size: (280, 130),
				items: vec![
					("First".to_owned(), Box::new(TabContainer1::new(&wnd))), // create the 2 tabs
					("Second".to_owned(), Box::new(TabContainer2::new(&wnd))),
				],
				..Default::default()
			},
		);

		let new_self = Self { wnd, tab_ctrl };
		new_self.events();
		new_self
	}

	pub fn run(&self) -> w::AnyResult<i32> {
		self.wnd.run_main(None)
	}

	fn events(&self) {

	}
}
