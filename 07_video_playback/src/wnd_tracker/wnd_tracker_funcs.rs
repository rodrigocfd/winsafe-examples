use std::cell::{Cell, RefCell};
use std::rc::Rc;
use winsafe::{self as w, co, gui, prelude::*};

use super::WndTracker;

impl WndTracker {
	pub fn new(
		parent: &(impl GuiParent + 'static),
		ctrl_id: u16,
		position: (i32, i32),
		size: (i32, i32),
	) -> Self {
		let wnd = gui::WindowControl::new(
			parent,
			gui::WindowControlOpts {
				ctrl_id,
				position,
				size,
				resize_behavior: (gui::Horz::Resize, gui::Vert::Repos),
				class_cursor: gui::Cursor::Idc(co::IDC::HAND),
				..Default::default()
			},
		);

		let new_self = Self {
			wnd,
			position_pct: Rc::new(Cell::new(0.0)),
			click_cb: Rc::new(RefCell::new(None)),
			space_cb: Rc::new(RefCell::new(None)),
			arrows_cb: Rc::new(RefCell::new(None)),
		};
		new_self.events();
		new_self
	}

	pub fn set_rendered_pos(&self, position_pct: f32) -> w::SysResult<()> {
		self.position_pct.replace(position_pct);
		self.wnd.hwnd().InvalidateRect(None, true)?;
		Ok(())
	}

	pub fn on_click<F>(&self, cb: F)
	where
		F: Fn(f32) -> w::AnyResult<()> + 'static,
	{
		*self.click_cb.borrow_mut() = Some(Box::new(cb));
	}

	pub fn on_space<F>(&self, cb: F)
	where
		F: Fn() -> w::AnyResult<()> + 'static,
	{
		*self.space_cb.borrow_mut() = Some(Box::new(cb));
	}

	pub fn on_arrows<F>(&self, cb: F)
	where
		F: Fn(co::VK) -> w::AnyResult<()> + 'static,
	{
		*self.arrows_cb.borrow_mut() = Some(Box::new(cb));
	}
}
