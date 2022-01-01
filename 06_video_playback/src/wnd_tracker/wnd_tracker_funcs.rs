use std::cell::{Cell, RefCell};
use std::rc::Rc;
use winsafe::{prelude::*, self as w, co, gui, ErrResult};

use super::WndTracker;

impl WndTracker {
	pub fn new(
		parent: &impl GuiParent,
		ctrl_id: u16,
		position: w::POINT, size: w::SIZE) -> w::ErrResult<Self>
	{
		let wnd = gui::WindowControl::new(
			parent,
			gui::WindowControlOpts {
				ctrl_id,
				position,
				size,
				horz_resize: gui::Horz::Resize,
				vert_resize: gui::Vert::Repos,
				class_cursor: w::HINSTANCE::NULL.LoadCursor(w::IdIdcStr::Idc(co::IDC::HAND))?,
				..Default::default()
			},
		);

		let new_self = Self {
			wnd,
			position_pct: Rc::new(Cell::new(0.0)),
			click_cb:     Rc::new(RefCell::new(None)),
			space_cb:     Rc::new(RefCell::new(None)),
			arrows_cb:    Rc::new(RefCell::new(None)),
		};
		new_self.events();
		Ok(new_self)
	}

	pub fn set_rendered_pos(&self, position_pct: f32) {
		self.position_pct.replace(position_pct);
		self.wnd.hwnd().InvalidateRect(None, true).unwrap();
	}

	pub fn on_click<F>(&self, cb: F)
		where F: Fn(f32) -> ErrResult<()> + 'static,
	{
		*self.click_cb.borrow_mut() = Some(Box::new(cb));
	}

	pub fn on_space<F>(&self, cb: F)
		where F: Fn() -> ErrResult<()> + 'static,
	{
		*self.space_cb.borrow_mut() = Some(Box::new(cb));
	}

	pub fn on_arrows<F>(&self, cb: F)
		where F: Fn(co::VK) -> ErrResult<()> + 'static,
	{
		*self.arrows_cb.borrow_mut() = Some(Box::new(cb));
	}
}
