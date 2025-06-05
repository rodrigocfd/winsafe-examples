mod wnd_tracker_events;
mod wnd_tracker_funcs;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use winsafe::{self as w, co, gui};

/// Clickable progress bar at the bottom of the main window.
#[derive(Clone)]
pub struct WndTracker {
	wnd: gui::WindowControl,
	position_pct: Rc<Cell<f32>>, // 0 to 1
	click_cb: Rc<RefCell<Option<Box<dyn Fn(f32) -> w::AnyResult<()>>>>>,
	space_cb: Rc<RefCell<Option<Box<dyn Fn() -> w::AnyResult<()>>>>>,
	arrows_cb: Rc<RefCell<Option<Box<dyn Fn(co::VK) -> w::AnyResult<()>>>>>,
}
