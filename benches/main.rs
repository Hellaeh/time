#![feature(test)]

extern crate test;

use hel_time::iso8601utc;

use test::Bencher;

#[bench]
fn bench_iso8601utc_string(b: &mut Bencher) {
	b.iter(iso8601utc)
}

#[bench]
fn bench_iso8601utc_raw(b: &mut Bencher) {
	b.iter(hel_time::ISO8601UTC::now)
}

#[bench]
fn bench_iso8601utc_fmt(b: &mut Bencher) {
	b.iter(|| {
		// use std::fmt::Write;

		let mut buf = String::with_capacity(25);

		unsafe { hel_time::ISO8601UTC::now().fmt(&mut buf).unwrap_unchecked() }

		// unsafe { write!(&mut buf, "{}", hel_time::ISO8601UTC::now()).unwrap_unchecked() };
	})
}

#[bench]
#[ignore]
fn bench_std(b: &mut Bencher) {
	b.iter(std::time::SystemTime::now)
}
