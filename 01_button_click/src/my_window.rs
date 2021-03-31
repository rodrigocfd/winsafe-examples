use winsafe::gui;
use winsafe::{HINSTANCE, IdIdiStr, POINT, SIZE, WinResult};

#[derive(Clone)]
pub struct MyWindow {
	wnd:       gui::WindowMain, // responsible for managing the window
	btn_hello: gui::Button,     // a button
}

impl MyWindow {
	pub fn new() -> MyWindow {
		let hinstance = HINSTANCE::GetModuleHandle(None).unwrap(); // handle to application instance

		let wnd = gui::WindowMain::new( // instantiate the window manager
			gui::WindowMainOpts {
				title: "My window title".to_owned(),
				class_icon: hinstance.LoadIcon(IdIdiStr::Id(101)).unwrap(),
				size: SIZE::new(300, 150),
				..Default::default() // leave all other options as default
			},
		);

		let btn_hello = gui::Button::new(
			&wnd, // the window manager is the parent of our button
			gui::ButtonOpts {
				text: "&Click me".to_owned(),
				position: POINT::new(20, 20),
				..Default::default()
			},
		);

		let new_self = Self { wnd, btn_hello };
		new_self.events(); // attach our events
		new_self
	}

	pub fn run(&self) -> WinResult<()> {
		self.wnd.run_main(None) // simply let the window manager do the hard work
	}

	fn events(&self) {
		let self2 = self.wnd.clone(); // clone so it can be passed into the closure

		self.btn_hello.on().bn_clicked(move || {
			self2.hwnd().SetWindowText("Hello, world!").unwrap();
		});
	}
}
