#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod my_window;

use winsafe::{prelude::*, self as w};
use my_window::MyWindow;

fn main() {
	if let Err(e) = run_app() {
		w::task_dlg::error(
			&w::HWND::NULL, "Unhandled error", None, &e.to_string())
			.unwrap();
	}
}

fn run_app() -> w::AnyResult<i32> {
	MyWindow::new()
		.run()
		.map_err(|err| err.into())
}
