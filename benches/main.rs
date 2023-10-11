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
#[ignore]
fn bench_std(b: &mut Bencher) {
	b.iter(std::time::SystemTime::now)
}
