pub fn line() -> Option<String> {
	#[cfg(target_os = "windows")]
	{
		Some("Windows NT".into())
	}
	#[cfg(target_os = "cygwin")]
	{
		Some("Windows NT (UNIX-like)".into())
	}
	#[cfg(any(
		target_os = "linux",
		target_os = "android",
		target_os = "redox",
		target_os = "hurd",
		target_os = "freebsd",
		target_os = "netbsd",
		target_os = "openbsd",
		target_os = "dragonfly",
		target_os = "macos",
		target_os = "ios",
		target_os = "visionos",
		target_os = "tvos",
		target_os = "watchos",
	))]
	{
		Some("UNIX-like".into())
	}
	#[cfg(any(target_os = "aix", target_os = "solaris", target_os = "illumos"))]
	{
		Some("UNIX".into())
	}
	#[cfg(not(any(
		target_os = "linux",
		target_os = "android",
		target_os = "redox",
		target_os = "hurd",
		target_os = "freebsd",
		target_os = "netbsd",
		target_os = "openbsd",
		target_os = "dragonfly",
		target_os = "macos",
		target_os = "ios",
		target_os = "visionos",
		target_os = "tvos",
		target_os = "watchos",
		target_os = "aix",
		target_os = "solaris",
		target_os = "illumos",
		target_os = "windows",
		target_os = "cygwin",
	)))]
	None
}
