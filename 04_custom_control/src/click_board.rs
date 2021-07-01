use std::cell::RefCell;
use std::rc::Rc;

use winsafe::{co, gui, msg};
use winsafe::{HINSTANCE, IdIdc, PAINTSTRUCT, POINT, SIZE};

#[derive(Clone)]
pub struct ClickBoard {
	wnd:      gui::WindowControl,
	points:   Rc<RefCell<Vec<POINT>>>,
	fn_click: Rc<RefCell<Option<Box<dyn Fn(usize)>>>>, // click callback
}

impl ClickBoard {
	pub fn new(parent: &dyn gui::Parent, position: POINT, size: SIZE) -> ClickBoard {
		let wnd = gui::WindowControl::new(
			parent,
			gui::WindowControlOpts {
				class_cursor: HINSTANCE::NULL.LoadCursor(IdIdc::Idc(co::IDC::CROSS)).unwrap(),
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

	pub fn on_click<F: Fn(usize) + 'static>(&mut self, func: F) {
		*self.fn_click.borrow_mut() = Some(Box::new(func)); // store user callback
	}

	fn events(&self) {
		self.wnd.on().wm_l_button_down({
			let self2 = self.clone();
			move |p: msg::wm::LButtonDown| {
				let mut points = self2.points.borrow_mut();
				points.push(p.coords);
				self2.wnd.hwnd().InvalidateRect(None, true).unwrap(); // redraw now

				if let Some(fn_click) = self2.fn_click.borrow().as_ref() {
					fn_click(points.len()); // execute user callback
				}
			}
		});

		self.wnd.on().wm_paint({
			let self2 = self.clone();
			move || {
				let mut ps = PAINTSTRUCT::default();
				let hdc = self2.wnd.hwnd().BeginPaint(&mut ps).unwrap();

				hdc.MoveToEx(0, 0, None).unwrap(); // first line starts from top left corner

				for pt in self2.points.borrow().iter() {
					hdc.LineTo(pt.x, pt.y).unwrap();
				}

				self2.wnd.hwnd().EndPaint(&ps);
			}
		});
	}
}
