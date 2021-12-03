use std::cell::RefCell;
use std::rc::Rc;
use winsafe::{prelude::*, self as w, co, dshow, gui};

use super::{ComObjs, WndVideo};

impl WndVideo {
	pub fn new(
		parent: &impl Parent,
		ctrl_id: u16,
		position: w::POINT, size: w::SIZE) -> Self
	{
		let wnd = gui::WindowControl::new(
			parent,
			gui::WindowControlOpts {
				ctrl_id,
				position,
				size,
				horz_resize: gui::Horz::Resize,
				vert_resize: gui::Vert::Resize,
				// ex_style: gui::WindowControlOpts::default().ex_style | co::WS_EX::CLIENTEDGE,
				..Default::default()
			},
		);

		let com_objs = Rc::new(RefCell::new(None)); // COM objects initially not loaded

		let new_self = Self { wnd, com_objs };
		new_self.events();
		new_self
	}

	pub fn unload(&self) -> w::ErrResult<()> {
		if let Some(com_objs) = self.com_objs.try_borrow()?.as_ref() {
			com_objs.media_ctrl.Stop()?;
		}
		*self.com_objs.try_borrow_mut()? = None; // will drop all COM objects
		Ok(())
	}

	pub fn load(&self, video_path: &str) -> w::ErrResult<()> {
		self.unload()?;

		let graph_builder = w::CoCreateInstance::<dshow::IGraphBuilder>(
			&dshow::clsid::FilterGraph,
			None,
			co::CLSCTX::INPROC_SERVER,
		)?;

		let vmr = w::CoCreateInstance::<dshow::IBaseFilter>(
			&dshow::clsid::EnhancedVideoRenderer,
			None,
			co::CLSCTX::INPROC_SERVER,
		)?;

		graph_builder.AddFilter(&vmr, "EVR")?;

		let get_svc = vmr.QueryInterface::<dshow::IMFGetService>()?;

		let controller_evr = get_svc.GetService::<dshow::IMFVideoDisplayControl>(
			&dshow::guid::MR_VIDEO_RENDER_SERVICE)?;
		controller_evr.SetVideoWindow(self.wnd.hwnd())?;
		controller_evr.SetAspectRatioMode(dshow::co::MFVideoARMode::PreservePicture)?;

		graph_builder.RenderFile(video_path)?;

		let mut rc = self.wnd.hwnd().GetWindowRect()?;    // screen coordinates
		self.wnd.hwnd().ScreenToClientRc(&mut rc)?;       // now relative to parent
		controller_evr.SetVideoPosition(None, Some(rc))?; // set video to fit window

		let media_seek = graph_builder.QueryInterface::<dshow::IMediaSeeking>()?;

		let media_ctrl = graph_builder.QueryInterface::<dshow::IMediaControl>()?;
		media_ctrl.Run()?;

		*self.com_objs.try_borrow_mut()? = Some( // finally save the COM objects
			ComObjs { graph_builder, vmr, controller_evr, media_seek, media_ctrl },
		);

		Ok(())
	}

	pub fn is_running(&self) -> w::ErrResult<bool> {
		Ok(match self.com_objs.try_borrow()?.as_ref() {
			Some(com_ojbs) => com_ojbs.media_ctrl.GetState(None)? == dshow::co::FILTER_STATE::Running,
			None => false,
		})
	}

	pub fn play_pause(&self) -> w::ErrResult<()> {
		if let Some(com_objs) = self.com_objs.try_borrow()?.as_ref() {
			if self.is_running()? {
				com_objs.media_ctrl.Pause()?;
			} else {
				com_objs.media_ctrl.Run()?;
			}
		}
		Ok(())
	}

	pub fn pause(&self) -> w::ErrResult<()> {
		if let Some(com_objs) = self.com_objs.try_borrow()?.as_ref() {
			com_objs.media_ctrl.Pause()?;
		}
		Ok(())
	}

	pub fn times(&self) -> w::ErrResult<Option<(i64, i64)>> {
		if let Some(com_objs) = self.com_objs.try_borrow()?.as_ref() {
			Ok(Some( // originally in 100 nanoseconds; now in milliseconds
				(com_objs.media_seek.GetCurrentPosition()? / 10_000,
					com_objs.media_seek.GetDuration()? / 10_000),
			))
		} else {
			Ok(None)
		}
	}

	pub fn set_pos(&self, ms: i64) -> w::ErrResult<()> {
		if let Some(com_objs) = self.com_objs.try_borrow_mut()?.as_ref() {
			com_objs.media_seek.SetPositions(
				ms * 10_000, dshow::co::SEEKING_FLAGS::AbsolutePositioning,
				0, dshow::co::SEEKING_FLAGS::NoPositioning,
			)?;
		}
		Ok(())
	}

	pub fn seek_forward(&self, ms_diff: i64) -> w::ErrResult<()> {
		if let Some((ms_pos, ms_tot)) = self.times()? {
			self.set_pos(if ms_pos + ms_diff >= ms_tot {
				ms_tot - 1 // never go beyond max
			} else {
				ms_pos + ms_diff
			})?;
		}
		Ok(())
	}

	pub fn seek_backwards(&self, ms_diff: i64) -> w::ErrResult<()> {
		if let Some((ms_pos, _)) = self.times()? {
			self.set_pos(if ms_diff > ms_pos {
				0 // never go before zero
			} else {
				ms_pos - ms_diff
			})?;
		}
		Ok(())
	}
}
