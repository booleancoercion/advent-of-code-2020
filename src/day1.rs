use aoc_runner_derive::*;
use std::collections::HashMap;

#[aoc_generator(day1)]
fn generate(input: &str) -> Vec<u16> {
    input.lines()
        .map(|x| u16::from_str_radix(x, 10).unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn solve_part1(vec: &[u16]) -> u32 {
    let map: HashMap<u16, u16> = vec
        .iter()
        .map(|x| (*x, 2020u16.checked_sub(*x)))
        .filter(|(_, y)| y.is_some())
        .map(|(x, y)| (y.unwrap(), x))
        .collect();

    for &x in vec {
        match map.get(&x) {
            Some(&y) => return (x as u32)*(y as u32),
            None => continue
        }
    }
    unreachable!()
}

#[aoc(day1, part2)]
fn solve_part2(vec: &[u16]) -> u32 {
    // sorry part 1, gonna have to go O(n^2) in this
    let map: HashMap<u16, u16> = vec
        .iter()
        .map(|x| (*x, 2020u16.checked_sub(*x)))
        .filter(|(_, y)| y.is_some())
        .map(|(x, y)| (y.unwrap(), x))
        .collect();

    let n = vec.len();
    for i in 0..n {
        let x = vec[i];

        for j in i..n {
            let y = vec[j];

            match map.get(&(x+y)) {
                Some(&z) => return (x as u32)*(y as u32)*(z as u32),
                None => continue
            }
        }
    }
    0
}