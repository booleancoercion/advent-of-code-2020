use std::cmp::min;

use aoc_runner_derive::*;

#[aoc_generator(day10)]
fn generate(input: &str) -> Vec<u32> {
    let mut vec: Vec<u32> = input.lines()
        .map(|x| u32::from_str_radix(x, 10).unwrap())
        .collect();

    vec.sort();
    vec.insert(0, 0);
    vec.reverse();
    vec.insert(0, vec[0]+3); // our adapter
    vec
}

#[aoc(day10, part1)]
fn solve_part1(input: &[u32]) -> u32 {
    let mut differences = [0u32; 3];
    for s in input.windows(2) {
        differences[(s[0] - s[1] - 1) as usize] += 1;
    }

    differences[0] * differences[2]
}

#[aoc(day10, part2)]
fn solve_part2(input: &[u32]) -> u64 {
    let input: Vec<u64> = input.iter()
        .map(|&x| x as u64)
        .collect();
    
    let mut memory = Vec::new();
    for _ in 0..input.len() {
        memory.push(None)
    }
    arrangements(0, &input, &mut memory)
}

fn arrangements(index: usize, input: &[u64], memory: &mut [Option<u64>]) -> u64 {
    if index >= input.len() {
        return 0;
    } else if index == input.len()-1 {
        return 1;
    } else if let Some(val) = memory[index] {
        return val;
    }

    let mut result = 0;
    for j in index+1..=min(index+3, input.len() - 1) {
        if input[index] - input[j] > 3 {
            break;
        }
        result += arrangements(j, input, memory);
    }
    memory[index] = Some(result);
    result
}