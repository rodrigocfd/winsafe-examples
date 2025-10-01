use std::cell::RefCell;
use std::rc::Rc;
use winsafe::{self as w, co, gui, prelude::*};

use super::{ComObjs, WndVideo};

impl WndVideo {
	#[must_use]
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
				resize_behavior: (gui::Horz::Resize, gui::Vert::Resize),
				// ex_style: gui::WindowControlOpts::default().ex_style | co::WS_EX::CLIENTEDGE,
				..Default::default()
			},
		);

		let com_objs = Rc::new(RefCell::new(None)); // COM objects initially not loaded

		let new_self = Self { wnd, com_objs };
		new_self.events();
		new_self
	}

	pub fn unload(&self) -> w::AnyResult<()> {
		if let Some(com_objs) = self.com_objs.try_borrow()?.as_ref() {
			com_objs.media_ctrl.Stop()?;
		}
		*self.com_objs.try_borrow_mut()? = None; // will drop all COM objects
		Ok(())
	}

	pub fn load(&self, video_path: &str) -> w::AnyResult<()> {
		self.unload()?;

		let graph_builder = w::CoCreateInstance::<w::IGraphBuilder>(
			&co::CLSID::FilterGraph,
			None::<&w::IUnknown>,
			co::CLSCTX::INPROC_SERVER,
		)?;

		let vmr = w::CoCreateInstance::<w::IBaseFilter>(
			&co::CLSID::EnhancedVideoRenderer,
			None::<&w::IUnknown>,
			co::CLSCTX::INPROC_SERVER,
		)?;

		graph_builder.AddFilter(&vmr, "EVR")?;

		let get_svc = vmr.QueryInterface::<w::IMFGetService>()?;

		let controller_evr = get_svc
			.GetService::<w::IMFVideoDisplayControl>(&co::MF_SERVICE::MR_VIDEO_RENDER_SERVICE)?;
		controller_evr.SetVideoWindow(self.wnd.hwnd())?;
		controller_evr.SetAspectRatioMode(co::MFVideoARMode::PreservePicture)?;

		graph_builder.RenderFile(video_path)?;

		let rc_s = self.wnd.hwnd().GetWindowRect()?; // first, in screen coordinates
		let rc_p = self.wnd.hwnd().ScreenToClientRc(rc_s)?; // now, relative to parent
		controller_evr.SetVideoPosition(None, Some(rc_p))?; // set video to fit window

		let media_seek = graph_builder.QueryInterface::<w::IMediaSeeking>()?;

		let media_ctrl = graph_builder.QueryInterface::<w::IMediaControl>()?;
		media_ctrl.Run()?;

		// let mut filter_info = w::FILTER_INFO::default();
		// for filter in graph_builder.EnumFilters()?.iter() {
		// 	let filter = filter?;
		// 	filter.QueryFilterInfo(&mut filter_info)?;
		// 	println!("Filter: {}", filter_info.achName());
		// }

		*self.com_objs.try_borrow_mut()? = Some(
			// finally save the COM objects
			ComObjs {
				graph_builder,
				vmr,
				controller_evr,
				media_seek,
				media_ctrl,
			},
		);
		Ok(())
	}

	pub fn is_running(&self) -> w::AnyResult<bool> {
		Ok(match self.com_objs.try_borrow()?.as_ref() {
			Some(com_ojbs) => com_ojbs.media_ctrl.GetState(None)? == co::FILTER_STATE::Running,
			None => false,
		})
	}

	pub fn play_pause(&self) -> w::AnyResult<()> {
		if let Some(com_objs) = self.com_objs.try_borrow()?.as_ref() {
			if self.is_running()? {
				com_objs.media_ctrl.Pause()?;
			} else {
				com_objs.media_ctrl.Run()?;
			}
		}
		Ok(())
	}

	pub fn pause(&self) -> w::AnyResult<()> {
		if let Some(com_objs) = self.com_objs.try_borrow()?.as_ref() {
			com_objs.media_ctrl.Pause()?;
		}
		Ok(())
	}

	pub fn curpos_duration(&self) -> w::AnyResult<Option<(i64, i64)>> {
		if let Some(com_objs) = self.com_objs.try_borrow()?.as_ref() {
			let cur_pos = com_objs.media_seek.GetCurrentPosition()? / 10_000;
			let duration = com_objs.media_seek.GetDuration()? / 10_000;
			Ok(Some((cur_pos, duration))) // originally in 100 nanoseconds; now in milliseconds
		} else {
			Ok(None)
		}
	}

	pub fn set_pos(&self, ms: i64) -> w::AnyResult<()> {
		if let Some(com_objs) = self.com_objs.try_borrow_mut()?.as_ref() {
			com_objs.media_seek.SetPositions(
				ms * 10_000,
				co::SEEKING_FLAGS::AbsolutePositioning,
				0,
				co::SEEKING_FLAGS::NoPositioning,
			)?;
		}
		Ok(())
	}

	pub fn seek_forward(&self, ms_diff: i64) -> w::AnyResult<()> {
		if let Some((ms_pos, ms_tot)) = self.curpos_duration()? {
			self.set_pos(if ms_pos + ms_diff >= ms_tot {
				ms_tot - 1 // never go beyond max
			} else {
				ms_pos + ms_diff
			})?;
		}
		Ok(())
	}

	pub fn seek_backwards(&self, ms_diff: i64) -> w::AnyResult<()> {
		if let Some((ms_pos, _)) = self.curpos_duration()? {
			self.set_pos(if ms_diff > ms_pos {
				0 // never go before zero
			} else {
				ms_pos - ms_diff
			})?;
		}
		Ok(())
	}
}
