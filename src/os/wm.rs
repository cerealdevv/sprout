use sysinfo::System;

#[cfg(any(
	target_os = "linux",
	target_os = "android",
	all(
		any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"),
		feature = "x11"
	)
))]
use crate::util::general::comm;

const KNOWN_WMS: &[(&str, &str)] = &[
	("kwin", "KWin"),
	("gnome-shell", "Mutter"),
	("gnome shell", "Mutter"),
	("mutter", "Mutter"),
	("cinnamon", "Muffin"),
	("muffin", "Muffin"),
	("marco", "Marco"),
	("xfwm", "Xfwm"),
	("budgie-wm", "Budgie WM"),
	("gala", "Gala"),
	("cosmic-comp", "COSMIC Comp"),
	("enlightenment", "Enlightenment"),
	("compiz", "Compiz"),
	("metacity", "Metacity"),
	("xmonad", "XMonad"),
	("hyprland", "Hyprland"),
	("phoc", "Phoc"),
];

fn strip_wrapper(raw: &str) -> &str {
	let s = raw.strip_prefix('.').unwrap_or(raw);
	s.strip_suffix("-wrapped")
		.or_else(|| s.strip_suffix("-real"))
		.unwrap_or(s)
}

fn pretty_wm(raw: &str) -> String {
	let raw = strip_wrapper(raw);
	let lower = raw.to_lowercase();
	KNOWN_WMS
		.iter()
		.find(|(pat, _)| lower.contains(pat))
		.map(|(_, pretty)| (*pretty).to_string())
		.unwrap_or_else(|| raw.to_owned())
}

// technically android can run wayland via termux
#[cfg(any(target_os = "linux", target_os = "android"))]
fn wayland_wm(sys: &mut System) -> Option<String> {
	use std::os::unix::net::UnixStream;
	use std::path::PathBuf;

	let display = std::env::var_os("WAYLAND_DISPLAY")?;
	let path = if PathBuf::from(&display).is_absolute() {
		PathBuf::from(display)
	} else {
		PathBuf::from(std::env::var_os("XDG_RUNTIME_DIR")?).join(display)
	};

	let stream = UnixStream::connect(path).ok()?;
	let cred = rustix::net::sockopt::socket_peercred(&stream).ok()?;
	comm(sys, cred.pid.as_raw_nonzero().get() as u32)
}

#[cfg(all(
	any(
		target_os = "linux",
		target_os = "android",
		target_os = "freebsd",
		target_os = "openbsd",
		target_os = "netbsd"
	),
	feature = "x11"
))]
fn x11_wm(sys: &mut System) -> Option<String> {
	use x11rb::connection::Connection;
	use x11rb::protocol::xproto::{AtomEnum, ConnectionExt};

	let (conn, screen) = x11rb::connect(None).ok()?;
	let root = conn.setup().roots[screen].root;
	let atom = |name: &str| {
		conn.intern_atom(false, name.as_bytes())
			.ok()?
			.reply()
			.ok()
			.map(|r| r.atom)
	};

	let check = atom("_NET_SUPPORTING_WM_CHECK")?;
	let win = conn
		.get_property(false, root, check, AtomEnum::WINDOW, 0, 1)
		.ok()?
		.reply()
		.ok()?
		.value32()?
		.next()?;

	if let Some(pid_atom) = atom("_NET_WM_PID")
		&& let Some(pid) = conn
			.get_property(false, win, pid_atom, AtomEnum::CARDINAL, 0, 1)
			.ok()
			.and_then(|c| c.reply().ok())
			.and_then(|r| r.value32()?.next())
		&& let Some(name) = comm(sys, pid)
	{
		return Some(name);
	}

	let net_wm_name = atom("_NET_WM_NAME")?;
	let utf8 = atom("UTF8_STRING")?;
	let reply = conn
		.get_property(false, win, net_wm_name, utf8, 0, 256)
		.ok()?
		.reply()
		.ok()?;
	String::from_utf8(reply.value).ok()
}

pub struct Wm {
	pub name: String,
	pub server: Option<&'static str>,
}

pub fn line(sys: &mut System) -> Option<Wm> {
	detect(sys).map(|mut wm| {
		wm.name = pretty_wm(&wm.name);
		wm
	})
}

fn detect(#[allow(unused_variables)] sys: &mut System) -> Option<Wm> {
	#[cfg(target_os = "windows")]
	{
		Some(Wm {
			name: "Desktop Window Manager".into(),
			server: None,
		})
	}

	#[cfg(target_os = "macos")]
	{
		Some(Wm {
			name: "Window Server".into(),
			server: None,
		})
	}

	#[cfg(any(target_os = "linux", target_os = "android"))]
	{
		let wm = wayland_wm(sys).map(|name| Wm {
			name,
			server: Some("Wayland"),
		});

		#[cfg(feature = "x11")]
		let wm = wm.or_else(|| {
			x11_wm(sys).map(|name| Wm {
				name,
				server: Some("X11"),
			})
		});

		#[cfg(target_os = "android")]
		let wm = wm.or_else(|| {
			Some(Wm {
				name: "SurfaceFlinger".into(),
				server: None,
			})
		});

		wm
	}

	#[cfg(all(
		any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"),
		feature = "x11"
	))]
	{
		x11_wm(sys).map(|name| Wm {
			name,
			server: Some("X11"),
		})
	}
	#[cfg(all(
		any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"),
		not(feature = "x11")
	))]
	{
		None
	}
	#[cfg(not(any(
		target_os = "windows",
		target_os = "macos",
		target_os = "linux",
		target_os = "android",
		target_os = "freebsd",
		target_os = "openbsd",
		target_os = "netbsd"
	)))]
	{
		None
	}
}
