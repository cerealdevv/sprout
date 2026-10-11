use sysinfo::System;

#[cfg(target_os = "android")]
fn getprop(property: &str) -> Option<String> {
	use std::ffi::{CStr, CString};
	use std::os::raw::{c_char, c_int};

	const PROP_VALUE_MAX: usize = 92;

	unsafe extern "C" {
		fn __system_property_get(name: *const c_char, value: *mut c_char) -> c_int;
	}

	let name = CString::new(property).ok()?;
	let mut buf = [0 as c_char; PROP_VALUE_MAX];

	let len = unsafe { __system_property_get(name.as_ptr(), buf.as_mut_ptr()) };
	if len <= 0 {
		return None;
	}

	let value = unsafe { CStr::from_ptr(buf.as_ptr()) };
	Some(value.to_string_lossy().into_owned())
}

#[cfg(not(target_os = "android"))]
fn getprop(_property: &str) -> Option<String> {
	None
}

fn rom_name(name: &str, version_prop: &str) -> String {
	match getprop(version_prop).filter(|v| !v.is_empty()) {
		Some(v) => format!("{} {}", name, v),
		None => name.to_string(),
	}
}

fn format_uptime(secs: u64) -> String {
	fn unit(n: u64, name: &str) -> String {
		if n == 1 {
			format!("{} {}", n, name)
		} else {
			format!("{} {}s", n, name)
		}
	}
	if secs < 60 {
		return unit(secs, "sec");
	}
	let days = secs / 86400;
	let hours = secs % 86400 / 3600;
	let mins = secs % 3600 / 60;
	[(days, "day"), (hours, "hour"), (mins, "min")]
		.iter()
		.filter(|(n, _)| *n > 0)
		.map(|(n, name)| unit(*n, name))
		.collect::<Vec<_>>()
		.join(", ")
}

fn sdk_check(sdk: u32) -> Option<u32> {
	Some(match sdk {
		21 | 22 => 5,
		23 => 6,
		24 | 25 => 7,
		26 | 27 => 8,
		28 => 9,
		29 => 10,
		30 => 11,
		31 | 32 => 12,
		33 => 13,
		34 => 14,
		35 => 15,
		36 => 16,
		37 => 17,
		_ => return None,
	})
}

fn android_version_mismatch() -> Option<String> {
	let release = getprop("ro.build.version.release")?;
	let sdk: u32 = getprop("ro.build.version.sdk")?.parse().ok()?;
	if getprop("ro.build.version.codename").is_some_and(|c| c != "REL") {
		return Some(sdk.to_string()); // afaik we can't really detect mismatches in preview builds
	}
	let reported: u32 = release.split('.').next()?.parse().ok()?;
	let expected = sdk_check(sdk)?;
	if reported == expected {
		Some(sdk.to_string())
	} else {
		Some(format!("{}, mismatched SDK", sdk))
	}
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
	let mut model = String::from("Generic CPU");
	if let Some(global_cpu) = sys.cpus().first() {
		model = global_cpu.brand().to_string();
	}
	let mut osname = System::name().unwrap_or_else(|| "Linux".to_string());
	let mut osver = System::os_version().unwrap_or_default();
	if rawkerneltype == "android" {
		let cpu_manufacturer =
			getprop("ro.soc.manufacturer").unwrap_or_else(|| "Generic".to_string());
		let cpu_name = getprop("ro.soc.model").unwrap_or_else(|| "CPU".to_string());
		model = format!("{} {}", cpu_manufacturer, cpu_name);
		let display_id = getprop("ro.build.display.id")
			.unwrap_or_default()
			.to_lowercase();
		let build_user = getprop("ro.build.user").unwrap_or_default().to_lowercase();
		let rom = if getprop("ro.lineage.version").is_some()
			|| getprop("ro.lineage.device").is_some()
			|| display_id.contains("lineage")
		{
			Some(rom_name("LineageOS", "ro.lineage.version"))
		} else if build_user == "grapheneos" || display_id.contains("grapheneos") {
			Some(rom_name("GrapheneOS", "ro.build.version.incremental"))
		} else if getprop("ro.calyxos.version").is_some() || display_id.contains("calyxos") {
			Some(rom_name("CalyxOS", "ro.calyxos.version"))
		} else {
			None
		};

		match rom {
			Some(name) => {
				osname = name;
				osver = format!("(Android {})", osver);
			}
			None => osname = "Android".to_string(),
		}
	}
	osver = osver.replace("rolling", "").trim().to_string();
	let uptime = format_uptime(System::uptime());
	println!("OS:     {} {}", osname, osver);
	println!("Kernel: {} {}", kerneltype, kernel);
	if rawkerneltype == "android"
		&& let Some(retsdk) = android_version_mismatch()
	{
		println!("SDK:    {}", retsdk)
	}
	println!("CPU:    {} {} {}c/{}t", model, arch, cores, threads);
	println!(
		"Prober: {} {}",
		env!("CARGO_BIN_NAME"),
		env!("CARGO_PKG_VERSION")
	);
	println!("Uptime: {}", uptime);
}
