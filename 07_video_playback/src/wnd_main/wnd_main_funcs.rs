use std::cell::RefCell;
use std::rc::Rc;
use winsafe::{prelude::*, self as w, co, gui};

use super::{ids, WndMain};
use crate::wnd_tracker::WndTracker;
use crate::wnd_video::WndVideo;

impl WndMain {
	pub fn new() -> Self {
		let (menu, accel_table) = Self::build_menu().unwrap();

		let wnd = gui::WindowMain::new(
			gui::WindowMainOpts {
				title: "DirectShow playback".to_owned(),
				style: gui::WindowMainOpts::default().style
					| co::WS::MINIMIZEBOX | co::WS::MAXIMIZEBOX | co::WS::SIZEBOX,
				class_icon: gui::Icon::Id(101),
				size: (700, 400),
				menu,
				accel_table: Some(accel_table),
				..Default::default()
			},
		);

		let wnd_video = WndVideo::new(&wnd, ids::WND_VIDEO, (0, 0), (700, 380));

		let wnd_tracker = WndTracker::new(&wnd, ids::WND_TRACKER, (0, 380), (700, 20));

		let taskbar = Rc::new(RefCell::new(None)); // taskbar object initially not loaded

		let new_self = Self { wnd, wnd_video, wnd_tracker, taskbar };
		new_self.events();
		new_self
	}

	fn build_menu() -> w::AnyResult<(w::HMENU, w::guard::DestroyAcceleratorTableGuard)> {
		// Create file submenu.
		let file_submenu = w::HMENU::CreatePopupMenu()?;

		file_submenu.AppendMenuEnum(&[
			w::MenuEnum::Entry(ids::MNU_FILE_OPEN, "&Open video...\tCtrl+O"),
			w::MenuEnum::Separator,
			w::MenuEnum::Entry(co::DLGID::CANCEL.into(), "E&xit"),
		])?;

		// Create main menu.
		let main_menu = w::HMENU::CreateMenu()?;

		main_menu.AppendMenuEnum(&[
			w::MenuEnum::Submenu(&file_submenu, "&File"),
		])?;

		// Create accelerator table.
		let accel_table = w::HACCEL::CreateAcceleratorTable(&mut [
			w::ACCEL {
				fVirt: co::ACCELF::VIRTKEY | co::ACCELF::CONTROL,
				cmd: ids::MNU_FILE_OPEN,
				key: co::VK::CHAR_O,
			},
		])?;

		Ok((main_menu, accel_table))
	}

	pub fn run(&self) -> w::AnyResult<i32> {
		self.wnd.run_main(None)
	}
}
