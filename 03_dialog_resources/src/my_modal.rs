use std::cell::RefCell;
use std::rc::Rc;
use winsafe::{prelude::*, co, gui, ErrResult};

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
	pub fn new(parent: &impl Parent, input_text: &str) -> MyModal {
		let dont_move = (gui::Horz::None, gui::Vert::None);

		let wnd = gui::WindowModal::new_dlg(parent, ids::DLG_MODAL);

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
		new_self
	}

	pub fn show(&self) -> ErrResult<Option<String>> {
		self.wnd.show_modal()?;
		Ok(self.return_val.as_ref().try_borrow()?.clone()) // return the text typed in the modal
	}

	fn events(&self) {
		// This event is fired right after the window is created,
		// and right before it appears on the screen.
		self.wnd.on().wm_init_dialog({
			let self2 = self.clone();
			move |_| {
				self2.txt_incoming.set_text(&self2.input_val.try_borrow()?)?;
				Ok(true)
			}
		});

		self.btn_ok.on().bn_clicked({
			let self2 = self.clone();
			move || {
				// Save the text typed by the user.
				*self2.return_val.try_borrow_mut()? = Some(self2.txt_return.text()?);
				self2.wnd.hwnd().EndDialog(0)?;
				Ok(())
			}
		});

		self.btn_cancel.on().bn_clicked({
			let self2 = self.clone();
			move || {
				*self2.return_val.try_borrow_mut()? = None; // no return text
				self2.wnd.hwnd().EndDialog(0)?;
				Ok(())
			}
		});

		self.wnd.on().wm_command_accel_menu(co::DLGID::CANCEL.into(), { // close on ESC key
			let self2 = self.clone();
			move || {
				*self2.return_val.try_borrow_mut()? = None; // no return text
				self2.wnd.hwnd().EndDialog(0)?;
				Ok(())
			}
		});
	}
}
