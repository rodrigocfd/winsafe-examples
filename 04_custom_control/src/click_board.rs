use std::cell::RefCell;
use std::rc::Rc;

use winsafe::{prelude::*, self as w, gui, co};

#[derive(Clone)]
pub struct ClickBoard {
	wnd:      gui::WindowControl,
	points:   Rc<RefCell<Vec<w::POINT>>>,
	fn_click: Rc<RefCell<Option<Box<dyn Fn(usize) -> w::AnyResult<()>>>>>, // click callback
}

impl ClickBoard {
	pub fn new(parent: &impl GuiParent, position: w::POINT, size: w::SIZE) -> Self {
		let wnd = gui::WindowControl::new(
			parent,
			gui::WindowControlOpts {
				class_cursor: gui::Cursor::Idc(co::IDC::CROSS),
				position,
				size,
				ex_style: gui::WindowControlOpts::default().ex_style | co::WS_EX::CLIENTEDGE,
				..Default::default()
			},
		);

		let new_self = Self {
			wnd,
			points: Rc::new(RefCell::new(Vec::default())),
			fn_click: Rc::new(RefCell::new(None)),
		};

		new_self.events();
		new_self
	}

	pub fn on_click<F>(&mut self, func: F)
		where F: Fn(usize) -> w::AnyResult<()> + 'static,
	{
		*self.fn_click.borrow_mut() = Some(Box::new(func)); // store user callback
	}

	fn events(&self) {
		let self2 = self.clone();
		self.wnd.on().wm_l_button_down(move |p| {
			let mut points = self2.points.borrow_mut();
			points.push(p.coords);
			self2.wnd.hwnd().InvalidateRect(None, true)?; // redraw now

			if let Some(fn_click) = self2.fn_click.borrow().as_ref() {
				fn_click(points.len())?; // execute user callback
			}
			Ok(())
		});

		let self2 = self.clone();
		self.wnd.on().wm_paint(move || {
			// Below, BeginPaint() returns a guard, which will call EndPaint()
			// automatically at the end of the current scope.
			let hdc = self2.wnd.hwnd().BeginPaint()?;

			hdc.MoveToEx(0, 0, None)?; // first drawn line starts from top left corner

			for pt in self2.points.borrow().iter() {
				hdc.LineTo(pt.x, pt.y)?;
			}

			Ok(())
		});
	}
}
