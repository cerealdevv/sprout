use std::fs;
use toml::Table;

fn fetch() -> Vec<String> {
	let mut info_entries: Vec<String> = Vec::new();
	if let Ok(true) = fs::exists(".git")
		&& let Ok(true) = fs::exists("Cargo.toml")
		&& let Ok(content) = fs::read_to_string("Cargo.toml")
		&& let Ok(table) = content.parse::<Table>()
		&& let Some(package) = table.get("package").and_then(|p| p.as_table())
	{
		let name = package.get("name").and_then(|n| n.as_str());
		let version = package.get("version").and_then(|v| v.as_str());

		if let (Some(n), Some(v)) = (name, version) {
			info_entries.push(format!("{} v{} (Rust)", n, v));
		}
	}
	info_entries
}

pub fn line() -> Option<String> {
	fetch().into_iter().next()
}
