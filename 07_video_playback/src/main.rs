#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use winsafe::{self as w, prelude::*, co};

mod wnd_main;
mod wnd_tracker;
mod wnd_video;

fn main() {
	if let Err(e) = run_main_window() {
		w::HWND::NULL.TaskDialog(
			None,
			Some("Unhandled error"),
			None,
			Some(&e.to_string()),
			co::TDCBF::OK,
			w::IconRes::Error,
		).unwrap();
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
