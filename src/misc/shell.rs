#[cfg(unix)]
use crate::util::general::comm;
#[cfg(unix)]
use std::env;
#[cfg(unix)]
use sysinfo::System;

#[cfg(unix)]
fn parent_pid(pid: u32) -> Option<u32> {
	let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
	let end = stat.rfind(')')?;
	let fields: Vec<&str> = stat.get(end + 2..)?.split_whitespace().collect();
	fields.get(1)?.parse().ok()
}
// idk if this even compiles for non linux OSes, ill assume UNIX-likes should work..?
#[cfg(unix)]
pub fn line(sys: &mut System) -> Option<String> {
	let mut pid = std::process::id();
	for _ in 0..5 {
		if pid == 1 {
			return None;
		}
		pid = parent_pid(pid)?;
		let name = comm(sys, pid)?;
		if name.ends_with("sh") || name.ends_with("shell") || name == "nu" {
			// deliberate exclusion of lua, python, etc, theyre not shells.
			return Some(name);
		}
	}
	env::var("SHELL").ok()
}

#[cfg(not(unix))]
pub fn line(_sys: &mut System) -> Option<String> {
	None // hey can someone PR and add windows support? thatd be super cool
}
