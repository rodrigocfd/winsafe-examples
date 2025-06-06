#![cfg_attr(any(), rustfmt::skip)]

use std::cell::RefCell;
use std::rc::Rc;
use winsafe::{prelude::*, co, gui};

use crate::ids;

#[derive(Clone)]
pub struct MyModal {
	wnd:          gui::WindowModal,

	lbl_incoming: gui::Label,
	txt_incoming: gui::Edit,
	lbl_return:   gui::Label,
	txt_return:   gui::Edit,
	btn_ok:       gui::Button,
	btn_cancel:   gui::Button,

	input_val:    Rc<RefCell<String>>, // Rc/RefCell because MyModal will be cloned into closures
	return_val:   Rc<RefCell<Option<String>>>,
}

impl MyModal {
	/// Creates and displays the modal window. Blocks until the modal is closed
	/// by the user.
	pub fn show(parent: &impl GuiParent, input_text: &str) -> Option<String> {
		let dont_move = (gui::Horz::None, gui::Vert::None);

		let wnd = gui::WindowModal::new_dlg(ids::DLG_MODAL);

		let lbl_incoming = gui::Label::new_dlg(&wnd, ids::LBL_INCOMING, dont_move);
		let txt_incoming = gui::Edit::new_dlg(&wnd, ids::TXT_INCOMING, dont_move);
		let lbl_return   = gui::Label::new_dlg(&wnd, ids::LBL_RETURN, dont_move);
		let txt_return   = gui::Edit::new_dlg(&wnd, ids::TXT_RETURN, dont_move);
		let btn_ok       = gui::Button::new_dlg(&wnd, ids::BTN_OK, dont_move);
		let btn_cancel   = gui::Button::new_dlg(&wnd, ids::BTN_CANCEL, dont_move);

		let new_self = Self {
			wnd,
			lbl_incoming, txt_incoming,
			lbl_return, txt_return,
			btn_ok, btn_cancel,
			input_val: Rc::new(RefCell::new(String::from(input_text))),
			return_val: Rc::new(RefCell::new(None)),
		};

		new_self.events();

		new_self.wnd.show_modal(parent).unwrap(); // blocks until the modal is closed
		new_self.return_val.borrow()
			.as_ref()
			.map(|s| s.clone()) // return the text typed in the modal, if any
	}

	fn events(&self) {
		// This event is fired right after the window is created,
		// and right before it appears on the screen.
		let self2 = self.clone();
		self.wnd.on().wm_init_dialog(move |_| {
			self2.txt_incoming.set_text(&self2.input_val.try_borrow()?);
			Ok(true)
		});

		let self2 = self.clone();
		self.btn_ok.on().bn_clicked(move || {
			// Save the text typed by the user.
			*self2.return_val.try_borrow_mut()? = Some(self2.txt_return.text()?);
			self2.wnd.hwnd().EndDialog(0)?;
			Ok(())
		});

		let self2 = self.clone();
		self.btn_cancel.on().bn_clicked(move || {
			*self2.return_val.try_borrow_mut()? = None; // no return text
			self2.wnd.hwnd().EndDialog(0)?;
			Ok(())
		});

		let self2 = self.clone();
		self.wnd.on().wm_command_acc_menu(co::DLGID::CANCEL, move || { // ESC key
			*self2.return_val.try_borrow_mut()? = None; // no return text
			self2.wnd.hwnd().EndDialog(0)?;
			Ok(())
		});
	}
}
