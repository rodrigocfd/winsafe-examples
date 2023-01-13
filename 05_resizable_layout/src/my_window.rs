use winsafe::{prelude::*, self as w, gui, co};

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
				title: "Resizable layout".to_owned(),
				class_icon: gui::Icon::Id(101),
				size: w::SIZE::new(300, 150),
				style: gui::WindowMainOpts::default().style |
					co::WS::MINIMIZEBOX | co::WS::MAXIMIZEBOX | co::WS::SIZEBOX, // window can be resized
				..Default::default()
			},
		);

		let lst = gui::ListView::new(
			&wnd,
			gui::ListViewOpts {
				position: w::POINT::new(10, 10),
				size: w::SIZE::new(280, 100),
				horz_resize: gui::Horz::Resize, // resize horz/vert with parent
				vert_resize: gui::Vert::Resize,
				..Default::default()
			},
		);

		let txt = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: w::POINT::new(10, 120),
				width: 180,
				horz_resize: gui::Horz::Resize, // resize horizontally with parent
				vert_resize: gui::Vert::Repos,  // move anchored at parent bottom
				..Default::default()
			},
		);

		let btn = gui::Button::new(
			&wnd,
			gui::ButtonOpts {
				text: "&Button".to_owned(),
				position: w::POINT::new(200, 120),
				horz_resize: gui::Horz::Repos, // move anchored at parent right/bottom
				vert_resize: gui::Vert::Repos,
				..Default::default()
			},
		);

		let new_self = Self { wnd, lst, txt, btn };
		new_self.events();
		new_self
	}

	pub fn run(&self) -> gui::MsgResult<i32> {
		self.wnd.run_main(None)
	}

	fn events(&self) {

	}
}
