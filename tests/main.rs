use hel_time::iso8601utc;

#[test]
fn test_iso8601utc() {
	let time = iso8601utc();

	assert!(time.len() >= 24);

	println!("{time}");

	// assert!(false);
}
