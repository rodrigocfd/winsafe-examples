use std::cell::RefCell;
use std::rc::Rc;
use winsafe::{dshow, gui};

mod wnd_video_events;
mod wnd_video_funcs;

#[derive(Clone)]
pub struct WndVideo {
	wnd:      gui::WindowControl,
	com_objs: Rc<RefCell<Option<ComObjs>>>,
}

struct ComObjs {
	graph_builder:  dshow::IGraphBuilder,
	vmr:            dshow::IBaseFilter,
	controller_evr: dshow::IMFVideoDisplayControl,
	media_seek:     dshow::IMediaSeeking,
	media_ctrl:     dshow::IMediaControl,
}
