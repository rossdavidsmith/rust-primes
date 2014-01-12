#[cfg(test)]
extern mod extra;

#[cfg(test)]
use extra::test::BenchHarness;

use std::iter::range_step;
use std::num;
use std::vec;

pub fn atkin(limit: uint) -> ~[uint] {
    // Return early for values less than 4. 
    if limit <= 2 {
        return ~[]
    }
    if limit <= 3 {
        return ~[2]
    }

    // initialize the sieve
    let mut field = vec::from_elem(limit, false);

    let limit_sqrt = num::sqrt(limit as f32).ceil() as uint;
    let limit_inclusive = (limit - 1) as f32;

    // put in candidate primes: 
    // integers which have an odd number of
    // representations by certain quadratic forms
    for x in range(1, limit_sqrt) {
        let xx = x*x;
        let xx3 = 3*xx;
        let xx4 = 4*xx;

        let max_y_a = num::sqrt(limit_inclusive - num::min(limit_inclusive, xx4 as f32)) as uint;
        let max_y_b = num::sqrt(limit_inclusive - num::min(limit_inclusive, xx3 as f32)) as uint;
        let min_y_c = num::sqrt(num::max(1f32, (xx3 as f32) - limit_inclusive)).ceil() as uint;

        for y in range(1, max_y_a+1) {
            let yy = y*y;
            func_a(field, xx4, yy);
            func_b(field, xx3, yy);
        }

        for y in range(max_y_a+1, max_y_b + 1) {
            let yy = y*y;
            func_b(field, xx3, yy);
        }

        for y in range(min_y_c, x) {
            let yy = y*y;
            func_c(field, xx3, yy);
        }
    }


    // eliminate composites by sieving
    for n in range(5, limit_sqrt) {
        if !field[n] {
            continue;
        }

        // n is prime, omit multiples of its square; this is
        // sufficient because composites which managed to get
        // on the list cannot be square-free
        for k in range_step(n*n, limit, n*n) {
            field[k] = false;
        }
    }

    let mut primes = ~[2, 3];
    for n in range(5, limit) {
        if !field[n] {
            continue;
        }

        primes.push(n);
    }

    primes
}

#[inline]
fn func_a(field: &mut [bool], xx4: uint, yy: uint) {
    let n = xx4 + yy;
    if (n % 12 == 1 || n % 12 == 5) {
        field[n] = !field[n];
    }
}

#[inline]
fn func_b(field: &mut [bool], xx3: uint, yy: uint) {
    let n = xx3 + yy;
    if n % 12 == 7 {
        field[n] = !field[n];
    }
}

#[inline]
fn func_c(field: &mut [bool], xx3: uint, yy: uint) {
    let n = xx3 - yy;
    if n % 12 == 11 {
        field[n] = !field[n];
    }
}

pub fn erat(limit: uint) -> ~[uint] {
    // Return early for values less than 4. 
    if limit <= 2 {
        return ~[]
    }
    if limit <= 3 {
        return ~[2]
    }

    // Create a vector of 'true' values, one for each number up to limit, exclusive.
    let mut field = vec::from_elem(limit, true);

    // Set all even numbers (apart from 2) to false. Strictly, 0 and 1 should be false
    // also, but the loop ignores everything below index 3 so it doesn't matter.
    mark_multiples_false(field, limit, 2);
    mark_multiples_false(field, limit, 3);

    // List of primes found.
    let mut primes = ~[2, 3];
    let mut last_prime = primes[primes.len()-1];

    loop {
        let mut prime_found = false;

        for i in range_step(last_prime+2, limit, 2) {
            if (field[i]) {
                primes.push(i);
                last_prime = i;
                prime_found = true;
                break;
            }
        }

        if !prime_found {
            break;
        }

        // Mark multiples of the new primes as non-prime
        mark_multiples_false(field, limit, last_prime);
    }

    primes
}

fn mark_multiples_false(field: &mut [bool], limit: uint, prime: uint) {
    let step = if prime == 2 { 2 } else { prime * 2 };

    // Mark multiples of the new primes as non-prime
    for m in range_step(prime * prime, limit, step) {
        field[m] = false;
    }
}

pub fn factors(n: uint) -> ~[uint] {
    let primes = atkin(num::sqrt(n as f32) as uint);

    let mut remaining = n;
    let mut factors = ~[];

    loop {
        let mut factor_found = false;
        for &prime in primes.iter() {
            if remaining % prime == 0 {
                factors.push(prime);

                if remaining == prime {
                    return factors
                }
                else {
                    remaining /= prime;
                    factor_found = true;
                    break;
                }
            }
        }

        if !factor_found {
            factors.push(remaining);
            return factors
        }
    }
}

#[test]
fn atkin_first_ten_primes_correct() {
    assert_eq!(~[2, 3, 5, 7, 11, 13, 17, 19, 23, 29], atkin(30));
}

#[test]
fn erat_first_ten_primes_correct() {
    assert_eq!(~[2, 3, 5, 7, 11, 13, 17, 19, 23, 29], erat(30));
}

#[test]
fn factors_test() {
    assert_eq!(~[2, 3, 5], factors(30));
}


#[bench]
fn atkin_bench(bh: &mut BenchHarness) {
    bh.iter(|| {
        atkin(1000000);
    })
}

#[bench]
fn erat_bench(bh: &mut BenchHarness) {
    bh.iter(|| {
        erat(1000000);
    })
}