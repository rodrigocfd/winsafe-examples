use std::cell::RefCell;
use std::rc::Rc;
use winsafe::{self as w, gui};

mod wnd_video_events;
mod wnd_video_funcs;

#[derive(Clone)]
pub struct WndVideo {
	wnd:      gui::WindowControl,
	com_objs: Rc<RefCell<Option<ComObjs>>>,
}

struct ComObjs {
	graph_builder:  w::IGraphBuilder,
	vmr:            w::IBaseFilter,
	controller_evr: w::IMFVideoDisplayControl,
	media_seek:     w::IMediaSeeking,
	media_ctrl:     w::IMediaControl,
}
