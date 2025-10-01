mod wnd_video_events;
mod wnd_video_funcs;

use std::cell::RefCell;
use std::rc::Rc;
use winsafe::{self as w, gui};

/// Child control which renders the video.
#[derive(Clone)]
pub struct WndVideo {
	wnd: gui::WindowControl,
	com_objs: Rc<RefCell<Option<ComObjs>>>,
}

struct ComObjs {
	media_ctrl: w::IMediaControl, // drop order is important
	media_seek: w::IMediaSeeking,
	controller_evr: w::IMFVideoDisplayControl,
	vmr: w::IBaseFilter,
	graph_builder: w::IGraphBuilder,
}
