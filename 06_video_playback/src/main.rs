#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use defer_lite::defer;
use winsafe::{prelude::*, self as w, co};

mod wnd_main;
mod wnd_tracker;
mod wnd_video;

fn main() {
	if let Err(e) = make_it_happen() {
		w::task_dlg::error(
			&w::HWND::NULL, "Unhandled error", None, &e.to_string())
			.unwrap();
	}
}

fn make_it_happen() -> w::AnyResult<i32> {
	w::CoInitializeEx(
		co::COINIT::APARTMENTTHREADED
		| co::COINIT::DISABLE_OLE1DDE)?;
	defer! { w::CoUninitialize(); }

	wnd_main::WndMain::new()
		.run()
		.map_err(|err| err.into())
}
