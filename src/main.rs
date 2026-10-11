use std::fs::File;
use std::io::{BufRead, BufReader};
use sysinfo::System;

fn getprop(property: &str) -> Option<String> {
	let prop_files = [
		"/system/build.prop",
		"/product/build.prop",
		"/vendor/build.prop",
		"/default.prop",
	];
	for path in prop_files {
		if let Ok(file) = File::open(path) {
			let reader = BufReader::new(file);
			for line in reader.lines().map_while(Result::ok) {
				let trimmed = line.trim();
				if trimmed.starts_with('#') || !trimmed.contains('=') {
					continue;
				}
				match trimmed.split_once('=') {
					Some((key, value)) if key.trim() == property => {
						return Some(value.trim().to_string());
					}
					_ => {}
				}
			}
		}
	}
	None
}

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
	let mut osname = System::name().unwrap_or_else(|| "Linux".to_string());
	let mut osver = System::os_version().unwrap_or_default();
	if rawkerneltype == "android" {
		let display_id = getprop("ro.build.display.id")
			.unwrap_or_default()
			.to_lowercase();
		let build_user = getprop("ro.build.user").unwrap_or_default().to_lowercase();
		if getprop("ro.lineage.version").is_some() || getprop("ro.lineage.device").is_some() || display_id.contains("lineage") {
			let lineage_ver = getprop("ro.lineage.version").unwrap_or_default();
			osname = format!("LineageOS {} (Android ", lineage_ver);
			osver = format!("{})", osver);
		} else if build_user == "grapheneos" || display_id.contains("grapheneos") {
			let gos_ver = getprop("ro.build.version.incremental").unwrap_or_default();
			osname = format!("GrapheneOS {} (Android ", gos_ver);
			osver = format!("{})", osver);
		} else if getprop("ro.calyxos.version").is_some() || display_id.contains("calyxos") {
			let calyx_ver = getprop("ro.calyxos.version").unwrap_or_default();
			osname = format!("CalyxOS {} (Android ", calyx_ver);
			osver = format!("{})", osver);
		} else {
			osname = "Android".to_string();
		}
	}
	osver = osver.replace("rolling", "").trim().to_string();
	println!("OS:     {} {}", osname, osver);
	println!("Kernel: {} {}", kerneltype, kernel);
	println!("CPU:    {} {} {}c/{}t", model, arch, cores, threads)
}
