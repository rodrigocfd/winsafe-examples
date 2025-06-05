use std::cell::RefCell;
use std::rc::Rc;
use winsafe::{self as w, co, gui, prelude::*};

use super::WndMain;
use crate::ids;
use crate::wnd_tracker::WndTracker;
use crate::wnd_video::WndVideo;

impl WndMain {
	pub fn new() -> Self {
		let (menu, accel_table) = Self::build_menu().unwrap();

		let wnd = gui::WindowMain::new(gui::WindowMainOpts {
			title: "DirectShow playback".to_owned(),
			style: gui::WindowMainOpts::default().style
				| co::WS::MINIMIZEBOX
				| co::WS::MAXIMIZEBOX
				| co::WS::SIZEBOX,
			class_icon: gui::Icon::Id(101),
			size: gui::dpi(700, 400),
			menu,
			accel_table: Some(accel_table),
			..Default::default()
		});

		let wnd_video = WndVideo::new(&wnd, ids::WND_VIDEO, gui::dpi(0, 0), gui::dpi(700, 380));

		let wnd_tracker =
			WndTracker::new(&wnd, ids::WND_TRACKER, gui::dpi(0, 380), gui::dpi(700, 20));

		let taskbar = Rc::new(RefCell::new(None)); // taskbar object initially not loaded

		let new_self = Self { wnd, wnd_video, wnd_tracker, taskbar };
		new_self.events();
		new_self
	}

	fn build_menu() -> w::AnyResult<(w::HMENU, w::guard::DestroyAcceleratorTableGuard)> {
		// Create file submenu.
		let file_submenu = w::HMENU::CreatePopupMenu()?;
		file_submenu.append_item(&[
			w::MenuItem::Entry {
				cmd_id: ids::MNU_FILE_OPEN,
				text: "&Open video...\tCtrl+O",
			},
			w::MenuItem::Separator,
			w::MenuItem::Entry {
				cmd_id: co::DLGID::CANCEL.into(),
				text: "E&xit",
			},
		])?;

		// Create main menu.
		let main_menu = w::HMENU::CreateMenu()?;
		main_menu.append_item(&[w::MenuItem::Submenu { submenu: &file_submenu, text: "&File" }])?;

		// Create accelerator table.
		let accel_table = w::HACCEL::CreateAcceleratorTable(&[w::ACCEL {
			fVirt: co::ACCELF::VIRTKEY | co::ACCELF::CONTROL,
			cmd: ids::MNU_FILE_OPEN,
			key: co::VK::CHAR_O,
		}])?;

		Ok((main_menu, accel_table))
	}

	pub fn run(&self) -> w::AnyResult<i32> {
		self.wnd.run_main(None)
	}
}
