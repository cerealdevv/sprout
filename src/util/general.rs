use sysinfo::{Pid, ProcessesToUpdate, System};

pub fn capitalize(s: &str) -> String {
	let mut chars = s.chars();
	match chars.next() {
		Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
		None => String::new(),
	}
}

pub fn comm(sys: &mut System, pid: u32) -> Option<String> {
	let pid = Pid::from_u32(pid);
	sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
	sys.process(pid)
		.map(|p| p.name().to_string_lossy().into_owned())
}
