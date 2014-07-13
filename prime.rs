#![crate_id = "primes#0.1"]
#![crate_type = "lib"]

use std::iter::range_step;

pub fn atkin(limit: uint) -> Vec<uint> {
    // Return early for values less than 4. 
    if limit <= 2 {
        return Vec::new();
    }
    if limit <= 3 {
        let mut primes = Vec::new();
        primes.push(2);
        return primes
    }

    // if limit < 300 {
    //     return erat(limit);
    // }

    // initialize the sieve
    let mut fixed_field = Vec::from_elem(limit, false);
    let field = fixed_field.as_mut_slice();

    let limit_inclusive = (limit - 1) as f32;

    // put in candidate primes: 
    // integers which have an odd number of
    // representations by certain quadratic forms

    ////////////////////////////////
    // x values from 1 func_a_x_max.
    ////////////////////////////////
    let func_a_x_max = (limit_inclusive / 4f32).sqrt().ceil() as uint;

    for x in range_step(1, func_a_x_max, 6) {
        let xx = x*x;
        let xx3 = 3*xx;
        let xx4 = 4*xx;
        let max_y_a = max_y_for_func_a(limit_inclusive, xx4);
        let max_y_b = max_y_for_func_b(limit_inclusive, xx3);
        let min_y_c = min_y_for_func_c(limit_inclusive, xx3);

        func_a_x_not_divisible_by_3(field, xx4, max_y_a+1);
        func_b_odd_x(field, xx3, max_y_b);
        func_c_odd_x(field, xx3, min_y_c, x);
    }

    for x in range_step(2, func_a_x_max, 6) {
        let xx = x*x;
        let xx3 = 3*xx;
        let xx4 = 4*xx;
        let max_y_a = max_y_for_func_a(limit_inclusive, xx4);
        let min_y_c = min_y_for_func_c(limit_inclusive, xx3);

        func_a_x_not_divisible_by_3(field, xx4, max_y_a+1);
        func_c_even_x(field, xx3, min_y_c, x);
    }

    for x in range_step(3, func_a_x_max, 6) {
        let xx = x*x;
        let xx3 = 3*xx;
        let xx4 = 4*xx;
        let max_y_a = max_y_for_func_a(limit_inclusive, xx4);
        let max_y_b = max_y_for_func_b(limit_inclusive, xx3);
        let min_y_c = min_y_for_func_c(limit_inclusive, xx3);

        func_a_x_divisible_by_3(field, xx4, max_y_a+1);
        func_b_odd_x(field, xx3, max_y_b);
        func_c_odd_x(field, xx3, min_y_c, x);
    }

    for x in range_step(4, func_a_x_max, 6) {
        let xx = x*x;
        let xx3 = 3*xx;
        let xx4 = 4*xx;
        let max_y_a = max_y_for_func_a(limit_inclusive, xx4);
        let min_y_c = min_y_for_func_c(limit_inclusive, xx3);

        func_a_x_not_divisible_by_3(field, xx4, max_y_a+1);
        func_c_even_x(field, xx3, min_y_c, x);
    }

    for x in range_step(5, func_a_x_max, 6) {
        let xx = x*x;
        let xx3 = 3*xx;
        let xx4 = 4*xx;
        let max_y_a = max_y_for_func_a(limit_inclusive, xx4);
        let max_y_b = max_y_for_func_b(limit_inclusive, xx3);
        let min_y_c = min_y_for_func_c(limit_inclusive, xx3);

        func_a_x_not_divisible_by_3(field, xx4, max_y_a+1);
        func_b_odd_x(field, xx3, max_y_b);
        func_c_odd_x(field, xx3, min_y_c, x);
    }

    for x in range_step(6, func_a_x_max, 6) {
        let xx = x*x;
        let xx3 = 3*xx;
        let xx4 = 4*xx;
        let max_y_a = max_y_for_func_a(limit_inclusive, xx4);
        let min_y_c = min_y_for_func_c(limit_inclusive, xx3);

        func_a_x_divisible_by_3(field, xx4, max_y_a+1);
        func_c_even_x(field, xx3, min_y_c, x);
    }


    //////////////////////////////////////////////
    // x values from func_a_x_max to func_b_x_max.
    //////////////////////////////////////////////
    let func_b_x_max = (limit_inclusive / 3f32).sqrt().ceil() as uint;

    for x in range_step(round_to_next_odd(func_a_x_max), func_b_x_max, 2) {
        let xx3 = 3*x*x;
        let max_y_b = max_y_for_func_b(limit_inclusive, xx3);
        let min_y_c = min_y_for_func_c(limit_inclusive, xx3);

        func_b_odd_x(field, xx3, max_y_b);
        func_c_odd_x(field, xx3, min_y_c, x);
    }


    ////////////////////////////////////////////
    // x values from func_b_x_max to limit_sqrt.
    ////////////////////////////////////////////
    let limit_sqrt = (limit as f32).sqrt().ceil() as uint;

    for x in range_step(round_to_next_odd(func_b_x_max), limit_sqrt, 2) {
        let xx3 = 3*x*x;
        let min_y_c = min_y_for_func_c(limit_inclusive, xx3);

        func_c_odd_x(field, xx3, min_y_c, x);
    }

    for x in range_step(round_to_next_even(func_a_x_max), limit_sqrt, 2) {
        let xx3 = 3*x*x;
        let min_y_c = min_y_for_func_c(limit_inclusive, xx3);

        func_c_even_x(field, xx3, min_y_c, x);
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

    let mut primes = Vec::new();
    primes.push(2);
    primes.push(3);
    for n in range(5, limit) {
        if !field[n] {
            continue;
        }

        primes.push(n);
    }

    primes
}

// Rounds up n to the nearest even number.
// Returns n   if n%2 == 0
// Returns n+1 if n%2 == 1
fn round_to_next_even(n: uint) -> uint {
    if n%2 == 0 {n} else {n+1}
}

// Rounds up n to the nearest even number.
// Returns n+1 if n%2 == 0
// Returns n   if n%2 == 1
fn round_to_next_odd(n: uint) -> uint {
    if n%2 == 1 {n} else {n+1}
}

// The largest value of y, where n = 4x^2 + y^2 is less than the limit.
#[inline]
fn max_y_for_func_a(limit_inclusive: f32, xx4: uint) -> uint {
    sqrt(limit_inclusive - min(limit_inclusive, xx4 as f32)) as uint
}

// The largest value of y, where n = 3x^2 + y^2 is less than the limit.
#[inline]
fn max_y_for_func_b(limit_inclusive: f32, xx3: uint) -> uint {
    sqrt(limit_inclusive - min(limit_inclusive, xx3 as f32)) as uint
}

// The smallest value of y, where n = 3x^2 - y^2 is less than the limit.
#[inline]
fn min_y_for_func_c(limit_inclusive: f32, xx3: uint) -> uint {
    sqrt(max(1f32, (xx3 as f32) - limit_inclusive as f32)).ceil() as uint
}

fn sqrt(a: f32) -> f32 {
    a.sqrt()
}
fn min(a: f32, b: f32) -> f32 {
    a.min(b)
}
fn max(a: f32, b: f32) -> f32 {
    a.max(b)
}

// Toggles solutions of n = 4x^2 + y^2.
// n must always satisfy n%12 = 1 or n%12 = 5
#[inline]
fn func_a(field: &mut [bool], xx4: uint, y: uint) {
    let yy = y*y;
    let n = xx4 + yy;
    field[n] = !field[n];
}

// Toggles solutions of n = 3x^2 + y^2.
// n must always satisfy n%12 = 7
#[inline]
fn func_b(field: &mut [bool], xx3: uint, y: uint) {
    let yy = y*y;
    let n = xx3 + yy;
    field[n] = !field[n];
}

// Toggles solutions of n = 3x^2 - y^2.
// n must always satisfy n%12 = 11
#[inline]
fn func_c(field: &mut [bool], xx3: uint, y: uint) {
    let yy = y*y;
    let n = xx3 - yy;
    field[n] = !field[n];
}

// Toggles all solutions of 4x^2 + y^2 where x is not divisible by 3 and y is odd.
// For all x where x%3 != 0, (4xx)%12 = 4
// For all y where y%2 == 1, (yy)%12 = 1 or 9
// Therefore, with the above restrictions, (4xx + yy)%12 will always be either
// (4+1)%12 = 5 or (4+9)%12 = 1
#[inline]
fn func_a_x_not_divisible_by_3(field: &mut [bool], xx4: uint, max_y: uint) {
    for y in range_step(1, max_y, 2) {
        func_a(field, xx4, y);
    }
}

// Toggles all solutions of 4x^2 + y^2 where x is divisible by 3 and y is odd and not
// divisible by three.
// For all x where x%3 == 0, (4xx)%12 = 0
// For all y where y%2 == 1 and y%3 != 0, (yy)%12 = 1
// Therefore, with the above restrictions, (4xx + yy)%12 will always be (0+1)%12 = 1
#[inline]
fn func_a_x_divisible_by_3(field: &mut [bool], xx4: uint, max_y: uint) {
    for y in range_step(1, max_y, 6) {
        func_a(field, xx4, y);
    }
    for y in range_step(5, max_y, 6) {
        func_a(field, xx4, y);
    }
}

// Toggles all solutions of 3x^2 + y^2 where x is odd and y is even and not
// divisible by three.
// For all x where x%2 == 1, (3xx)%12 = 3
// For all y where y%2 == 0 and y%3 != 0, (yy)%12 = 4
// Therefore, with the above restrictions, (3xx + yy)%12 will always be (3+4)%12 = 7
#[inline]
fn func_b_odd_x(field: &mut [bool], xx3: uint, max_y: uint) {
    for y in range_step(2, max_y + 1, 6) {
        func_b(field, xx3, y);
    }

    for y in range_step(4, max_y + 1, 6) {
        func_b(field, xx3, y);
    }
}

// Toggles all solutions of 3x^2 - y^2 where x is even and y is odd and not
// divisible by three.
// For all x where x%2 == 0, (3xx)%12 = 0
// For all y where y%2 == 1 and y%3 != 0, (yy)%12 = 1
// Therefore, with the above restrictions, (3xx - yy)%12 will always be (0-1)%12 = 11
#[inline]
fn func_c_even_x(field: &mut [bool], xx3: uint, min_y: uint, max_y: uint) {
    let next_mod_6_r_1 = min_y + ((7 - (min_y % 6)) % 6);
    let next_mod_6_r_5 = min_y + ((11 - (min_y % 6)) % 6);

    for y in range_step(next_mod_6_r_1, max_y + 1, 6) {
        func_c(field, xx3, y);
    }
    for y in range_step(next_mod_6_r_5, max_y + 1, 6) {
        func_c(field, xx3, y);
    }
}

// Toggles all solutions of 3x^2 - y^2 where x is odd and y is even and not
// divisible by three.
// For all x where x%2 == 1, (3xx)%12 = 3
// For all y where y%2 == 0 and y%3 != 0, (yy)%12 = 4
// Therefore, with the above restrictions, (3xx - yy)%12 will always be (3-4)%12 = 11
#[inline]
fn func_c_odd_x(field: &mut [bool], xx3: uint, min_y: uint, max_y: uint) {
    let next_mod_6_r_2 = min_y + ((8 - (min_y % 6)) % 6);
    let next_mod_6_r_4 = min_y + ((10 - (min_y % 6)) % 6);

    for y in range_step(next_mod_6_r_2, max_y + 1, 6) {
        func_c(field, xx3, y);
    }

    for y in range_step(next_mod_6_r_4, max_y + 1, 6) {
        func_c(field, xx3, y);
    }
}

pub fn erat(limit: uint) -> Vec<uint> {
    // Return early for values less than 4. 
    if limit <= 2 {
        return Vec::new();
    }
    if limit <= 3 {
        let mut primes = Vec::new();
        primes.push(2);
        return primes
    }

    // Create a vector of 'true' values, one for each number up to limit, exclusive.
    let mut fixed_field = Vec::from_elem(limit, true);
    let field = fixed_field.as_mut_slice();

    // Set all even numbers (apart from 2) to false. Strictly, 0 and 1 should be false
    // also, but the loop ignores everything below index 3 so it doesn't matter.
    mark_multiples_false(field, limit, 2);
    mark_multiples_false(field, limit, 3);

    // List of primes found.
    let mut primes = Vec::new();
    primes.push(2);
    primes.push(3);
    let mut last_prime = 3;

    loop {
        let mut prime_found = false;

        for i in range_step(last_prime+2, limit, 2) {
            if field[i] {
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

pub fn factors(n: uint) -> Vec<uint> {
    let primes = atkin((n as f32).sqrt() as uint);

    let mut remaining = n;
    let mut factors = Vec::new();

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