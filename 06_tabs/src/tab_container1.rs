use winsafe::{self as w, prelude::*, gui, co};

#[derive(Clone)]
pub struct TabContainer1 {
	wnd: gui::WindowControl,
	txt: gui::Edit,
	btn: gui::Button,
}

impl AsRef<gui::WindowControl> for TabContainer1 { // we must implement AsRef so this window can be used as a tab
	fn as_ref(&self) -> &gui::WindowControl {
		&self.wnd
	}
}

impl TabContainer1 {
	pub fn new(parent: &impl GuiParent) -> Self {
		let wnd = gui::WindowControl::new(
			parent,
			gui::WindowControlOpts {
				ex_style: co::WS_EX::CONTROLPARENT, // so the focus rotation works properly
				..Default::default()
			},
		);

		let txt = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (20, 20),
				width: 180,
				..Default::default()
			},
		);

		let btn = gui::Button::new(
			&wnd,
			gui::ButtonOpts {
				position: (20, 52),
				text: "&Hello".to_owned(),
				..Default::default()
			},
		);

		let new_self = Self { wnd, txt, btn };
		new_self.events();
		new_self
	}

	fn events(&self) {
		let self2 = self.clone();
		self.btn.on().bn_clicked(move || {
			self2.wnd.hwnd().GetParent()?.TaskDialog(
				Some("Hello"),
				None,
				Some(&self2.txt.text()),
				co::TDCBF::OK,
				w::IconRes::Info,
			)?;
			Ok(())
		});
	}
}
