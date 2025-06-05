#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ids;
mod wnd_main;
mod wnd_tracker;
mod wnd_video;

use winsafe::{self as w, co, prelude::*};

fn main() {
	if let Err(e) = (|| {
		let _com_lib =
			w::CoInitializeEx(co::COINIT::APARTMENTTHREADED | co::COINIT::DISABLE_OLE1DDE)?;
		wnd_main::WndMain::new().run()
	})() {
		w::HWND::NULL
			.MessageBox(&e.to_string(), "Uncaught error", co::MB::ICONERROR)
			.unwrap();
	}
}
