mod wnd_main_events;
mod wnd_main_funcs;

use std::cell::RefCell;
use std::rc::Rc;
use winsafe::{self as w, gui};

use crate::wnd_tracker::WndTracker;
use crate::wnd_video::WndVideo;

/// Main application window.
#[derive(Clone)]
pub struct WndMain {
	wnd: gui::WindowMain,
	wnd_video: WndVideo,
	wnd_tracker: WndTracker,
	taskbar: Rc<RefCell<Option<w::ITaskbarList4>>>,
}
