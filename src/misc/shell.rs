use std::env;

pub fn line() -> Option<String> {
	let shell = env::var("SHELL").ok()?;
	let name = shell.rsplit(['/', '\\']).next()?;
	Some(name.to_string())
}
