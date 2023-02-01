use winsafe::{prelude::*, gui, AnyResult};

use crate::ids;
use crate::my_modal::MyModal;

#[derive(Clone)]
pub struct MyWindow {
	wnd:       gui::WindowMain,
	lbl_input: gui::Label,
	txt_input: gui::Edit,
	btn_show:  gui::Button,
}

impl MyWindow {
	pub fn new() -> Self {
		let dont_move = (gui::Horz::None, gui::Vert::None);

		let wnd = gui::WindowMain::new_dlg(ids::DLG_MAIN, Some(ids::ICO_MAIN), None);

		let lbl_input = gui::Label::new_dlg(&wnd, ids::LBL_INPUT, dont_move);
		let txt_input = gui::Edit::new_dlg(&wnd, ids::TXT_INPUT, dont_move);
		let btn_show  = gui::Button::new_dlg(&wnd, ids::BTN_SHOW, dont_move);

		let new_self = Self { wnd, lbl_input, txt_input, btn_show };
		new_self.events();
		new_self
	}

	pub fn run(&self) -> AnyResult<i32> {
		self.wnd.run_main(None)
	}

	fn events(&self) {
		let self2 = self.clone();
		self.btn_show.on().bn_clicked(move || {
			let input_text = self2.txt_input.text();

			let my_modal = MyModal::new(&self2.wnd, &input_text);
			let returned_text = my_modal.show();

			if let Some(text) = &returned_text {
				// If user clicked OK on the modal, a text is returned,
				// so we replace our current text with the new one.
				self2.txt_input.set_text(text);
			}
			Ok(())
		});
	}
}
