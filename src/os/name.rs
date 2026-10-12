use sysinfo::System;

#[cfg(target_os = "android")]
use crate::util::android::getprop;

#[cfg(target_os = "android")]
fn rom_name(name: &str, version_prop: &str) -> String {
	match getprop(version_prop).filter(|v| !v.is_empty()) {
		Some(v) => format!("{} {}", name, v),
		None => name.to_string(),
	}
}

#[cfg(target_os = "android")]
fn rom_fetch() -> Option<String> {
	let display_id = getprop("ro.build.display.id")
		.unwrap_or_default()
		.to_lowercase();
	let build_user = getprop("ro.build.user").unwrap_or_default().to_lowercase();

	if getprop("ro.lineage.version").is_some()
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
	}
}

#[cfg(target_os = "android")]
fn fetch_os() -> (String, String) {
	let osver = System::os_version().unwrap_or_default();
	match rom_fetch() {
		Some(name) => (name, format!("(Android {})", osver)),
		None => ("Android".to_string(), osver),
	}
}

#[cfg(not(target_os = "android"))]
fn fetch_os() -> (String, String) {
	(
		System::name().unwrap_or_else(|| "Linux".to_string()),
		System::os_version().unwrap_or_default(),
	)
}

pub fn line() -> String {
	let (name, ver) = fetch_os();
	let ver = ver.replace("rolling", "");
	format!("{} {}", name, ver).trim().to_string()
}
