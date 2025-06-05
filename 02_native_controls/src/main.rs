#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(any(), rustfmt::skip)]

// Since MyWindow implementation is too long, we split it in three files.
mod wnd_decl;
mod wnd_events;
mod wnd_new;

use winsafe::{self as w, prelude::*, co};
use wnd_decl::MyWindow;

fn main() {
	if let Err(e) = (|| MyWindow::new().run())() {
		w::HWND::NULL.MessageBox(
			&e.to_string(), "Uncaught error", co::MB::ICONERROR).unwrap();
	}
}
