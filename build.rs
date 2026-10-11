fn main() {
	println!("cargo:rerun-if-env-changed=SPROUT_SOURCE");
	let source = std::env::var("SPROUT_SOURCE").unwrap_or_else(|_| {
		let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
		if dir.contains("registry/src") || dir.contains("registry\\src") {
			"cargo".into()
		} else {
			String::new()
		}
	});
	println!("cargo:rustc-env=SPROUT_SOURCE={source}");
}
