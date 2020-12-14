use std::collections::HashMap;

use aoc_runner_derive::*;

#[aoc_generator(day13, part1)]
fn generate_part1(input: &str) -> (i32, Vec<Option<i32>>) {
    let mut lines = input.lines();
    let depart = i32::from_str_radix(lines.next().unwrap(), 10).unwrap();
    let busses = lines.next().unwrap()
                        .split(',')
                        .map(|s| s.parse().ok())
                        .collect();

    (depart, busses)
}

#[aoc(day13, part1)]
fn solve_part1(input: &(i32, Vec<Option<i32>>)) -> i32 {
    let depart = input.0;
    let busses = &input.1;

    let out = busses.iter()
        .filter_map(|x| *x)
        .map(|x| (x, x - (depart % x)))
        .min_by_key(|x| x.1)
        .unwrap();

    out.0 * out.1
}

#[aoc_generator(day13, part2)]
fn generate_part2(input: &str) -> Vec<(i64, i64)> {
    let busses: Vec<Option<i64>> = input.lines().nth(1).unwrap()
        .split(',')
        .map(|s| s.parse().ok())
        .collect();
    
    let mut equations = Vec::new();

    for (i, bus) in busses.iter().enumerate() {
        match bus {
            Some(val) => {
                let i = i as i64;
                equations.push( ((*val - i).rem_euclid(*val), *val) );
            },
            None => continue
        }
    }

    equations
}

#[aoc(day13, part2)]
fn solve_part2(input: &Vec<(i64, i64)>) -> i64 {

    // Each entry represents the equation   t === val (mod key)
    let prime_equations = {
        let mut map = HashMap::new();

        for eq in input {
            for factor in prime_factors(eq.1) {
                map.insert(factor, eq.0 % factor);
            }
        }

        map
    };

    let n: i64 = prime_equations.iter()
        .map(|x| x.0)
        .product();

    // Solve the system of modular equations
    // https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Existence_(direct_construction)
    let sum: i64 = prime_equations.iter()
        .map(|(ni, ai)| ai*((n/ni) * extended_gcd(n/ni, *ni).0))
        .sum();
    
    sum.rem_euclid(n) // since the solution is not minimal
}

fn prime_factors(num: i64) -> Vec<i64> {
    let mut out = Vec::new();

    for k in 2..=num {
        if num % k == 0 && is_prime(k) {
            out.push(k);
        }
    }

    out
}

fn is_prime(num: i64) -> bool {
    let num = num.abs();

    if num == 2 {
        return true;
    }

    for k in 2..=((num as f64).sqrt().ceil() as usize) {
        if num % (k as i64) == 0 {
            return false;
        }
    }

    true
}

macro_rules! assign_simul {
    (($i1:ident, $i2:ident) = ($e1:expr, $e2:expr)) => {{
        let temp1 = $e1;
        let temp2 = $e2;

        $i1 = temp1;
        $i2 = temp2;
    }};
}

// Finds integers n,m such that n*a + m*b = gcd(a, b)
// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode
fn extended_gcd(a: i64, b: i64) -> (i64, i64) {
    let (mut s, mut old_s) = (0, 1);
    let (mut r, mut old_r) = (b, a);

    while r != 0 {
        let quotient = old_r / r;
        assign_simul!((old_r, r) = (r, old_r - quotient * r));
        assign_simul!((old_s, s) = (s, old_s - quotient * s));
    }

    let bezout_t = if b != 0 {
        (old_r - old_s * a) / b
    } else {
        0
    };

    (old_s, bezout_t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(3));
        assert!(is_prime(19));
        assert!(!is_prime(4));
        assert!(!is_prime(12));
    }
}