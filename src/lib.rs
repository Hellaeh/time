//! A simple more of a util crate that deals with time
pub use iso8601::ISO8601UTC;

use std::time::SystemTime;

use consts::*;

/// Simple struct that holds Date and Time in UTC
#[derive(Debug, Clone, Copy)]
pub struct DateTime {
	pub year: u16,
	pub month: u8,
	pub day: u16,

	pub hour: u8,
	pub minute: u8,
	pub second: u8,

	pub ms: u16,
}

impl DateTime {
	// Inspiration - https://git.musl-libc.org/cgit/musl/tree/src/time/__secs_to_tm.c
	#[inline]
	pub fn utc() -> Self {
		let since_anchor = SystemTime::now().duration_since(ANCHOR).unwrap_or_default();

		let mut rem_secs = since_anchor.as_secs();

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

		let year = (ANCHOR_YEAR + years + leaps * 4) as u16;
		let month = month + 3;
		let day = rem_days as u16 + 1;

		let hour = (rem_secs / SPH) as u8;
		let minute = (rem_secs / SPM % SPM) as u8;
		let second = (rem_secs % 60) as u8;

		let ms = since_anchor.subsec_millis() as u16;

		Self {
			year,
			month,
			day,

			hour,
			minute,
			second,

			ms,
		}
	}
}

/// Will return a [`String`] in `yyyy-MM-ddTHH:mm:ss.fffZ` format
///
/// # Example
/// ```
/// use hel_time::iso8601utc;
/// println!("{}", iso8601utc());
/// ```
#[inline(always)]
pub fn iso8601utc() -> String {
	iso8601::ISO8601UTC::now().to_string()
}

/// Will return raw [`DateTime`]
#[inline(always)]
pub fn utc() -> DateTime {
	DateTime::utc()
}

mod consts;
mod iso8601;
mod utils;
