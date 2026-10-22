use std::env;
pub fn line() -> Option<String> {
	let user = env::var("USER").ok()?;
	let host = env::var("HOST")
		.or_else(|_| env::var("HOSTNAME"))
		.ok()
		.or_else(sysinfo::System::host_name)?; // env vars are cheaper than calling sysinfo
	Some(format!("{user}@{host}"))
}
