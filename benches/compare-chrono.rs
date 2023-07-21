#![feature(test)]

extern crate test;

use hel_time::iso8601utc;
use test::Bencher;

#[bench]
#[ignore = "run 'cargo bench -- --ignored' to compare"]
fn compare_iso8601utc(b: &mut Bencher) {
	let this = b
		.bench(|b| {
			b.iter(iso8601utc);
			Ok(())
		})
		.unwrap()
		.unwrap();

	let other = b
		.bench(|b| {
			b.iter(|| {
				let now = std::time::SystemTime::now();
				let now: chrono::DateTime<chrono::Utc> = now.into();
				now.to_rfc3339()
			});

			Ok(())
		})
		.unwrap()
		.unwrap();

	// dbg!(this, other);

	assert!(this.mean < other.mean && this.median < other.median);
}
