use winsafe::{co, gui};
use winsafe::{HINSTANCE, IdIdiStr, POINT, SIZE, WinResult};

#[derive(Clone)]
pub struct MyWindow {
	wnd:     gui::WindowMain,
	lst:     gui::ListView,
	txt:     gui::Edit,
	btn:     gui::Button,
	resizer: gui::Resizer,
}

impl MyWindow {
	pub fn new() -> MyWindow {
		let hinstance = HINSTANCE::GetModuleHandle(None).unwrap();

		let wnd = gui::WindowMain::new(
			gui::WindowMainOpts {
				title: "Resizable layout".to_owned(),
				class_icon: hinstance.LoadIcon(IdIdiStr::Id(101)).unwrap(),
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
				..Default::default()
			},
		);

		let txt = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: POINT::new(10, 120),
				width: 180,
				..Default::default()
			},
		);

		let btn = gui::Button::new(
			&wnd,
			gui::ButtonOpts {
				text: "&Click".to_owned(),
				position: POINT::new(200, 120),
				..Default::default()
			},
		);

		let resizer = gui::Resizer::new(&wnd);

		let new_self = Self { wnd, lst, txt, btn, resizer };
		new_self.events();
		new_self
	}

	pub fn run(&self) -> WinResult<()> {
		self.wnd.run_main(None)
	}

	fn events(&self) {
		self.wnd.on().wm_create({
			let self2 = self.clone();
			move |_| {

				// Setup the resizer control, passing the horizontal/vertical
				// behavior of each control.
				self2.resizer

					// Horizontally/vertically: resize with parent window.
					.add(gui::Resz::Resize, gui::Resz::Resize, &[&self2.lst])

					// Horizontally: resize with parent window.
					// Vertically: move the control anchored at parent window bottom.
					.add(gui::Resz::Resize, gui::Resz::Repos, &[&self2.txt])

					// Horizontally/vertically: move the control anchored at parent window right/bottom.
					.add(gui::Resz::Repos, gui::Resz::Repos, &[&self2.btn]);

				0
			}
		});
	}
}
