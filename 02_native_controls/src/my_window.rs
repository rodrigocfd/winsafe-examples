use winsafe::{prelude::*, gui, AnyResult};

#[derive(Clone)]
pub struct MyWindow {
	wnd:        gui::WindowMain,
	lbl_name:   gui::Label,
	txt_name:   gui::Edit,
	chk_sleepy: gui::CheckBox,
	cmb_cities: gui::ComboBox,
	rad_seas:   gui::RadioGroup,
	month_cal:  gui::MonthCalendar,
	tree:       gui::TreeView,
	sbar:       gui::StatusBar,
}

impl MyWindow {
	pub fn new() -> Self {
		let wnd = gui::WindowMain::new( // create the container window
			gui::WindowMainOpts {
				title: "Native controls".to_owned(),
				class_icon: gui::Icon::Id(101),
				size: (600, 240),
				..Default::default()
			},
		);

		// From now on, create the child controls.

		// Note that we are creating each control manually, which means we must
		// specify the x,y coordinates by hand. This is very tedious. That's
		// why, when creating many controls, it's easier to use dialog boxes,
		// since we can place the controls visually by using a WYSIWYG resource
		// editor (like Visual Studio).

		let lbl_name = gui::Label::new(
			&wnd,
			gui::LabelOpts {
				position: (20, 23),
				text: "Name".to_owned(),
				..Default::default()
			},
		);

		let txt_name = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (70, 20),
				width: 120,
				..Default::default()
			},
		);

		let chk_sleepy = gui::CheckBox::new(
			&wnd,
			gui::CheckBoxOpts {
				position: (20, 60),
				text: "I am sleepy".to_owned(),
				..Default::default()
			},
		);

		let cmb_cities = gui::ComboBox::new(
			&wnd,
			gui::ComboBoxOpts {
				position: (20, 100),
				width: 140,
				items: vec![ // items to be added right away
					"Avocado".to_owned(),
					"Banana".to_owned(),
					"Grape".to_owned(),
					"Orange".to_owned(),
				],
				selected_item: Some(0), // first item selected initially
				..Default::default()
			},
		);

		let rad_seas = gui::RadioGroup::new(
			&wnd, &[
				gui::RadioButtonOpts { // let's create three radio buttons
					text: "Mediterranean".to_owned(),
					position: (20, 140),
					..Default::default()
				},
				gui::RadioButtonOpts {
					text: "Caribbean".into(),
					position: (20, 160),
					..Default::default()
				},
				gui::RadioButtonOpts {
					text: "Adriatic".into(),
					position: (20, 180),
					..Default::default()
				},
			],
		);

		let month_cal = gui::MonthCalendar::new(
			&wnd,
			gui::MonthCalendarOpts {
				position: (210, 20), // note that the MonthCalendar has a fixed size
				..Default::default()
			},
		);

		let tree = gui::TreeView::new(
			&wnd,
			gui::TreeViewOpts {
				position: (450, 20),
				size: (130, 160),
				..Default::default()
			},
		);

		let sbar = gui::StatusBar::new(
			&wnd,
			&[
				gui::SbPart::Proportional(1),
				gui::SbPart::Fixed(160),
			],
		);

		let new_self = Self {
			wnd,
			lbl_name,
			txt_name,
			chk_sleepy,
			cmb_cities,
			rad_seas,
			month_cal,
			tree,
			sbar,
		};
		new_self.events();
		new_self
	}

	pub fn run(&self) -> AnyResult<i32> {
		self.wnd.run_main(None)
	}

	fn events(&self) {
		let self2 = self.clone();
		self.wnd.on().wm_create(move |_| { // called once, right after the window is created
			self2.sbar.parts().get(0).set_text("This is the status bar");
			self2.sbar.parts().get(1).set_text("Hi");

			self2.tree.items().add_root("First", None, ());
			self2.tree.items().add_root("Second", None, ());
			let third = self2.tree.items().add_root("Third", None, ());
			third.add_child("Child 1", None, ());
			third.add_child("Child 2", None, ());
			third.expand(true);

			self2.txt_name.focus();
			Ok(0)
		});

		let self2 = self.clone();
		self.txt_name.on().en_change(move || { // text changed
			let text = self2.txt_name.text();
			self2.wnd.set_text(&text);
			Ok(())
		});

		let self2 = self.clone();
		self.chk_sleepy.on().bn_clicked(move || { // check changed
			let yes_no = if self2.chk_sleepy.is_checked() { "yes" } else { "no" };
			self2.wnd.set_text(&format!("Sleepy: {}", yes_no.to_owned()));
			Ok(())
		});

		let self2 = self.clone();
		self.cmb_cities.on().cbn_sel_change(move || { // combo item is selected
			if let Some(the_city) = self2.cmb_cities.items().selected_text() {
				self2.wnd.set_text(&the_city);
			}
			Ok(())
		});

		let self2 = self.clone();
		self.rad_seas.on().bn_clicked(move || { // radio item is selected
			if let Some(selected_radio) = self2.rad_seas.checked() {
				let the_sea = selected_radio.hwnd().GetWindowText()?;
				self2.wnd.set_text(&the_sea);
			}
			Ok(())
		});

		let self2 = self.clone();
		self.month_cal.on().mcn_sel_change(move |p| {
			let s = &p.stSelStart;
			self2.wnd.set_text(&format!("{}-{}-{}", s.wYear, s.wMonth, s.wDay));
			Ok(())
		});

		let self2 = self.clone();
		self.tree.on().tvn_sel_changed(move |p| {
			let selected_item = self2.tree.items().get(&p.itemNew.hItem);
			self2.wnd.set_text(&selected_item.text());
			Ok(())
		});
	}
}
