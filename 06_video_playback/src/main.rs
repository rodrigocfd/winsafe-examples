#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use defer_lite::defer;
use winsafe::{prelude::*, self as w};

mod wnd_main;
mod wnd_tracker;
mod wnd_video;

fn main() {
	if let Err(e) = do_shit() {
		w::HWND::NULL.MessageBox(&e.to_string(),
			"Unhandled error", w::co::MB::ICONERROR).unwrap();
	}
}

fn do_shit() -> w::ErrResult<i32> {
	w::CoInitializeEx(w::co::COINIT::APARTMENTTHREADED)?;
	defer! { w::CoUninitialize(); }

	wnd_main::MyMain::new()?.run()
}
