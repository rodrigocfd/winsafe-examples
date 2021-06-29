use std::cell::RefCell;
use std::rc::Rc;
use winsafe::{co, gui};
use winsafe::{HINSTANCE, IdIdi, NMLVKEYDOWN, POINT, SIZE, WinResult};

#[derive(Clone)]
pub struct MyWindow {
	wnd:      gui::WindowMain,
	cmb_view: gui::ComboBox,
	btn_add:  gui::Button,
	my_list:  gui::ListView,
	next_num: Rc<RefCell<u32>>,
}

impl MyWindow {
	pub fn new() -> MyWindow {
		let hinstance = HINSTANCE::GetModuleHandle(None).unwrap();

		let wnd = gui::WindowMain::new(
			gui::WindowMainOpts {
				title: "ListView".to_owned(),
				class_icon: hinstance.LoadIcon(IdIdi::Id(101)).unwrap(),
				size: SIZE::new(320, 200),
				..Default::default()
			},
		);

		let cmb_view = gui::ComboBox::new(
			&wnd,
			gui::ComboBoxOpts {
				position: POINT::new(10, 10),
				width: 120,
				items: vec!["Icons".to_owned(), "Report".to_owned()],
				selected_item: Some(1),
				..Default::default()
			},
		);

		let btn_add = gui::Button::new(
			&wnd,
			gui::ButtonOpts {
				position: POINT::new(180, 10),
				text: "&Add item".to_owned(),
				..Default::default()
			},
		);

		let my_list = gui::ListView::new(
			&wnd,
			gui::ListViewOpts {
				position: POINT::new(10, 40),
				size: SIZE::new(300, 150),
				list_view_style: co::LVS::REPORT | co::LVS::SHOWSELALWAYS,
				columns: vec![
					("Name".to_owned(), 120),
					("Another".to_owned(), 100),
				],
				..Default::default()
			},
		);

		let next_num = Rc::new(RefCell::new(1));

		let new_self = Self { wnd, cmb_view, btn_add, my_list, next_num };
		new_self.events();
		new_self
	}

	pub fn run(&self) -> WinResult<()> {
		self.wnd.run_main(None)
	}

	fn events(&self) {
		self.cmb_view.on().cbn_sel_change({
			let self2 = self.clone();
			move || {
				if let Some(v) = self2.cmb_view.items().selected_text() {
					self2.my_list.set_current_view(
						if v == "Report" {
							co::LV_VIEW::DETAILS
						} else {
							co::LV_VIEW::SMALLICON
						},
					).unwrap();
				}
			}
		});

		self.btn_add.on().bn_clicked({
			let self2 = self.clone();
			move || {
				let mut next_num = self2.next_num.borrow_mut();

				self2.my_list.items()
					.add(&[
						&format!("This is the item {}", *next_num) as &str,
						"Foo",
					], None).unwrap();

				*next_num += 1;
			}
		});

		self.my_list.on().lvn_key_down({
			let my_list = self.my_list.clone();
			move |p: &NMLVKEYDOWN| {
				if p.wVKey == co::VK::DELETE { // user hit DEL key?
					my_list.items().delete_selected().unwrap();
				}
			}
		});
	}
}
