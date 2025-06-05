#![cfg_attr(any(), rustfmt::skip)]

use winsafe::{self as w, prelude::*};

use super::wnd_decl::MyWindow;

impl MyWindow {
	pub fn events(&self) {

		// There are a lot of events here, for educational purposes.
		// In a real-life application, you're unlikely to have that many events.

		let self2 = self.clone();
		self.wnd.on().wm_create(move |_| {
			self2.lsv.items().add(&["Abc", "def"], None, ())?;
			self2.lsv.items().add(&["Ghi", "jkl"], None, ())?.select(true)?;
			self2.lsv.items().add(&["Zhi", "jkl"], None, ())?;

			self2.sta.parts().get(0).set_text("Status bar")?;

			let r1 = self2.tvw.items().add_root("Root 1", None, ())?;
			r1.add_child("Child 1", None, ())?;
			r1.expand(true)?;
			self2.tvw.items().add_root("Root 2", None, ())?;

			Ok(0)
		});

		let self2 = self.clone();
		self.btn.on().bn_clicked(move || {
			self2.wnd.hwnd().SetWindowText("Button clicked")?;
			self2.pro.set_position(95);
			Ok(())
		});

		let self2 = self.clone();
		self.chk.on().bn_clicked(move || {
			let checked = self2.chk.is_checked();
			self2.wnd.hwnd().SetWindowText(&format!("Status: {checked}"))?;
			Ok(())
		});

		let self2 = self.clone();
		self.cmb.on().cbn_sel_change(move || {
			let sel = self2.cmb.items().selected_text()?.unwrap_or("(none)".to_owned());
			self2.wnd.hwnd().SetWindowText(&format!("Selected: {sel}"))?;
			self2.sta.parts().get(1).set_text(&sel)?;
			Ok(())
		});

		let self2 = self.clone();
		self.dtp.on().dtn_date_time_change(move |p| {
			let text = format!("{}-{}-{}", p.st.wYear, p.st.wMonth, p.st.wDay);
			self2.wnd.hwnd().SetWindowText(&format!("Date and time picker: {text}"))?;
			Ok(())
		});

		let self2 = self.clone();
		self.txt.on().en_change(move || {
			let text = self2.txt.text()?;
			self2.wnd.hwnd().SetWindowText(&format!("Typed: {text}"))?;
			Ok(())
		});

		let self2 = self.clone();
		self.hea.on().hdn_item_click(move |p| {
			let item = self2.hea.items().get(p.iItem as _);
			self2.wnd.hwnd().SetWindowText(&format!("Header: {}", item.text()))?;
			Ok(())
		});

		let self2 = self.clone();
		self.lbl.on().stn_clicked(move || {
			self2.wnd.hwnd().SetWindowText("Label clicked")?;
			Ok(())
		});

		let self2 = self.clone();
		self.lst.on().lbn_sel_change(move || {
			let text = if self2.lst.items().selected_count()? > 0 {
				self2.lst.items()
					.iter_selected()?
					.try_fold("".to_owned(), |mut str, idx_txt| {
						let (_idx, txt) = idx_txt?;
						str.push_str(&format!("{txt}, "));
						w::SysResult::Ok(str)
					})?
			} else {
				"(no selection)".to_owned()
			};
			self2.wnd.hwnd().SetWindowText(&text)?;
			Ok(())
		});

		let self2 = self.clone();
		self.lsv.on().lvn_item_changed(move |_| {
			let text = if self2.lsv.items().selected_count() > 0 {
				self2.lsv.items()
					.iter_selected()
					.try_fold("".to_owned(), |mut str, item| {
						str.push_str(&format!("{}, ", item.text(0)));
						w::SysResult::Ok(str)
					})?
			} else {
				"(no selection)".to_owned()
			};
			self2.wnd.hwnd().SetWindowText(&text)?;
			Ok(())
		});

		let self2 = self.clone();
		self.lsv.header().unwrap().on().hdn_item_click(move |p| {
			let h = self2.lsv.header().unwrap().items().get(p.iItem as _);
			self2.wnd.hwnd().SetWindowText(&format!("Col: {}", h.text()))?;
			Ok(())
		});

		let self2 = self.clone();
		self.mca.on().mcn_sel_change(move |p| {
			let text = format!("Month calendar: {}-{}-{}",
				p.stSelStart.wYear, p.stSelStart.wMonth, p.stSelStart.wDay);
			self2.wnd.hwnd().SetWindowText(&text)?;
			Ok(())
		});

		let self2 = self.clone();
		self.rad.on().bn_clicked(move || {
			let text = self2.rad.selected().unwrap().hwnd().GetWindowText()?;
			self2.wnd.hwnd().SetWindowText(&text)?;
			Ok(())
		});

		let self2 = self.clone();
		self.tra.on().wm_h_scroll(move |_| {
			self2.wnd.hwnd().SetWindowText(&format!("Pos: {}", self2.tra.pos()))?;
			Ok(())
		});

		let self2 = self.clone();
		self.tvw.on().tvn_item_changed(move |_| {
			let text = match self2.tvw.items().iter_selected().next() {
				None => "(none)".to_owned(),
				Some(item) => item.text()?,
			};
			self2.wnd.hwnd().SetWindowText(&format!("Tree: {text}"))?;
			Ok(())
		});

	}
}
