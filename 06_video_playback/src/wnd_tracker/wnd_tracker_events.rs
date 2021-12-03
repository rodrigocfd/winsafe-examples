use defer_lite::defer;
use winsafe::{prelude::*, self as w, co};

use super::MyTracker;

impl MyTracker {
	pub(super) fn events(&self) {
		self.wnd.on().wm_paint({
			let self2 = self.clone();
			move || {
				let has_focus = w::HWND::GetFocus()
					.map_or(false, |h| h == self2.wnd.hwnd());

				let mut ps = w::PAINTSTRUCT::default();
				let hdc = self2.wnd.hwnd().BeginPaint(&mut ps)?;
				defer! { self2.wnd.hwnd().EndPaint(&ps); }

				let color = w::GetSysColor(if has_focus {
					co::COLOR::ACTIVECAPTION
				} else {
					co::COLOR::ACTIVEBORDER
				});

				let hpen = w::HPEN::CreatePen(co::PS::SOLID, 1, color)?;
				defer! { hpen.DeleteObject().unwrap(); }
				let hpen_def = hdc.SelectObjectPen(hpen)?;
				defer! { hdc.SelectObjectPen(hpen_def).unwrap(); }

				let hbrush = w::HBRUSH::CreateSolidBrush(color)?;
				defer! { hbrush.DeleteObject().unwrap(); }
				let hbrush_def = hdc.SelectObjectBrush(hbrush)?;
				defer! { hdc.SelectObjectBrush(hbrush_def).unwrap(); }

				let rc = self2.wnd.hwnd().GetClientRect()?;
				let pos = rc.right as f32 * self2.position_pct.get();
				hdc.Rectangle(w::RECT {
					left: 0,
					top: 0,
					right: pos as _,
					bottom: rc.bottom,
				})?;

				Ok(())
			}
		});

		self.wnd.on().wm_l_button_down({
			let self2 = self.clone();
			move |p| {
				self2.wnd.hwnd().SetFocus();

				if let Some(click_cb) = self2.click_cb.try_borrow()?.as_ref() {
					let rc = self2.wnd.hwnd().GetClientRect()?;
					let pct = p.coords.x as f32 / rc.right as f32;
					click_cb(pct)?;
				}

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
			let space_cb = self.space_cb.clone();
			move |p| {
				let key = co::VK::from(p.char_code as u16);
				if key == co::VK::SPACE {
					if let Some(space_cb) = space_cb.try_borrow()?.as_ref() {
						space_cb()?;
					}
				}
				Ok(())
			}
		});

		self.wnd.on().wm_get_dlg_code({
			let self2 = self.clone();
			move |p| {
				if p.vkey_code == co::VK::LEFT || p.vkey_code == co::VK::RIGHT {
					if let Some(arrows_cb) = self2.arrows_cb.try_borrow()?.as_ref() {
						arrows_cb(p.vkey_code)?;
					}
					Ok(co::DLGC::WANTARROWS)
				} else {
					Ok(co::DLGC::NoValue)
				}
			}
		});
	}
}
