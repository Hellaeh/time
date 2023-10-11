use crate::write_hundreds;
use crate::DateTime;

#[derive(Debug, Clone, Copy)]
pub struct ISO8601UTC(DateTime);
impl ISO8601UTC {
	#[inline]
	pub fn now() -> Self {
		Self(DateTime::utc())
	}

	#[inline]
	fn fmt(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
		write_hundreds!(f, 20);
		write_hundreds!(f, self.0.year % 100);
		f.write_char('-')?;

		write_hundreds!(f, self.0.month);
		f.write_char('-')?;

		write_hundreds!(f, self.0.day);
		f.write_char('T')?;

		write_hundreds!(f, self.0.hour);
		f.write_char(':')?;

		write_hundreds!(f, self.0.minute);
		f.write_char(':')?;

		write_hundreds!(f, self.0.second);
		f.write_char('.')?;

		f.write_char((b'0' + (self.0.ms / 100) as u8) as char)?;
		write_hundreds!(f, self.0.ms % 100);
		f.write_char('Z')
	}

	#[inline]
	#[allow(clippy::inherent_to_string_shadow_display)]
	pub fn to_string(&self) -> String {
		let mut res = String::with_capacity(25);
		unsafe { self.fmt(&mut res).unwrap_unchecked() };
		res
	}
}

impl std::fmt::Display for ISO8601UTC {
	#[inline(always)]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		ISO8601UTC::fmt(self, f)
	}
}
