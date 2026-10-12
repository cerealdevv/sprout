use crate::util::android::getprop;

fn sdk_check(sdk: u32) -> Option<u32> {
	Some(match sdk {
		21 | 22 => 5,
		23 => 6,
		24 | 25 => 7,
		26 | 27 => 8,
		28 => 9,
		29 => 10,
		30 => 11,
		31 | 32 => 12,
		33 => 13,
		34 => 14,
		35 => 15,
		36 => 16,
		37 => 17,
		_ => return None,
	})
}

pub fn line() -> Option<String> {
	let release = getprop("ro.build.version.release")?;
	let sdk: u32 = getprop("ro.build.version.sdk")?.parse().ok()?;
	if getprop("ro.build.version.codename").is_some_and(|c| c != "REL") {
		return Some(sdk.to_string()); // afaik we can't really detect mismatches in preview builds
	}
	let reported: u32 = release.split('.').next()?.parse().ok()?;
	let expected = sdk_check(sdk)?;
	if reported == expected {
		Some(sdk.to_string())
	} else {
		Some(format!("{}, mismatched SDK", sdk))
	}
}
