//! A simple more of a util crate that deals with time
use std::time::SystemTime;

// seconds per ...
const SPM: u64 = 60;
const SPH: u64 = SPM * 60;
const SPD: u64 = SPH * 24;

// we skip 100 and 400
const DP4Y: u64 = 365 * 4 + 1;

// Windows increments time by 100ns intervals
#[cfg(target_os = "windows")]
const MULTIPLIER: u64 = 10_000_000;

#[cfg(not(target_os = "windows"))]
const MULTIPLIER: u64 = 1_000_000_000;

// 01 March 2000
const ANCHOR: SystemTime = unsafe {
	// TODO should handle this better ?
	// if there's any changes in the future transmute should catch it before compilation
	std::mem::transmute(
		std::mem::transmute::<SystemTime, u64>(SystemTime::UNIX_EPOCH)
			+ 946684800 * MULTIPLIER
			+ SPD * 60 * MULTIPLIER,
	)
};

const ANCHOR_YEAR: u64 = 2000;

// March is 1st month here
const DM: [u64; 12] = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29];

// Took inspiration from `chrono` crate
macro_rules! write_hundreds {
	($w: ident, $n: expr) => {
		$w.push((b'0' + ($n) as u8 / 10) as char);
		$w.push((b'0' + ($n) as u8 % 10) as char);
	};
}

/// Returns a [`String`] formatted as a ISO8601 in UTC
///
/// # Example
/// ```
/// use hel_time::iso8601utc;
///
/// let datetime: String = iso8601utc();
/// println!("{datetime}");
/// ```
// Inspiration https://git.musl-libc.org/cgit/musl/tree/src/time/__secs_to_tm.c
#[inline]
pub fn iso8601utc() -> String {
	let Ok(since_epoch) = SystemTime::now().duration_since(ANCHOR) else {
		return String::new();
	};

	let mut rem_secs = since_epoch.as_secs();

	let mut rem_days = rem_secs / SPD;
	rem_secs -= rem_days * SPD;

	let leaps = rem_days / DP4Y;
	rem_days -= leaps * DP4Y;

	let mut years = rem_days / 365;
	rem_days -= years * 365;

	let mut month = 0u8;
	for d in DM {
		if d > rem_days {
			break;
		}

		rem_days -= d;
		month += 1;
	}

	if month >= 10 {
		month -= 12;
		years += 1;
	}

	let year = ANCHOR_YEAR + years + leaps * 4;
	let month = month + 3;
	let day = rem_days + 1;

	let hour = rem_secs / SPH;
	let minute = rem_secs / SPM % SPM;
	let second = rem_secs % 60;

	let millis = since_epoch.subsec_millis();

	// format is slower by 5x on my machine
	// format!("{year}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{millis:03}Z")

	let mut res = String::with_capacity(25);

	write_hundreds!(res, year / 100);
	write_hundreds!(res, year % 100);
	res.push('-');

	write_hundreds!(res, month);
	res.push('-');

	write_hundreds!(res, day);
	res.push('T');

	write_hundreds!(res, hour);
	res.push(':');

	write_hundreds!(res, minute);
	res.push(':');

	write_hundreds!(res, second);
	res.push('.');

	res.push((b'0' + (millis / 100) as u8) as char);
	write_hundreds!(res, millis / 10);
	res.push('Z');

	res
}
