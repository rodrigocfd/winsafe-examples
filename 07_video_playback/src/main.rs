#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use winsafe::{prelude::*, self as w, co};

mod wnd_main;
mod wnd_tracker;
mod wnd_video;

fn main() {
	if let Err(e) = run_main_window() {
		w::task_dlg::error(
			&w::HWND::NULL, "Unhandled error", None, &e.to_string())
			.unwrap();
	}
}

fn run_main_window() -> w::AnyResult<i32> {
	let _com_lib = w::CoInitializeEx(
		co::COINIT::APARTMENTTHREADED
		| co::COINIT::DISABLE_OLE1DDE)?;

	wnd_main::WndMain::new()
		.run()
		.map_err(|err| err.into())
}
