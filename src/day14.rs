use std::collections::HashMap;

use aoc_runner_derive::*;

use Instruction::*;
#[derive(Copy, Clone, Debug)]
enum Instruction {
    Mask([char; 36]),
    Mem(i64, i64),
}

#[aoc_generator(day14)]
fn generate(input: &str) -> Vec<Instruction> {
    let mut out = Vec::new();

    for line in input.lines() {
        if line.starts_with("mem") {
            let a = line.find(']').unwrap();
            let index = &line[4..a];
            let value = &line[a + 4..];
            out.push(Mem(index.parse().unwrap(), value.parse().unwrap()));
        } else if line.starts_with("mask") {
            out.push(Mask(make_mask_from_slice(&line[7..7 + 36])));
        }
    }

    out
}

#[aoc(day14, part1)]
fn solve_part1(input: &[Instruction]) -> i64 {
    let mut memory: HashMap<i64, i64> = HashMap::new();
    let mut mask = ['X'; 36];

    for inst in input {
        execute(*inst, &mut memory, &mut mask);
    }

    memory.values().sum()
}

fn execute(inst: Instruction, mem: &mut HashMap<i64, i64>, mask: &mut [char; 36]) {
    match inst {
        Mask(charr) => mask.clone_from(&charr),
        Mem(a, b) => {
            mem.insert(a, apply_mask(&mask, b));
        }
    }
}

fn apply_mask(mask: &[char; 36], mut num: i64) -> i64 {
    for i in 0..36 {
        match mask[36 - i - 1] {
            '0' => {
                let i = i as u32;
                num &= 0b1111_1111_1111_1111_1111_1111_1111_1111_1111 - 2i64.pow(i);
            }
            '1' => {
                let i = i as u32;
                num |= 2i64.pow(i);
            }
            _ => continue,
        }
    }

    num
}

fn make_mask_from_slice(slice: &str) -> [char; 36] {
    let char_vec: Vec<char> = slice.chars().collect();
    let mut chars = ['X'; 36];
    chars.clone_from_slice(&char_vec);
    chars
}

#[aoc(day14, part2)]
fn solve_part2(input: &[Instruction]) -> i64 {
    let mut memory: HashMap<i64, i64> = HashMap::new();
    let mut mask = ['0'; 36];

    for inst in input {
        execute_v2(*inst, &mut memory, &mut mask);
    }

    memory.values().sum()
}

fn execute_v2(inst: Instruction, mem: &mut HashMap<i64, i64>, mask: &mut [char; 36]) {
    match inst {
        Mask(charr) => mask.clone_from(&charr),
        Mem(a, b) => {
            for addr in compute_addresses(&mask, a) {
                mem.insert(addr, b);
            }
        }
    }
}

fn compute_addresses(mask: &[char; 36], mut addr: i64) -> Vec<i64> {
    let mut floating = Vec::new();
    for i in 0..36 {
        match mask[36 - i - 1] {
            '1' => {
                let i = i as u32;
                addr |= 2i64.pow(i);
            }
            'X' => {
                let i = i as u32;
                floating.push(i);
                addr |= 2i64.pow(i);
            }
            _ => continue,
        }
    }

    permute_address(addr, &mut floating)
}

fn permute_address(addr: i64, floating: &mut Vec<u32>) -> Vec<i64> {
    let mut output = Vec::new();

    permute_address_int(addr, floating, &mut output);
    output
}

fn permute_address_int(mut addr: i64, floating: &mut Vec<u32>, output: &mut Vec<i64>) {
    let index = match floating.pop() {
        Some(val) => val,
        None => return output.push(addr),
    };

    permute_address_int(addr, floating, output);
    addr &= 0b1111_1111_1111_1111_1111_1111_1111_1111_1111 - 2i64.pow(index);
    permute_address_int(addr, floating, output);

    floating.push(index);
}

#[cfg(test)]
mod tests {
    use super::{apply_mask, make_mask_from_slice};

    #[test]
    fn test_apply_mask() {
        let mask = make_mask_from_slice("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(apply_mask(&mask, 101), 101);
        assert_eq!(apply_mask(&mask, 11), 73);
        assert_eq!(apply_mask(&mask, 0), 64);
    }
}
