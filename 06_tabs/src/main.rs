#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod my_window;
mod tab_container1;
mod tab_container2;

use winsafe::{self as w, prelude::*, co};
use my_window::MyWindow;

fn main() {
	if let Err(e) = run_app() {
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

fn run_app() -> w::AnyResult<i32> {
	MyWindow::new()
		.run()
		.map_err(|err| err.into())
}
