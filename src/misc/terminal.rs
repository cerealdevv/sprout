use crate::util::general::capitalize;
use crate::util::general::comm;
#[cfg(unix)]
use crate::util::unix::parent_pid;
use std::env;
use sysinfo::System;

const SKIP: &[&str] = &[
	"sh", "bash", "zsh", "fish", "dash", "ksh", "mksh", "tcsh", "csh", "nu", "elvish", "xonsh",
	"pwsh", "sudo", "su", "doas", "env", "script", "just", "make", "cargo",
];

const STOP: &[&str] = &[
	"login",
	"sshd",
	"sshd-session",
	"launchd",
	"screen",
	"getty",
	"systemd",
];

fn display(name: &str) -> String {
	match name {
		n if n.starts_with("gnome-terminal") => "GNOME Terminal".into(),
		"wezterm-gui" => "WezTerm".into(),
		"kgx" => "GNOME Console".into(),
		"xfce4-terminal" => "Xfce Terminal".into(),
		"urxvt" => "rxvt-unicode".into(),
		"st" | "foot" | "kitty" => name.into(),
		_ => capitalize(name),
	}
}

#[cfg(unix)]
fn fetch(sys: &mut System) -> Option<String> {
	let mut pid = parent_pid(std::process::id())?;
	for _ in 0..8 {
		if pid <= 1 {
			return None;
		}
		let name = comm(sys, pid)?;
		if name.starts_with("tmux") || STOP.contains(&name.as_str()) {
			return None;
		}
		if !SKIP.contains(&name.as_str()) {
			return Some(display(&name));
		}
		pid = parent_pid(pid)?;
	}
	None
}

#[cfg(not(unix))]
fn fetch(_sys: &mut System) -> Option<String> {
	None
}

fn fallback_fetch() -> Option<String> {
	let term = env::var("TERM").ok()?;
	#[cfg(target_os = "android")]
	let v = |k: &str| env::var(k).ok().filter(|s| !s.is_empty());
	#[cfg(target_os = "android")]
	if let Some(termux_version) = v("TERMUX_VERSION") {
		return Some(format!("Termux {ver}"));
	}
	#[cfg(target_os = "android")]
	if v("PREFIX").is_some_and(|p| p.contains("com.termux")) {
		return Some("Termux".to_string());
	}
	match term.as_str() {
		"" | "dumb" | "unknown" => None,
		"xterm" | "xterm-256color" | "xterm-color" | "screen" | "screen-256color" | "vt100" => None,
		"tmux" | "tmux-256color" => Some("tmux".to_string()),
		"linux" => Some("Linux TTY".to_string()),
		_ => Some(term.strip_prefix("xterm-").unwrap_or(&term).to_string()),
	}
}

pub fn line(sys: &mut System) -> Option<String> {
	fetch(sys).or_else(fallback_fetch)
}
