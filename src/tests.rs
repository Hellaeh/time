use super::*;

// Tests will be added as necessary
#[test]
fn primal() {
	assert_eq!(
		"2000-03-01T00:00:00.000Z",
		iso8601utc_from_ts(Duration::new(0, 0))
	)
}
