pub fn line() -> String {
	let source = option_env!("SPROUT_SOURCE").unwrap_or_default();
	if source.is_empty() {
		format!("{} {}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"))
	} else {
		format!(
			"{} {}-{}",
			env!("CARGO_BIN_NAME"),
			env!("CARGO_PKG_VERSION"),
			source
		)
	}
}
