#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(any(), rustfmt::skip)]

mod my_window;

use winsafe::{prelude::*, co, AnyResult, HWND};
use my_window::MyWindow;

fn main() {
	if let Err(e) = run_app() {
		HWND::NULL.MessageBox(
			&e.to_string(), "Uncaught error", co::MB::ICONERROR).unwrap();
	}
}

fn run_app() -> AnyResult<i32> {
	MyWindow::new() // create our main window...
		.run()      // ...and run it
		.map_err(|err| err.into())
}
