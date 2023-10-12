#![allow(clippy::assertions_on_constants)]

use hel_time::{iso8601utc, utc};

#[test]
#[ignore = "to see output run 'rust t -- --ignored'"]
fn output_iso8601utc() {
	println!("{}", iso8601utc());

	println!("{:?}", utc());

	assert!(false);
}
