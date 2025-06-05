#![cfg_attr(any(), rustfmt::skip)]

use winsafe::{co, gui};

use super::wnd_decl::MyWindow;

impl MyWindow {
	pub fn new() -> Self {
		let wnd = gui::WindowMain::new( // first, we create the container window
			gui::WindowMainOpts {
				title:      "The controls".to_owned(),
				class_icon: gui::Icon::Id(101),
				size:       gui::dpi(660, 290),
				..Default::default()
			},
		);

		// Now we create the child controls.

		// Note that we are creating each control manually, which means we must
		// specify the x,y coordinates by hand. This is very tedious. That's
		// why, when creating many controls, it's easier to use dialog boxes,
		// since we can place the controls visually by using a WYSIWYG resource
		// editor (like Visual Studio).

		let btn = gui::Button::new(&wnd, gui::ButtonOpts {
			text:            "&Click me".to_owned(),
			position:        gui::dpi(10, 10),
			resize_behavior: (gui::Horz::Repos, gui::Vert::Resize),
			..Default::default()
		});

		let cmb = gui::ComboBox::new(&wnd, gui::ComboBoxOpts {
			position: gui::dpi(110, 12),
			width:    gui::dpi_x(120),
			items:    vec!["banana".to_owned(), "avocado".to_owned(), "pineapple".to_owned()],
			..Default::default()
		});

		let chk = gui::CheckBox::new(&wnd, gui::CheckBoxOpts {
			text:     "C&heck me".to_owned(),
			position: gui::dpi(240, 15),
			..Default::default()
		});

		let dtp = gui::DateTimePicker::new(&wnd, gui::DateTimePickerOpts {
			position: gui::dpi(330, 12),
			..Default::default()
		});

		let txt = gui::Edit::new(&wnd, gui::EditOpts {
			position: gui::dpi(10, 50),
			width:    gui::dpi_x(180),
			..Default::default()
		});

		let hea = gui::Header::new(&wnd, gui::HeaderOpts {
			position: gui::dpi(210, 50),
			width:    gui::dpi_x(180),
			items:    vec![("First".to_owned(), 80), ("Second".to_owned(), 70)],
			..Default::default()
		});

		let lbl = gui::Label::new(&wnd, gui::LabelOpts {
			position: gui::dpi(410, 55),
			text:     "&Label".to_owned(),
			..Default::default()
		});

		let lst = gui::ListBox::new(&wnd, gui::ListBoxOpts {
			position:      gui::dpi(10, 90),
			items:         vec!["First".to_owned(), "Second".to_owned(), "Third".to_owned()],
			control_style: gui::ListBoxOpts::default().control_style | co::LBS::MULTIPLESEL,
			..Default::default()
		});

		let lsv = gui::ListView::new(&wnd, gui::ListViewOpts {
			position:      gui::dpi(140, 90),
			columns:       vec![("First".to_owned(), 50), ("Second".to_owned(), 60)],
			control_style: co::LVS::REPORT | co::LVS::SHOWSELALWAYS,
			..Default::default()
		});

		let mca = gui::MonthCalendar::new(&wnd, gui::MonthCalendarOpts {
			position: gui::dpi(280, 90),
			..Default::default()
		});

		let pro = gui::ProgressBar::new(&wnd, gui::ProgressBarOpts {
			position: gui::dpi(470, 50),
			size:     gui::dpi(160, 23),
			value:    40,
			..Default::default()
		});

		let rad = gui::RadioGroup::new(&wnd, &[
			gui::RadioButtonOpts {
				text:     "First radio".to_owned(),
				position: gui::dpi(15, 210),
				..Default::default()
			},
			gui::RadioButtonOpts {
				text:     "Second radio".to_owned(),
				position: gui::dpi(15, 230),
				selected: true,
				..Default::default()
			},
		]);

		let sta = gui::StatusBar::new(&wnd, &[
			gui::SbPart::Fixed(200),
			gui::SbPart::Proportional(1),
			gui::SbPart::Proportional(1),
		]);

		let tra = gui::Trackbar::new(&wnd, gui::TrackbarOpts {
			position: gui::dpi(140, 220),
			range:    (0, 10),
			value:    4,
			..Default::default()
		});

		let tvw = gui::TreeView::new(&wnd, gui::TreeViewOpts {
			position: gui::dpi(520, 90),
			..Default::default()
		});

		// Create MyWindow object with the container window and all the controls.
		let new_self = Self {
			wnd, btn, cmb, chk, dtp, txt, hea, lbl, lst, lsv, mca, pro, rad, sta, tra, tvw,
		};
		new_self.events(); // attach the events
		new_self           // return MyWindow object
	}
}
