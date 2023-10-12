use crate::write_hundreds;
use crate::DateTime;

#[derive(Debug, Clone, Copy)]
pub struct ISO8601UTC(DateTime);
impl ISO8601UTC {
	#[inline]
	pub fn now() -> Self {
		Self(DateTime::utc())
	}

	/// Will format self into `yyyy-MM-ddTHH:mm:ss.fffZ` string
	/// Is not accepting any format options
	#[inline]
	pub fn fmt(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
		use crate::utils::Helper;

		write_hundreds!(f, self.0.year / 100);
		write_hundreds!(f, self.0.year % 100);
		f.write_u8(b'-')?;

		write_hundreds!(f, self.0.month);
		f.write_u8(b'-')?;

		write_hundreds!(f, self.0.day);
		f.write_u8(b'T')?;

		write_hundreds!(f, self.0.hour);
		f.write_u8(b':')?;

		write_hundreds!(f, self.0.minute);
		f.write_u8(b':')?;

		write_hundreds!(f, self.0.second);
		f.write_u8(b'.')?;

		f.write_u8(b'0' + (self.0.ms / 100) as u8)?;
		write_hundreds!(f, self.0.ms % 100);
		f.write_u8(b'Z')
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
