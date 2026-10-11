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
		if getprop("ro.lineage.version").is_some()
			|| getprop("ro.lineage.device").is_some()
			|| display_id.contains("lineage")
		{
			let lineage_ver = getprop("ro.lineage.version").unwrap_or_default();
			osname = format!("LineageOS {}", lineage_ver);
			osver = format!("(Android {})", osver);
		} else if build_user == "grapheneos" || display_id.contains("grapheneos") {
			let gos_ver = getprop("ro.build.version.incremental").unwrap_or_default();
			osname = format!("GrapheneOS {}", gos_ver);
			osver = format!("(Android {})", osver);
		} else if getprop("ro.calyxos.version").is_some() || display_id.contains("calyxos") {
			let calyx_ver = getprop("ro.calyxos.version").unwrap_or_default();
			osname = format!("CalyxOS {}", calyx_ver);
			osver = format!("(Android {})", osver);
		} else {
			osname = "Android".to_string();
		}
	}
	osver = osver.replace("rolling", "").trim().to_string();
	println!("OS:     {} {}", osname, osver);
	println!("Kernel: {} {}", kerneltype, kernel);
	println!("CPU:    {} {} {}c/{}t", model, arch, cores, threads)
}
