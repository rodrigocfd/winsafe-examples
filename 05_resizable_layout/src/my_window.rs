use winsafe::{prelude::*, co, gui};
use winsafe::{ErrResult, HINSTANCE, IdIdiStr, POINT, SIZE};

#[derive(Clone)]
pub struct MyWindow {
	wnd: gui::WindowMain,
	lst: gui::ListView,
	txt: gui::Edit,
	btn: gui::Button,
}

impl MyWindow {
	pub fn new() -> ErrResult<MyWindow> {
		let hinstance = HINSTANCE::GetModuleHandle(None)?;

		let wnd = gui::WindowMain::new(
			gui::WindowMainOpts {
				title: "Resizable layout".to_owned(),
				class_icon: hinstance.LoadIcon(IdIdiStr::Id(101))?,
				size: SIZE::new(300, 150),
				style: gui::WindowMainOpts::default().style |
					co::WS::MINIMIZEBOX | co::WS::MAXIMIZEBOX | co::WS::SIZEBOX, // window can be resized
				..Default::default()
			},
		);

		let lst = gui::ListView::new(
			&wnd,
			gui::ListViewOpts {
				position: POINT::new(10, 10),
				size: SIZE::new(280, 100),
				horz_resize: gui::Horz::Resize, // resize horz/vert with parent
				vert_resize: gui::Vert::Resize,
				..Default::default()
			},
		);

		let txt = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: POINT::new(10, 120),
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
				position: POINT::new(200, 120),
				horz_resize: gui::Horz::Repos, // move anchored at parent right/bottom
				vert_resize: gui::Vert::Repos,
				..Default::default()
			},
		);

		let new_self = Self { wnd, lst, txt, btn };
		new_self.events();
		Ok(new_self)
	}

	pub fn run(&self) -> ErrResult<i32> {
		self.wnd.run_main(None)
	}

	fn events(&self) {

	}
}
