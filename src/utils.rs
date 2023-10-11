// Took inspiration from `chrono` crate
#[macro_export]
macro_rules! write_hundreds {
	($f: ident, $n: expr) => {
		$f.write_char((b'0' + ($n) as u8 / 10) as char)?;
		$f.write_char((b'0' + ($n) as u8 % 10) as char)?;
	};
}
