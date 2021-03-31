use winsafe::gui;
use winsafe::WinResult;

use crate::my_modal::MyModal;

#[derive(Clone)]
pub struct MyWindow {
	wnd:       gui::WindowMain,
	lbl_input: gui::Label,
	txt_input: gui::Edit,
	btn_show:  gui::Button,
}

impl MyWindow {
	pub fn new() -> MyWindow {
		// In the resource file, we defined the dialog ID as 1000,
		// and the icon ID as 101.
		let wnd = gui::WindowMain::new_dlg(1000, Some(101), None);

		let lbl_input = gui::Label::new_dlg(&wnd, 1001); // we defined the label ID as 1001
		let txt_input = gui::Edit::new_dlg(&wnd, 1002);
		let btn_show = gui::Button::new_dlg(&wnd, 1003);

		let new_self = Self { wnd, lbl_input, txt_input, btn_show };
		new_self.events();
		new_self
	}

	pub fn run(&self) -> WinResult<()> {
		self.wnd.run_main(None)
	}

	fn events(&self) {
		self.btn_show.on().bn_clicked({
			let self2 = self.clone();
			move || {
				let input_text = self2.txt_input.text().unwrap();

				let my_modal = MyModal::new(&self2.wnd, &input_text);
				let returned_text = my_modal.show();

				if let Some(text) = &returned_text {
					// If user clicked OK on the modal, a text is returned,
					// so we replace our current text with the new one.
					self2.txt_input.set_text(text).unwrap();
				}
			}
		});
	}
}
