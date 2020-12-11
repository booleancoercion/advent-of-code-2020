use std::collections::HashSet;

use aoc_runner_derive::*;

#[aoc_generator(day9)]
fn generate(input: &str) -> Vec<u64> {
    input.lines()
        .map(|x| u64::from_str_radix(x, 10).unwrap())
        .collect()
}

#[aoc(day9, part1)]
fn solve_part1(input: &[u64]) -> u64 {
    let mut latest_nums: HashSet<u64> = HashSet::new();

    latest_nums.extend(&input[..25]);
    for i in 25..input.len() {
        if !check_sum(&latest_nums, input[i]) {
            return input[i];
        }
        latest_nums.remove(&input[i-25]);
        latest_nums.insert(input[i]);
    }
    unreachable!()
}

fn check_sum(latest: &HashSet<u64>, num: u64) -> bool {
    for &e in latest {
        let diff = match num.checked_sub(e) {
            Some(diff) => diff,
            None => continue
        };
        if latest.contains(&diff) {
            return true;
        }
    }
    false
}

#[aoc(day9, part2)]
fn solve_part2(input: &[u64]) -> u64 {
    let invalid_num = solve_part1(input);
    for i in 0..input.len() {
        let mut sum = input[i];

        for j in i+1..input.len() {
            sum += input[j];
            if sum == invalid_num {
                return *input[i..=j].iter().min().unwrap()
                     + *input[i..=j].iter().max().unwrap();
            }
        }
    }
    unreachable!()
}