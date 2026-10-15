mod hardware;
mod misc;
mod os;
mod util;

use sysinfo::System;

fn main() {
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
		("Kernel", Some(os::kernel::line())),
		("Init", os::init::line(&mut sys)),
		("SDK", sdk),
		("Libc", os::libc::line()),
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
