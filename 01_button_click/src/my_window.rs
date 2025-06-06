#![cfg_attr(any(), rustfmt::skip)]

use winsafe::{prelude::*, gui, AnyResult};

/// Main application window.
#[derive(Clone)]
pub struct MyWindow {
	wnd:       gui::WindowMain, // responsible for managing the window
	btn_hello: gui::Button,     // a button
}

impl MyWindow {
	pub fn new() -> Self {
		let wnd = gui::WindowMain::new( // instantiate the window manager
			gui::WindowMainOpts {
				title:      "My window title".to_owned(),
				class_icon: gui::Icon::Id(101), // load icon from resource ID 101
				size:       (300, 150),
				..Default::default() // leave all other options as default
			},
		);

		let btn_hello = gui::Button::new(
			&wnd, // the window manager is the parent of our button
			gui::ButtonOpts {
				text:     "&Click me".to_owned(),
				position: (20, 20),
				..Default::default()
			},
		);

		let new_self = Self { wnd, btn_hello };
		new_self.events(); // attach our events
		new_self
	}

	pub fn run(&self) -> AnyResult<i32> {
		self.wnd.run_main(None) // simply let the window manager do the hard work
	}

	fn events(&self) {
		let self2 = self.clone();
		self.btn_hello.on().bn_clicked(move || { // button click event
			self2.wnd.hwnd().SetWindowText("Hello, world!")?;
			Ok(())
		});
	}
}
