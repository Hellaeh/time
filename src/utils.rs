pub trait Helper: std::fmt::Write {
	#[inline(always)]
	fn write_u8(&mut self, byte: u8) -> std::fmt::Result {
		self.write_str(unsafe { std::str::from_utf8_unchecked(&[byte]) })
	}
}

impl<T: std::fmt::Write> Helper for T {}

// Took inspiration from `chrono` crate
#[macro_export]
macro_rules! write_hundreds {
	($f: ident, $n: expr) => {
		$f.write_u8(b'0' + ($n) as u8 / 10)?;
		$f.write_u8(b'0' + ($n) as u8 % 10)?;
	};
}
