use std::collections::HashSet;

use aoc_runner_derive::*;

use Instruction::*;
#[derive(Copy, Clone)]
enum Instruction {
    ACC(i32),
    JMP(i32),
    NOP(i32)
}

#[aoc_generator(day8)]
fn generate(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|x| {
            let mut s = x.split(' ');
            let part1 = s.next().unwrap();
            let part2 = s.next().unwrap();
            
            let num = i32::from_str_radix(part2, 10).unwrap();

            match part1 {
                "acc" => ACC(num),
                "jmp" => JMP(num),
                "nop" => NOP(num),
                _ => unreachable!()
            }
        })
        .collect()
}

struct Helper(bool, i32);

impl std::fmt::Display for Helper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[aoc(day8, part1)]
fn solve_part1(input: &Vec<Instruction>) -> Helper {
    let mut acc = 0;
    let mut i = 0;
    let mut encountered = HashSet::new();

    while i < input.len() {
        if encountered.contains(&i) {
            return Helper(false, acc);
        }
        encountered.insert(i);
        match input[i] {
            ACC(num) => {
                acc += num;
                i += 1;
            },
            NOP(_) => i += 1,
            JMP(num) => i = (i as i32 + num) as usize
        }
    }

    Helper(true, acc)
}

#[aoc(day8, part2)]
fn solve_part2(input: &Vec<Instruction>) -> i32 {
    let mut copy = input.clone();

    for i in 0..copy.len() {
        let prev = copy[i];
        match prev {
            ACC(_) => continue,
            JMP(num) => copy[i] = NOP(num),
            NOP(num) => copy[i] = JMP(num)
        }
        if let Helper(true, acc) = solve_part1(&copy) {
            return acc;
        }
        copy[i] = prev;
    }

    unreachable!()
}