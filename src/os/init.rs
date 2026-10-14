use sysinfo::System;

#[cfg(target_os = "android")]
fn fetch_init(_sys: &mut System) -> Option<String> {
	use crate::util::android::find;
	if find("runit").is_some() {
		Some("Android Init + runit".into())
	} else {
		Some("Android Init".into())
	}
}

#[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "dragonfly"))]
fn fetch_init(_sys: &mut System) -> Option<String> {
	Some("BSD Init + rc".into())
}

#[cfg(target_os = "netbsd")]
fn fetch_init(_sys: &mut System) -> Option<String> {
	Some("BSD Init + rc.d".into())
}

#[cfg(any(
	target_os = "macos",
	target_os = "ios",
	target_os = "tvos",
	target_os = "visionos",
	target_os = "watchos"
))]
fn fetch_init(_sys: &mut System) -> Option<String> {
	Some("launchd".into())
}

#[cfg(any(target_os = "illumos", target_os = "solaris"))]
fn fetch_init(_sys: &mut System) -> Option<String> {
	Some("SMF".into())
}

#[cfg(target_os = "windows")]
fn fetch_init(_sys: &mut System) -> Option<String> {
	Some("Session Manager + Service Control Manager".into())
}

#[cfg(target_os = "haiku")]
fn fetch_init(_sys: &mut System) -> Option<String> {
	Some("Launch Daemon + Launch Roster".into())
}

#[cfg(target_os = "linux")]
fn fetch_init(sys: &mut System) -> Option<String> {
	use crate::util::general::comm;
	comm(sys, 1)
}

#[cfg(not(any(
	target_os = "android",
	target_os = "freebsd",
	target_os = "openbsd",
	target_os = "dragonfly",
	target_os = "netbsd",
	target_os = "macos",
	target_os = "ios",
	target_os = "tvos",
	target_os = "visionos",
	target_os = "watchos",
	target_os = "illumos",
	target_os = "solaris",
	target_os = "windows",
	target_os = "haiku",
	target_os = "linux",
)))]
fn fetch_init(_sys: &mut System) -> Option<String> {
	None
}

pub fn line(sys: &mut System) -> Option<String> {
	fetch_init(sys)
}
