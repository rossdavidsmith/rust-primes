extern crate primes;
extern crate test;
use primes::square_multiples::MultiplesOfSquares;
use std::iter::range_step;

/// Slow, but verifiably correct implementation of MultiplesOfSquares
fn square_multiples_under_limit(
    start: uint,
    multiple: uint,
    skip: uint,
    limit: uint) -> Vec<uint> {

    let mut factors = Vec::new();
    for x in range_step(start, limit, skip) {
        let next = multiple*x*x;

        if next >= limit {
            break;
        }

        factors.push(next);
    }

    factors
}

///
#[test]
fn multiples_change_start() {
    for n in range(1u, 100u) {
        let multiples_vec: Vec<uint> =
            MultiplesOfSquares::new(n, 4, 6, 100000).collect();
        let correct_multiples = square_multiples_under_limit(n, 4, 6, 100000);

        assert_eq!(multiples_vec, correct_multiples);
    }
}

#[test]
fn multiples_change_limit() {
    for n in range(1u, 100u) {
        let multiples_vec: Vec<uint> =
            MultiplesOfSquares::new(1, n, 6, 100000).collect();
        let correct_multiples = square_multiples_under_limit(1, n, 6, 100000);

        assert_eq!(multiples_vec, correct_multiples);
    }
}

#[test]
fn multiples_change_skip() {
    for n in range(1u, 100u) {
        let multiples_vec: Vec<uint> =
            MultiplesOfSquares::new(1, 4, n, 100000).collect();
        let correct_multiples = square_multiples_under_limit(1, 4, n, 100000);

        assert_eq!(multiples_vec, correct_multiples);
    }
}

#[test]
fn multiples_change_all() {
    for i in range(1u, 100u) {
        for j in range(1u, 100u) {
            for k in range(1u, 100u) {
                let mut multiples_iter = MultiplesOfSquares::new(i, j, k, 100000);
                let multiples_vec: Vec<uint> = multiples_iter.collect();
                let correct_multiples = square_multiples_under_limit(i, j, k, 100000);

                assert_eq!(multiples_vec, correct_multiples);
            }
        }
    }
}