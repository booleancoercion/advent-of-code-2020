use std::collections::HashMap;

use aoc_runner_derive::*;

#[aoc_generator(day15)]
fn generate(input: &str) -> Vec<i64> {
    input.split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

#[aoc(day15, part1)]
fn solve_part1(input: &Vec<i64>) -> i64 {
    find_nth(input, 2020)
}

#[aoc(day15, part2)]
fn solve_part2(input: &Vec<i64>) -> i64 {
    find_nth(input, 30000000)
}

fn find_nth(input: &Vec<i64>, n: usize) -> i64 {
    let mut map: HashMap<i64, usize> = HashMap::new();
    for (i, e) in input.iter().enumerate() {
        map.insert(*e, i);
    }

    let mut last = *input.last().unwrap();
    for i in input.len()..n {
        let last_dist = match map.get(&last) {
            Some(val) => (i - 1) - val,
            None => 0
        };
        map.insert(last, i - 1);

        last = last_dist as i64;
    }
    last
}