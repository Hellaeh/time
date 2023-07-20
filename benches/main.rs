#![feature(test)]

extern crate test;

use hel_time::iso8601utc;

use test::Bencher;

#[bench]
fn test_iso8601utc(b: &mut Bencher) {
	b.iter(iso8601utc)
}
