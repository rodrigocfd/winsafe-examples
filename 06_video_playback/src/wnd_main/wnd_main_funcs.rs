use std::cell::RefCell;
use std::rc::Rc;
use winsafe::{self as w, co, gui};

use super::{ids, WndMain};
use crate::wnd_tracker::WndTracker;
use crate::wnd_video::WndVideo;

impl WndMain {
	pub fn new() -> w::ErrResult<Self> {
		let hinst = w::HINSTANCE::GetModuleHandle(None)?;
		let (menu, accel_table) = Self::build_menu()?;

		let wnd = gui::WindowMain::new(
			gui::WindowMainOpts {
				title: "DirectShow playback".to_owned(),
				style: gui::WindowMainOpts::default().style
					| co::WS::MINIMIZEBOX | co::WS::MAXIMIZEBOX | co::WS::SIZEBOX,
				class_icon: hinst.LoadIcon(w::IdIdiStr::Id(101))?,
				size: w::SIZE::new(700, 400),
				menu,
				accel_table,
				..Default::default()
			},
		);

		let wnd_video = WndVideo::new(&wnd,
			ids::WND_VIDEO, w::POINT::new(0, 0), w::SIZE::new(700, 380));

		let wnd_tracker = WndTracker::new(&wnd,
			ids::WND_TRACKER, w::POINT::new(0, 380), w::SIZE::new(700, 20));

		let taskbar = Rc::new(RefCell::new(None)); // taskbar object initially not loaded

		let new_self = Self { wnd, wnd_video, wnd_tracker, taskbar };
		new_self.events();
		Ok(new_self)
	}

	fn build_menu() -> w::ErrResult<(w::HMENU, w::HACCEL)> {
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
			w::MenuEnum::Submenu(file_submenu, "&File"),
		])?;

		// Create accelerator table.
		let accel_table = w::HACCEL::CreateAcceleratorTable(&mut [
			w::ACCEL {
				fVirt: co::ACCELF::VIRTKEY | co::ACCELF::CONTROL,
				cmd: ids::MNU_FILE_OPEN,
				key: co::VK::from('O' as u16),
			},
		])?;

		Ok((main_menu, accel_table))
	}

	pub fn run(&self) -> w::ErrResult<i32> {
		self.wnd.run_main(None)
	}
}
