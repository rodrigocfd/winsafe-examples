#![cfg_attr(any(), rustfmt::skip)]

use winsafe::{self as w, gui, co, prelude::*};

/// Contents of second tab.
#[derive(Clone)]
pub struct TabContainer2 {
	wnd: gui::TabPage,
	cmb: gui::ComboBox,
}

impl Into<gui::TabPage> for TabContainer2 { // so we can pass TabContainer2 to TabOpts
	fn into(self) -> gui::TabPage {
		self.wnd.clone()
	}
}

impl TabContainer2 {
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static)) -> Self {
		let wnd = gui::TabPage::new( // create the window for tab page 1
			parent,
			gui::TabPageOpts::default(),
		);

		let cmb = gui::ComboBox::new( // create a combobox
			&wnd,
			gui::ComboBoxOpts {
				position: gui::dpi(10, 10),
				items: &[
					"Avocado",
					"Banana",
					"Grape",
					"Orange",
				],
				selected_item: Some(0),
				..Default::default()
			},
		);

		let new_self = Self { wnd, cmb };
		new_self.events();
		new_self
	}

	fn events(&self) {

	}
}
