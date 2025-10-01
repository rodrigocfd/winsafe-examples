#![cfg_attr(any(), rustfmt::skip)]

use winsafe::{self as w, gui, co, prelude::*};

/// Contents of first tab.
#[derive(Clone)]
pub struct TabContainer1 {
	wnd: gui::TabPage,
	txt: gui::Edit,
	btn: gui::Button,
}

impl Into<gui::TabPage> for TabContainer1 { // so we can pass TabContainer1 to TabOpts
	fn into(self) -> gui::TabPage {
		self.wnd.clone()
	}
}

impl TabContainer1 {
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static)) -> Self {
		let wnd = gui::TabPage::new( // create the window for tab page 1
			parent,
			gui::TabPageOpts::default(),
		);

		let txt = gui::Edit::new( // create a textbox
			&wnd,
			gui::EditOpts {
				position: gui::dpi(20, 20),
				width:    gui::dpi_x(180),
				text:     "You",
				..Default::default()
			},
		);

		let btn = gui::Button::new( // create a button
			&wnd,
			gui::ButtonOpts {
				position: gui::dpi(20, 52),
				text:     "&Hello",
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
				Some(&self2.txt.text()?),
				co::TDCBF::OK,
				w::IconRes::Info,
			)?;
			Ok(())
		});
	}
}
