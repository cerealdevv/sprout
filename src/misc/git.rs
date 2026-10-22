use std::fs;
use toml::Table;

type LangFetcher = fn() -> Option<String>;

const LANGUAGES: &[LangFetcher] = &[fetch_rust, fetch_python, fetch_aur, fetch_apk, fetch_csharp];

fn fetch() -> Vec<String> {
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

fn fetch_pkgbuild(recipe: &str, rtype: &str) -> Option<String> {
	let content = fs::read_to_string(recipe).ok()?;
	let name = pkgbuild_var(&content, "pkgbase").or_else(|| pkgbuild_var(&content, "pkgname"))?;
	let version = pkgbuild_var(&content, "pkgver")?;
	Some(format!("{name} v{version} ({rtype})"))
}

fn fetch_aur() -> Option<String> {
	fetch_pkgbuild("PKGBUILD", "AUR")
}

fn fetch_apk() -> Option<String> {
	fetch_pkgbuild("APKBUILD", "APK")
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

fn fetch_csharp() -> Option<String> {
	let path = fs::read_dir(".")
		.ok()?
		.filter_map(Result::ok)
		.map(|e| e.path())
		.find(|p| p.extension().is_some_and(|ext| ext == "csproj"))?;
	let name = path.file_stem()?.to_str()?;
	let content = fs::read_to_string(&path).ok()?;
	let version =
		xml_tag(&content, "InformationalVersion").or_else(|| xml_tag(&content, "Version"))?;
	Some(format!("{name} v{version} (C#)"))
}

fn xml_tag(content: &str, tag: &str) -> Option<String> {
	let open = format!("<{tag}>");
	let close = format!("</{tag}>");
	let start = content.find(&open)? + open.len();
	let end = start + content[start..].find(&close)?;
	let value = content[start..end].trim();
	(!value.is_empty() && !value.contains('$')).then(|| value.to_string())
}

pub fn line() -> Option<String> {
	fetch().into_iter().next()
}
