use defer_lite::defer;
use winsafe::{prelude::*, self as w, co};

use super::WndTracker;

impl WndTracker {
	pub(super) fn events(&self) {
		let self2 = self.clone();
		self.wnd.on().wm_paint(move || {
			let has_focus = w::HWND::GetFocus()
				.map_or(false, |h| h == *self2.wnd.hwnd());

			let hdc = self2.wnd.hwnd().BeginPaint()?;

			let color = w::GetSysColor(if has_focus {
				co::COLOR::ACTIVECAPTION
			} else {
				co::COLOR::ACTIVEBORDER
			});

			let hpen = w::HPEN::CreatePen(co::PS::SOLID, 1, color)?;
			defer! { hpen.DeleteObject().unwrap(); }
			let hpen_def = hdc.SelectObject(&hpen)?;
			defer! { hdc.SelectObject(&hpen_def).unwrap(); }

			let hbrush = w::HBRUSH::CreateSolidBrush(color)?;
			defer! { hbrush.DeleteObject().unwrap(); }
			let hbrush_def = hdc.SelectObject(&hbrush)?;
			defer! { hdc.SelectObject(&hbrush_def).unwrap(); }

			let rc = self2.wnd.hwnd().GetClientRect()?;
			let pos = rc.right as f32 * self2.position_pct.get();
			hdc.Rectangle(w::RECT {
				left: 0,
				top: 0,
				right: pos as _,
				bottom: rc.bottom,
			})?;

			Ok(())
		});

		let self2 = self.clone();
		self.wnd.on().wm_l_button_down(move |p| {
			self2.wnd.hwnd().SetFocus();

			if let Some(click_cb) = self2.click_cb.try_borrow()?.as_ref() {
				let rc = self2.wnd.hwnd().GetClientRect()?;
				let pct = p.coords.x as f32 / rc.right as f32;
				click_cb(pct)?;
			}

			Ok(())
		});

		let wnd = self.wnd.clone();
		self.wnd.on().wm_r_button_down(move |_| {
			wnd.hwnd().SetFocus();
			Ok(())
		});

		let space_cb = self.space_cb.clone();
		self.wnd.on().wm_key_down(move |p| {
			let key = co::VK::from(p.char_code as u16);
			if key == co::VK::SPACE {
				if let Some(space_cb) = space_cb.try_borrow()?.as_ref() {
					space_cb()?;
				}
			}
			Ok(())
		});

		let self2 = self.clone();
		self.wnd.on().wm_get_dlg_code(move |p| {
			if p.vkey_code == co::VK::LEFT || p.vkey_code == co::VK::RIGHT {
				if let Some(arrows_cb) = self2.arrows_cb.try_borrow()?.as_ref() {
					arrows_cb(p.vkey_code)?;
				}
				Ok(co::DLGC::WANTARROWS)
			} else {
				Ok(co::DLGC::NoValue)
			}
		});
	}
}
