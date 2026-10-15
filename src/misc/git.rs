use std::fs;
use toml::Table;

type LangFetcher = fn() -> Option<String>;

const LANGUAGES: &[LangFetcher] = &[fetch_rust, fetch_python];

fn fetch() -> Vec<String> {
	if !matches!(fs::exists(".git"), Ok(true)) {
		return Vec::new();
	}
	LANGUAGES.iter().filter_map(|f| f()).collect()
}

fn fetch_rust() -> Option<String> {
	let content = fs::read_to_string("Cargo.toml").ok()?;
	let table = content.parse::<Table>().ok()?;
	let package = table.get("package")?.as_table()?;

	let name = package.get("name")?.as_str()?;
	let version = package.get("version")?.as_str()?;

	Some(format!("{name} v{version} (Rust)"))
}

fn fetch_python() -> Option<String> {
	let content = fs::read_to_string("pyproject.toml").ok()?;
	let table = content.parse::<Table>().ok()?;
	let project = table.get("project")?.as_table()?;
	let name = project.get("name")?.as_str()?;
	let version = project.get("version")?.as_str()?;
	Some(format!("{name} v{version} (Python)"))
}

pub fn line() -> Option<String> {
	fetch().into_iter().next()
}
