use std::time::SystemTime;

// seconds per ...
pub const SPM: u64 = 60;
pub const SPH: u64 = SPM * 60;
pub const SPD: u64 = SPH * 24;

// we skip 100 and 400
pub const DP4Y: u64 = 365 * 4 + 1;

#[cfg(target_os = "windows")]
type Local = u64;
#[cfg(not(target_os = "windows"))]
type Local = u128;

// Windows increments time by 100ns intervals
#[cfg(target_os = "windows")]
const MULTIPLIER: Local = 10_000_000;
#[cfg(not(target_os = "windows"))]
const MULTIPLIER: Local = 1_000_000_000;

// 01 March 2000
pub const ANCHOR: SystemTime = unsafe {
	// TODO should handle this better ?
	// if there's any changes in the future, transmute should catch it before compilation
	std::mem::transmute(
		std::mem::transmute::<SystemTime, Local>(SystemTime::UNIX_EPOCH)
			+ 946684800 * MULTIPLIER
			+ SPD as Local * 60 * MULTIPLIER,
	)
};

pub const ANCHOR_YEAR: u64 = 2000;

// March is 0 month
pub const DM: [u64; 12] = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29];
