use std::cell::RefCell;
use std::rc::Rc;
use winsafe::{gui, shell};

use crate::wnd_tracker::MyTracker;
use crate::wnd_video::MyVideo;

mod ids;
mod wnd_main_events;
mod wnd_main_funcs;

#[derive(Clone)]
pub struct MyMain {
	wnd:         gui::WindowMain,
	wnd_video:   MyVideo,
	wnd_tracker: MyTracker,
	taskbar:     Rc<RefCell<Option<shell::ITaskbarList4>>>,
}
