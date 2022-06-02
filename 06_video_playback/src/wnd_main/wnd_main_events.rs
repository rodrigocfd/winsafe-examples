use winsafe::{prelude::*, self as w, co};

use super::{ids, WndMain};

impl WndMain {
	pub(super) fn events(&self) {
		self.wnd.on().wm_init_menu_popup({
			let self2 = self.clone();
			move |_| {
				self2.wnd_video.pause()?; // if a menu is shown, pause video
				Ok(())
			}
		});

		self.wnd.on().wm_command_accel_menu(co::DLGID::CANCEL.into(), {
			let wnd = self.wnd.clone();
			move || {
				wnd.hwnd().SendMessage(w::msg::wm::Close {}); // close on ESC
				Ok(())
			}
		});

		self.wnd.on().wm_command_accel_menu(ids::MNU_FILE_OPEN, {
			let self2 = self.clone();
			move || {
				let fileo = w::CoCreateInstance::<w::IFileOpenDialog>(
					&co::CLSID::FileOpenDialog,
					None,
					co::CLSCTX::INPROC_SERVER,
				)?;

				fileo.SetOptions(
					fileo.GetOptions()?
						| co::FOS::FORCEFILESYSTEM
						| co::FOS::ALLOWMULTISELECT
						| co::FOS::FILEMUSTEXIST,
				)?;

				fileo.SetFileTypes(&[
					("Video files", "*.avi;*.mkv;*.mp4"),
					("AVI video files", "*.avi"),
					("MKV video files", "*.mkv"),
					("MP4 video files", "*.mp4"),
					("All files", "*.*"),
				])?;

				fileo.SetFileTypeIndex(0)?;

				if fileo.Show(self2.wnd.hwnd())? {
					self2.wnd_video.load(
						&fileo.GetResult()?
							.GetDisplayName(co::SIGDN::FILESYSPATH)?,
					)?;

					let mut taskbar = self2.taskbar.try_borrow_mut()?;
					if taskbar.is_none() { // taskbar object not created yet?
						*taskbar = Some(
							w::CoCreateInstance(
								&co::CLSID::TaskbarList,
								None,
								co::CLSCTX::INPROC_SERVER,
							)?,
						);
					}

					self2.wnd.hwnd().KillTimer(1).ok(); // kill any previous timer
					self2.wnd.hwnd().SetTimer(1, 100, None)?; // will fire WM_TIMER each 100 ms
				}
				Ok(())
			}
		});

		self.wnd_tracker.on_click({
			let self2 = self.clone();
			move |pct| {
				if let Some((_, ms_total)) = self2.wnd_video.times()? {
					let ms_pos = pct * ms_total as f32;
					self2.wnd_video.set_pos(ms_pos as _)?;
				}
				Ok(())
			}
		});

		self.wnd_tracker.on_space({
			let self2 = self.clone();
			move || {
				self2.wnd_video.play_pause()?;

				if let Some(taskbar) = self2.taskbar.try_borrow()?.as_ref() {
					taskbar.SetProgressState(self2.wnd.hwnd(),
						if self2.wnd_video.is_running()? { // toggle taskbar green/yellow color
							co::TBPF::NORMAL
						} else {
							co::TBPF::PAUSED
						})?;
				}

				Ok(())
			}
		});

		self.wnd_tracker.on_arrows({
			let wnd_video = self.wnd_video.clone();
			move |key| {
				Ok(match key {
					co::VK::LEFT => wnd_video.seek_backwards(10 * 1000)?,
					co::VK::RIGHT => wnd_video.seek_forward(10 * 1000)?,
					_ => {},
				})
			}
		});

		self.wnd.on().wm_timer(1, { // started when a video is loaded
			let self2 = self.clone();
			move || {
				if let Some((ms_cur, ms_total)) = self2.wnd_video.times()? {
					self2.wnd.hwnd().SetWindowText(
						&format!("{} / {}", ms_to_hms(ms_cur), ms_to_hms(ms_total)),
					)?;

					if let Some(taskbar) = self2.taskbar.try_borrow()?.as_ref() {
						taskbar.SetProgressValue(self2.wnd.hwnd(), ms_cur as _, ms_total as _)?;
					}

					self2.wnd_tracker.set_rendered_pos(ms_cur as f32 / ms_total as f32);
				}
				Ok(())
			}
		});
	}
}

/// Converts milliseconds to formatted h:mm:ss.
fn ms_to_hms(ms_cur: i64) -> String {
	let ms = ms_cur % 1000;
	let secs = ((ms_cur - ms) / 1000) % 60;
	let mins = ((ms_cur - secs * 1000 - ms) / 1000 / 60) % 60;
	let hs = (ms_cur - mins * 1000 * 60 - secs * 1000 - ms) / 1000 / 60 / 60;
	format!("{}:{:02}:{:02}", hs, mins, secs)
}
