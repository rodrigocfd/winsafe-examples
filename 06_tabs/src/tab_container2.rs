use winsafe::{self as w, prelude::*, gui, co};

#[derive(Clone)]
pub struct TabContainer2 {
	wnd: gui::WindowControl,
	cmb: gui::ComboBox,
}

impl AsRef<gui::WindowControl> for TabContainer2 { // we must implement AsRef so this window can be used as a tab
	fn as_ref(&self) -> &gui::WindowControl {
		&self.wnd
	}
}

impl TabContainer2 {
	pub fn new(parent: &impl GuiParent) -> Self {
		let wnd = gui::WindowControl::new(
			parent,
			gui::WindowControlOpts {
				ex_style: co::WS_EX::CONTROLPARENT, // so the focus rotation works properly
				..Default::default()
			},
		);

		let cmb = gui::ComboBox::new(
			&wnd,
			gui::ComboBoxOpts {
				position: (10, 10),
				items: vec![
					"Avocado".to_owned(),
					"Banana".to_owned(),
					"Grape".to_owned(),
					"Orange".to_owned(),
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
