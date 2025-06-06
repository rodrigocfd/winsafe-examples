#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(any(), rustfmt::skip)]

mod my_window;
mod tab_container1;
mod tab_container2;

use winsafe::{self as w, prelude::*, co};
use my_window::MyWindow;

fn main() {
	if let Err(e) = (|| MyWindow::new().run())() {
		w::HWND::NULL.MessageBox(
			&e.to_string(), "Uncaught error", co::MB::ICONERROR).unwrap();
	}
}
