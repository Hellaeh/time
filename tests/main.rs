use hel_time::iso8601utc;

#[test]
fn test_iso8601utc() {
	let time = iso8601utc();
	assert!(time.len() >= 24);
}

#[test]
fn test_iso8601utc_fmt() {
	use std::fmt::Write;

	let mut time = String::new();

	unsafe {
		write!(&mut time, "{}", hel_time::ISO8601UTC::now()).unwrap_unchecked();
	}

	assert!(time.len() >= 24);
}
