use defer_lite::defer;
use winsafe::{prelude::*, self as w, co};

use super::MyVideo;

impl MyVideo {
	pub(super) fn events(&self) {
		self.wnd.on().wm_destroy({
			let self2 = self.clone();
			move || self2.unload() // cleanup
		});

		self.wnd.on().wm_paint({
			let self2 = self.clone();
			move || {
				let mut ps = w::PAINTSTRUCT::default();
				self2.wnd.hwnd().BeginPaint(&mut ps)?;
				defer! { self2.wnd.hwnd().EndPaint(&ps); }

				if let Some(com_objs) = self2.com_objs.try_borrow()?.as_ref() {
					com_objs.controller_evr.RepaintVideo()?;
				}
				Ok(())
			}
		});

		// self.wnd.on().wm_erase_bkgnd(|_| 0);

		self.wnd.on().wm_size({
			let self2 = self.clone();
			move |p| {
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
			}
		});

		self.wnd.on().wm_l_button_down({
			let self2 = self.clone();
			move |_| {
				self2.wnd.hwnd().SetFocus();
				self2.play_pause()?; // play/pause on left click
				Ok(())
			}
		});

		self.wnd.on().wm_r_button_down({
			let wnd = self.wnd.clone();
			move |_| {
				wnd.hwnd().SetFocus();
				Ok(())
			}
		});

		self.wnd.on().wm_key_down({
			let self2 = self.clone();
			move |p| {
				let key = co::VK::from(p.char_code as u16);
				if key == co::VK::SPACE {
					self2.play_pause()?;
				}
				Ok(())
			}
		});

		self.wnd.on().wm_get_dlg_code({
			let self2 = self.clone();
			move |p| {
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
			}
		});
	}
}
