// pls make WM/DE detection destinct later on

#[cfg(target_os = "windows")]
fn de() -> Option<String> {
	Some("Windows Explorer".to_string())
}

#[cfg(target_os = "macos")]
fn de() -> Option<String> {
	Some("Aqua".to_string())
}

#[cfg(target_os = "ios")]
fn de() -> Option<String> {
	Some("SpringBoard".to_string())
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "ios")))]
fn de() -> Option<String> {
	use crate::util::general::capitalize;
	use std::env;
	let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
	if desktop.is_empty() {
		return None;
	}
	let server = capitalize(&env::var("XDG_SESSION_TYPE").unwrap_or_default());
	if server.is_empty() {
		Some(desktop)
	} else {
		Some(format!("{} ({})", desktop, server))
	}
}

pub fn line() -> Option<String> {
	de()
}
