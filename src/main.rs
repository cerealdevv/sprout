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

	let lines = [
		("OS", Some(os::name::line())),
		("Kernel", Some(os::kernel::line())),
		("SDK", sdk),
		("DE", os::desktop::line()),
		("CPU", Some(hardware::cpu::line(&sys))),
		("Prober", Some(misc::prober::line())),
		("Uptime", Some(misc::uptime::line())),
	];

	for (label, value) in lines {
		if let Some(v) = value {
			println!("{:<8}{}", format!("{label}:"), v);
		}
	}
}
