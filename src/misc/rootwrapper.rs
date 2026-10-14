use sysinfo::System;

#[cfg(unix)]
fn fetch(sys: &mut System) -> Option<String> {
	use crate::util::general::comm;
	use std::env;
	if rustix::process::geteuid().is_root() {
		let ppid = rustix::process::getppid()?.as_raw_pid() as u32;
		let parent_name = comm(sys, ppid);
		let mut target_name = parent_name.as_deref().unwrap_or("");
		let g_name;
		match target_name {
			"bash" | "zsh" | "sh" | "fish" | "ksh" | "tcsh" | "dash" => {
				let pid_type = sysinfo::Pid::from_u32(ppid);
				sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid_type]), true);
				if let Some(parent_proc) = sys.process(pid_type) {
					let gppid = parent_proc.parent().map(|p| p.as_u32()).unwrap_or(0);
					g_name = comm(sys, gppid);
					if let Some(ref name) = g_name {
						target_name = name;
					}
				}
			}
			_ => {}
		}

		let mut wrapper = match target_name {
			"sudo" => "sudo",
			"doas" => "doas",
			"please" => "please",
			"su" => "su",
			"runuser" => "runuser",
			"gosu" => "gosu",
			"asroot" => "asroot",
			"dzdo" => "dzdo",
			_ => "",
		};
		if wrapper.is_empty() {
			if env::var("PKEXEC_UID").is_ok() {
				wrapper = "pkexec";
				Some(wrapper.to_string())
			} else if env::var("SHELL_PROMPT_PREFIX")
				.unwrap_or_default()
				.contains('🦸')
			{
				wrapper = "run0";
				Some(wrapper.to_string())
			} else {
				None
			}
		} else {
			if env::var("PKEXEC_UID").is_ok() {
				Some(format!("{} + pkexec", wrapper))
			} else if env::var("SHELL_PROMPT_PREFIX")
				.unwrap_or_default()
				.contains('🦸')
			{
				Some(format!("{} + run0", wrapper))
			} else {
				Some(wrapper.to_string())
			}
		}
	} else {
		None
	}
}

#[cfg(not(unix))]
fn fetch(_sys: &mut System) -> Option<String> {
	None
}

pub fn line(sys: &mut System) -> Option<String> {
	fetch(sys)
}
