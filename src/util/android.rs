use std::{env, path::PathBuf};

pub fn getprop(property: &str) -> Option<String> {
	use std::ffi::{CStr, CString};
	use std::os::raw::{c_char, c_int};

	const PROP_VALUE_MAX: usize = 92;

	unsafe extern "C" {
		fn __system_property_get(name: *const c_char, value: *mut c_char) -> c_int;
	}

	let name = CString::new(property).ok()?;
	let mut buf = [0 as c_char; PROP_VALUE_MAX];

	let len = unsafe { __system_property_get(name.as_ptr(), buf.as_mut_ptr()) };
	if len <= 0 {
		return None;
	}

	let value = unsafe { CStr::from_ptr(buf.as_ptr()) };
	Some(value.to_string_lossy().into_owned())
}

fn find(name: &str) -> Option<PathBuf> {
	let mut dirs: Vec<PathBuf> = env::var_os("PATH")
		.map(|p| env::split_paths(&p).collect())
		.unwrap_or_default();
	dirs.push(PathBuf::from("/data/data/com.termux/files/usr/bin"));
	dirs.push(PathBuf::from("/system/bin"));
	dirs.push(PathBuf::from("/system/xbin"));

	dirs.into_iter().map(|d| d.join(name)).find(|p| p.is_file())
}
