mod flags;
mod hardware;
mod misc;
mod os;
mod util;

use flags::flag_set;
use sysinfo::System;
fn main() {
	if flag_set('V') {
		let x11 = cfg!(feature = "x11");
		let verbosity = cfg!(feature = "verbosity");
		let preset = if cfg!(feature = "default") {
			"Default"
		} else if cfg!(feature = "nox11") {
			"No X11"
		} else {
			"Custom"
		};
		println!("Prober:    {}", misc::prober::line());
		println!("Preset:    {preset}");
		println!("X11:       {x11}");
		println!("Verbosity: {verbosity}");
		println!("Flags:     true");
		return;
	}
	let mut sys = System::new();
	sys.refresh_cpu_all();

	#[cfg(target_os = "android")]
	let sdk = os::sdk::line();
	#[cfg(not(target_os = "android"))]
	let sdk: Option<String> = None;
	let de = os::desktop::line();
	let wm = os::wm::line(&mut sys);

	let de = match (de, &wm) {
		(Some(d), Some(w)) if d.eq_ignore_ascii_case(&w.name) => None,
		(d, _) => d,
	};

	let wm = wm.map(|w| match w.server {
		Some(s) => format!("{} ({s})", w.name),
		None => w.name,
	});
	let lines = [
		("OS", Some(os::name::line())),
		#[cfg(feature = "verbosity")]
		("OS Family", flag_set('v').then(os::family::line).flatten()),
		("Kernel", Some(os::kernel::line())),
		("Init", os::init::line(&mut sys)),
		("SDK", sdk),
		("Libc", os::libc::line()),
		("Shell", misc::shell::line(&mut sys)),
		("Terminal", misc::terminal::line(&mut sys)),
		#[cfg(feature = "project")]
		("Project", misc::git::line()),
		("DE", de),
		("WM", wm),
		("CPU", Some(hardware::cpu::line(&sys))),
		("Prober", Some(misc::prober::line())),
		("Uptime", Some(misc::uptime::line())),
		("Root Wrapper", misc::rootwrapper::line(&mut sys)),
	];
	let padding_width = lines
		.iter()
		.filter(|(_, value)| value.is_some())
		.map(|(label, _)| label.len() + 2)
		.max()
		.unwrap_or(0);
	for (label, value) in lines {
		if let Some(v) = value {
			println!(
				"{:<width$}{}",
				format!("{label}:"),
				v,
				width = padding_width
			);
		}
	}
}
