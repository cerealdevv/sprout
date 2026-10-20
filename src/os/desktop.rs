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
	let desktop = std::env::var("XDG_CURRENT_DESKTOP")
		.ok()?
		.strip_suffix(":wlroots")
		.unwrap_or_default()
		.to_string();
	(!desktop.is_empty()).then_some(desktop)
}

pub fn line() -> Option<String> {
	de()
}
