use std::fs;
use toml::Table;

type LangFetcher = fn() -> Option<String>;

const LANGUAGES: &[LangFetcher] = &[fetch_rust, fetch_python, fetch_aur];

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

fn fetch_aur() -> Option<String> {
	let content = fs::read_to_string("PKGBUILD").ok()?;
	let name = pkgbuild_var(&content, "pkgbase").or_else(|| pkgbuild_var(&content, "pkgname"))?;
	let version = pkgbuild_var(&content, "pkgver")?;
	Some(format!("{name} v{version} (AUR)"))
}

fn pkgbuild_var(content: &str, key: &str) -> Option<String> {
	content.lines().find_map(|line| {
		let value = line.strip_prefix(key)?.strip_prefix('=')?;
		let value = value.split(" #").next()?.trim();
		let value = match value.strip_prefix('(') {
			Some(inner) => inner.trim_end_matches(')').split_whitespace().next()?,
			None => value,
		};
		let value = value.trim_matches(|c| c == '"' || c == '\'');
		(!value.is_empty()).then(|| value.to_string())
	})
}

pub fn line() -> Option<String> {
	fetch().into_iter().next()
}
