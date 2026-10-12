#[cfg(target_os = "android")]
use crate::util::android::getprop;

#[cfg(not(target_os = "android"))]
use sysinfo::System;

#[cfg(target_os = "android")]
pub fn line(_sys: &sysinfo::System) -> String {
	let cpu_manufacturer = getprop("ro.soc.manufacturer").unwrap_or_else(|| "Generic".to_string());
	let cpu_name = getprop("ro.soc.model").unwrap_or_else(|| "CPU".to_string());
	format!("{} {}", cpu_manufacturer, cpu_name)
}

#[cfg(not(target_os = "android"))]
pub fn line(sys: &System) -> String {
	let arch = System::cpu_arch();
	let threads = sys.cpus().len();
	let cores = System::physical_core_count()
		.map(|c| c.to_string())
		.unwrap_or_else(|| "Unknown".to_string());
	let mut model = String::from("Generic CPU");
	if let Some(global_cpu) = sys.cpus().first() {
		model = global_cpu.brand().to_string();
	}
	format!("{} {} {}c/{}t", model, arch, cores, threads)
}
