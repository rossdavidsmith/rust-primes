extern crate primes;
extern crate test;
use test::Bencher;
use primes::{atkin, erat, factors};

#[test]
fn atkin_first_ten_primes_correct() {
    assert_eq!(vec!(2, 3, 5, 7, 11, 13, 17, 19, 23, 29), atkin(30));
}

#[test]
fn erat_first_ten_primes_correct() {
    assert_eq!(vec!(2, 3, 5, 7, 11, 13, 17, 19, 23, 29), erat(30));
}

#[test]
fn compare_erat_and_atkin() {
    for n in range(0, 1000) {
        let n = n as uint;
        assert_eq!(atkin(n), erat(n));
    }
}

#[test]
fn factors_test() {
    assert_eq!(vec!(2, 3, 5), factors(30));
}

#[bench]
fn atkin_bench_1000000(bh: &mut Bencher) {
    bh.iter(|| {
        atkin(1000000);
    })
}

#[bench]
fn erat_bench_1000000(bh: &mut Bencher) {
    bh.iter(|| {
        erat(1000000);
    })
}

#[bench]
fn atkin_bench_100000(bh: &mut Bencher) {
    bh.iter(|| {
        atkin(100000);
    })
}

#[bench]
fn erat_bench_100000(bh: &mut Bencher) {
    bh.iter(|| {
        erat(100000);
    })
}

#[bench]
fn atkin_bench_10000(bh: &mut Bencher) {
    bh.iter(|| {
        atkin(10000);
    })
}

#[bench]
fn erat_bench_10000(bh: &mut Bencher) {
    bh.iter(|| {
        erat(10000);
    })
}

#[bench]
fn atkin_bench_1000(bh: &mut Bencher) {
    bh.iter(|| {
        atkin(1000);
    })
}

#[bench]
fn erat_bench_1000(bh: &mut Bencher) {
    bh.iter(|| {
        erat(1000);
    })
}

#[bench]
fn atkin_bench_100(bh: &mut Bencher) {
    bh.iter(|| {
        atkin(100);
    })
}

#[bench]
fn erat_bench_100(bh: &mut Bencher) {
    bh.iter(|| {
        erat(100);
    })
}