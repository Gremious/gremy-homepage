use super::*;

extern crate test;

use test::Bencher;

#[bench]
pub fn test(b: &mut Bencher) {
    b.iter(|| 1+1)
}
