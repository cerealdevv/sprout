pub fn line() -> String {
	let source = option_env!("SPROUT_SOURCE").unwrap_or_default();
	if source.is_empty() {
		format!("{} v{}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"))
	} else {
		format!(
			"{} v{}-{}",
			env!("CARGO_BIN_NAME"),
			env!("CARGO_PKG_VERSION"),
			source
		)
	}
}
