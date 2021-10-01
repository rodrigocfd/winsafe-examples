#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod my_window;

use winsafe::{co, BoxResult, HWND};
use my_window::MyWindow;

fn main() {
	if let Err(e) = run_app() {
		HWND::NULL.MessageBox(
			&e.to_string(), "Uncaught error", co::MB::ICONERROR).unwrap();
	}
}

fn run_app() -> BoxResult<i32> {
	MyWindow::new()?.run() // create our main window and run it
}
