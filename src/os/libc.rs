use std::env::consts::OS;

fn fetch_libc() -> Option<String> {
	match OS {
		"windows" => Some("UCRT".into()),
		"android" => Some("Bionic".into()),
		"netbsd" => Some("NetBSD libc".into()),
		"freebsd" => Some("FreeBSD libc".into()),
		"openbsd" => Some("OpenBSD libc".into()),
		"dragonfly" => Some("DragonflyBSD libc".into()),
		"haiku" => Some("Haiku libroot".into()),
		"redox" => Some("relibc".into()),
		"hurd" => Some("glibc".into()),
		"macos" | "ios" | "tvos" | "visionos" | "watchos" => Some("Darwin".into()),
		"linux" => {
			if cfg!(any(target_env = "gnu")) {
				Some("glibc".into())
			} else if cfg!(any(target_env = "musl", target_env = "ohos")) {
				Some("musl".into())
			} else if cfg!(target_env = "sgx") {
				Some("fortanix-sgx-libc".into())
			} else {
				None
			}
		}
		_ => None,
	}
}

pub fn line() -> Option<String> {
	fetch_libc()
}
