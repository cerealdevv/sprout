use crate::util::general::capitalize;

pub fn line() -> String {
	let kerneltype = match std::env::consts::OS {
		"windows" => "Windows NT".to_string(),
		"macos" | "ios" | "tvos" | "watchos" | "visionos" => "Darwin".to_string(),
		"freebsd" => "FreeBSD".to_string(),
		"netbsd" => "NetBSD".to_string(),
		"openbsd" => "OpenBSD".to_string(),
		"android" => "Android (Linux)".to_string(),
		"wasi" => "WASI".to_string(), // okay, why?
		"uefi" => "UEFI".to_string(), // okay, how?
		other => {
			let s = capitalize(other);
			if s.is_empty() {
				"Unknown".to_string()
			} else {
				s
			}
		}
	};
	let kernelversion = sysinfo::System::kernel_version().unwrap_or_default();
	format!("{} {}", kerneltype, kernelversion)
}
