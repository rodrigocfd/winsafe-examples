use std::cell::{Cell, RefCell};
use std::rc::Rc;
use winsafe::{self as w, co, gui};

mod wnd_tracker_events;
mod wnd_tracker_funcs;

#[derive(Clone)]
pub struct MyTracker {
	wnd:          gui::WindowControl,
	position_pct: Rc<Cell<f32>>, // 0 to 1
	click_cb:     Rc<RefCell<Option<Box<dyn Fn(f32) -> w::ErrResult<()>>>>>,
	space_cb:     Rc<RefCell<Option<Box<dyn Fn() -> w::ErrResult<()>>>>>,
	arrows_cb:    Rc<RefCell<Option<Box<dyn Fn(co::VK) -> w::ErrResult<()>>>>>,
}
