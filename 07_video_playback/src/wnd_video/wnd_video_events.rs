use winsafe::{prelude::*, self as w, co};

use super::WndVideo;

impl WndVideo {
	pub(super) fn events(&self) {
		let self2 = self.clone();
		self.wnd.on().wm_destroy(move || {
			self2.unload() // cleanup
		});

		let self2 = self.clone();
		self.wnd.on().wm_paint(move || {
			let _hdc = self2.wnd.hwnd().BeginPaint()?;
			if let Some(com_objs) = self2.com_objs.try_borrow()?.as_ref() {
				com_objs.controller_evr.RepaintVideo()?;
			}
			Ok(())
		});

		// self.wnd.on().wm_erase_bkgnd(|_| 0);

		let self2 = self.clone();
		self.wnd.on().wm_size(move |p| {
			if let Some(com_objs) = self2.com_objs.try_borrow()?.as_ref() {
				com_objs.controller_evr.SetVideoPosition(
					None,
					Some(w::RECT {
						left: 0,
						top: 0,
						right: p.client_area.cx, // resize the video to fit
						bottom: p.client_area.cy,
					}),
				)?;
			}
			Ok(())
		});

		let self2 = self.clone();
		self.wnd.on().wm_l_button_down(move |_| {
			self2.wnd.hwnd().SetFocus();
			self2.play_pause()?; // play/pause on left click
			Ok(())
		});

		let wnd = self.wnd.clone();
		self.wnd.on().wm_r_button_down(move |_| {
			wnd.hwnd().SetFocus();
			Ok(())
		});

		let self2 = self.clone();
		self.wnd.on().wm_key_down(move |p| {
			if p.vkey_code == co::VK::SPACE {
				self2.play_pause()?;
			}
			Ok(())
		});

		let self2 = self.clone();
		self.wnd.on().wm_get_dlg_code(move |p| {
			Ok(match p.vkey_code {
				co::VK::LEFT => {
					self2.seek_backwards(10 * 1000)?;
					co::DLGC::WANTARROWS
				},
				co::VK::RIGHT => {
					self2.seek_forward(10 * 1000)?;
					co::DLGC::WANTARROWS
				},
				_ => co::DLGC::NoValue,
			})
		});
	}
}
