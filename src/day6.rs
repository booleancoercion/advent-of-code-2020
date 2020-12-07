use aoc_runner_derive::*;
use std::collections::HashSet;

type Person = HashSet<char>;
type Group = Vec<Person>;

#[aoc_generator(day6)]
fn generate(input: &str) -> Vec<Group> {
    let mut output = Vec::new();
    let mut current_group = Group::new();

    for line in input.lines() {
        if line.len() == 0 {
            output.push(current_group);
            current_group = Group::new();
            continue;
        }
        let mut current_person = Person::new();
        for c in line.chars() {
            current_person.insert(c);
        }
        current_group.push(current_person);
    }
    if current_group.len() != 0 {
        output.push(current_group);
    }

    output
}

#[aoc(day6, part1)]
fn solve_part1(input: &[Group]) -> usize {
    let mut count = 0;

    for g in input {
        let mut questions: HashSet<char> = HashSet::new();
        for p in g {
            questions.extend(p);
        }
        count += questions.len();
    }
    count
}

#[aoc(day6, part2)]
fn solve_part2(input: &[Group]) -> usize {
    let mut count = 0;

    for g in input {
        let mut first = g[0].clone();

        for i in 1..g.len() {
            first.retain(|c| g[i].contains(c));
        }
        count += first.len();
    }
    count
}