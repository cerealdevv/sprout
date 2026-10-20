#[cfg(feature = "flags")]
mod imp {
	use std::collections::HashSet;
	use std::ffi::OsStr;
	use std::sync::OnceLock;

	#[derive(Debug, Default)]
	pub struct Flags(HashSet<char>);

	impl Flags {
		pub fn parse<I, S>(args: I) -> Self
		where
			I: IntoIterator<Item = S>,
			S: AsRef<OsStr>,
		{
			let mut set = HashSet::new();

			for arg in args {
				let arg = arg.as_ref().to_string_lossy();

				if arg == "--" {
					break;
				}
				if arg.starts_with("--") {
					continue;
				}
				if let Some(shorts) = arg.strip_prefix('-')
					&& !shorts.is_empty()
					&& shorts.chars().all(|c| c.is_ascii_alphanumeric())
				{
					set.extend(shorts.chars());
				}
			}

			Flags(set)
		}

		pub fn is_set(&self, flag: char) -> bool {
			self.0.contains(&flag)
		}
	}
	pub fn flag_set(flag: char) -> bool {
		static FLAGS: OnceLock<Flags> = OnceLock::new();
		FLAGS
			.get_or_init(|| Flags::parse(std::env::args_os().skip(1)))
			.is_set(flag)
	}
	#[cfg(test)]
	mod tests {
		use super::Flags;

		fn set(args: &[&str], flag: char) -> bool {
			Flags::parse(args).is_set(flag)
		}
		#[test]
		fn detects_short_flags() {
			assert!(set(&["-v"], 'v'));
			assert!(set(&["-xv"], 'v'));
			assert!(set(&["-vx"], 'x'));
			assert!(set(&["file", "-v"], 'v'));
			assert!(!set(&["-x"], 'v'));
			assert!(!set(&["-V"], 'v'));
		}

		#[test]
		fn ignores_long_flags() {
			assert!(!set(&["--verbose"], 'v'));
			assert!(!set(&["--verbose=1", "--v"], 'v'));
		}

		#[test]
		fn ignores_nonsense() {
			assert!(!set(&["-", "-x=v", "-é", "v", "verbose"], 'v'));
		}

		#[test]
		fn double_dash_stops_parsing() {
			assert!(!set(&["--", "-v"], 'v'));
			assert!(set(&["-v", "--", "-x"], 'v'));
			assert!(!set(&["-v", "--", "-x"], 'x'));
		}
	}
}

#[cfg(feature = "flags")]
pub use imp::flag_set;

#[cfg(not(feature = "flags"))]
pub fn flag_set(_flag: char) -> bool {
	false
}
