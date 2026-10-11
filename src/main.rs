use sysinfo::System;

fn main() {
	let rawkerneltype = std::env::consts::OS;
	let kerneltype = match rawkerneltype {
		"windows" => "Windows NT".to_string(),
		"macos" | "ios" | "tvos" | "watchos" | "visionos" => "Darwin".to_string(),
		"freebsd" => "FreeBSD".to_string(),
		"netbsd" => "NetBSD".to_string(),
		"openbsd" => "OpenBSD".to_string(),
		"android" => "Android (Linux)".to_string(),
		"wasi" => "WASI".to_string(), // okay, why?
		"uefi" => "UEFI".to_string(), // okay, how?
		other => {
			let mut chars = other.chars();
			match chars.next() {
				None => "Unknown".to_string(),
				Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
			}
		}
	};
	let kernel = System::kernel_version().unwrap_or_default();
	let mut sys = System::new_all();
	sys.refresh_cpu_all();
	let arch = System::cpu_arch();
	let threads = sys.cpus().len();
	let cores = System::physical_core_count()
		.map(|c| c.to_string())
		.unwrap_or_else(|| "Unknown".to_string());
	let mut model = "Generic CPU";
	if let Some(global_cpu) = sys.cpus().first() {
		model = global_cpu.brand();
	}
	let osname = System::name().unwrap_or_else(|| "Linux".to_string());
	let mut osver = System::os_version().unwrap_or_default();
	osver = osver.replace("rolling", "").trim().to_string();
	println!("OS:     {} {}", osname, osver);
	println!("Kernel: {} {}", kerneltype, kernel);
	println!("CPU:    {} {} {}c/{}t", model, arch, cores, threads)
}
