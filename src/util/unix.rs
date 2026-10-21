pub fn parent_pid(pid: u32) -> Option<u32> {
	let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
	let end = stat.rfind(')')?;
	let fields: Vec<&str> = stat.get(end + 2..)?.split_whitespace().collect();
	fields.get(1)?.parse().ok()
}

pub fn is_root() -> bool {
	unsafe extern "C" {
		fn geteuid() -> u32;
	}
	unsafe { geteuid() == 0 }
}
