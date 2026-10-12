fn format_uptime(secs: u64) -> String {
	fn unit(n: u64, name: &str) -> String {
		if n == 1 {
			format!("{} {}", n, name)
		} else {
			format!("{} {}s", n, name)
		}
	}
	if secs < 60 {
		return unit(secs, "sec");
	}
	let days = secs / 86400;
	let hours = secs % 86400 / 3600;
	let mins = secs % 3600 / 60;
	[(days, "day"), (hours, "hour"), (mins, "min")]
		.iter()
		.filter(|(n, _)| *n > 0)
		.map(|(n, name)| unit(*n, name))
		.collect::<Vec<_>>()
		.join(", ")
}

pub fn line() -> String {
	format_uptime(sysinfo::System::uptime())
}
