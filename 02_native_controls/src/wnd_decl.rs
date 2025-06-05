#![cfg_attr(any(), rustfmt::skip)]

use winsafe::{self as w, gui};

/// Main application window.
#[derive(Clone)]
pub struct MyWindow {
	pub wnd: gui::WindowMain,
	pub btn: gui::Button,
	pub cmb: gui::ComboBox,
	pub chk: gui::CheckBox,
	pub dtp: gui::DateTimePicker,
	pub txt: gui::Edit,
	pub hea: gui::Header,
	pub lbl: gui::Label,
	pub lst: gui::ListBox,
	pub lsv: gui::ListView,
	pub mca: gui::MonthCalendar,
	pub pro: gui::ProgressBar,
	pub rad: gui::RadioGroup,
	pub sta: gui::StatusBar,
	pub tra: gui::Trackbar,
	pub tvw: gui::TreeView,
}

impl MyWindow {
	pub fn run(&self) -> w::AnyResult<i32> {
		self.wnd.run_main(None)
	}
}
