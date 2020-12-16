use std::collections::HashSet;
use std::iter::FromIterator;

use aoc_runner_derive::*;

struct Rule {
    a1: u16,
    b1: u16,
    a2: u16,
    b2: u16
}

impl Rule {
    fn new(ranges: &str) -> Rule {
        let mut split_or = ranges.split(" or ");
        let first_range = split_or.next().unwrap().trim();
        let second_range = split_or.next().unwrap().trim();

        let (a1, b1) = parse_range(first_range);
        let (a2, b2) = parse_range(second_range);

        Rule {
            a1,
            b1,
            a2,
            b2
        }
    }

    fn applies(&self, val: u16) -> bool {
        (self.a1 <= val && val <= self.b1) || (self.a2 <= val && val <= self.b2)
    }
}

type Ticket = Vec<u16>;
type Input = (Vec<Rule>, Ticket, Vec<Ticket>);

#[aoc_generator(day16)]
fn generate(input: &str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let mut rules = vec![];
    let mut lines = input.lines();

    for line in lines.by_ref() {
        if line.len() == 0 {
            break;
        }

        let mut split_colon = line.split(':');
        let rule = Rule::new(split_colon.nth(1).unwrap());
        rules.push(rule);
    }

    let my_ticket = parse_ticket(lines.nth(1).unwrap());

    lines.nth(1).unwrap();
    let mut tickets = vec![];
    for line in lines {
        tickets.push(parse_ticket(line));
    }

    (rules, my_ticket, tickets)
}

fn parse_range(range: &str) -> (u16, u16) {
    let mut split = range.split('-');
    let first = split.next().unwrap().parse().unwrap();
    let second = split.next().unwrap().parse().unwrap();

    (first, second)
}

fn parse_ticket(ticket: &str) -> Ticket {
    ticket.split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}


#[aoc(day16, part1)]
fn solve_part1(input: &Input) -> u16 {
    let tickets = &input.2;

    tickets.into_iter()
        .map(|x| invalid_ticket_values(x, &input.0))
        .sum()
}

fn invalid_ticket_values(ticket: &Ticket, rules: &[Rule]) -> u16 {
    ticket.into_iter()
        .filter(|x| !is_valid_value(**x, rules))
        .sum()
}

fn is_valid_value(val: u16, rules: &[Rule]) -> bool {
    rules.into_iter()
        .any(|rule| rule.applies(val))
}

#[aoc(day16, part2)]
fn solve_part2(input: &Input) -> u64 {
    let valid_tickets: Vec<Ticket> = input.2
        .iter().cloned()
        .filter(|ticket| is_valid_ticket(ticket, &input.0))
        .collect();
    
    let rules = &input.0;
    
    let mut rule_positions: Vec<HashSet<usize>> = vec![];

    for rule in rules {
        let mut flags: HashSet<usize> = HashSet::from_iter(0..rules.len());
        for ticket in &valid_tickets {
            let inner_flags: HashSet<usize> = HashSet::from_iter(
                ticket.into_iter()
                    .enumerate()
                    .filter(|(_, val)| rule.applies(**val))
                    .map(|(i, _)| i)
            );

            flags.retain(|x| inner_flags.contains(x));
        }

        rule_positions.push(flags);
    }

    // at this point, rule_positions contains a list of possible positions
    // for each rule.

    let rule_positions = find_rules_permutation(rules, &rule_positions);
    let my_ticket = &input.1;

    rule_positions.iter()
        .take(6)
        .map(|rule_pos| my_ticket[*rule_pos] as u64)
        .product()
}

fn is_valid_ticket(ticket: &Ticket, rules: &[Rule]) -> bool {
    ticket.into_iter()
        .all(|&val| is_valid_value(val, rules))
}

fn find_rules_permutation(rules: &[Rule], positions: &Vec<HashSet<usize>>) -> Vec<usize> {
    let mut perm: Vec<usize> = vec![];
    find_rules_permutation_inner(rules, positions, &mut perm, 0);
    assert_eq!(perm.len(), rules.len());
    perm
}

fn find_rules_permutation_inner(rules: &[Rule], positions: &Vec<HashSet<usize>>, perm: &mut Vec<usize>, i: usize) -> bool {
    if i == rules.len() {
        return true;
    }

    for pos in &positions[i] {
        if perm.contains(pos) {
            continue;
        }

        perm.push(*pos);
        let ret = find_rules_permutation_inner(rules, positions, perm, i+1);
        if ret {
            return true;
        }
        perm.pop();
    }

    false
}

#[allow(dead_code)]
fn count_legal_permutations(rules: &[Rule], positions: &Vec<HashSet<usize>>) -> usize {
    let mut perm = vec![];
    count_legal_permutations_inner(rules, positions, &mut perm, 0)
}

#[allow(dead_code)]
fn count_legal_permutations_inner(rules: &[Rule], positions: &Vec<HashSet<usize>>, perm: &mut Vec<usize>, i: usize) -> usize {
    if i == rules.len() {
        return 1;
    }

    let mut count = 0;

    for pos in &positions[i] {
        if perm.contains(pos) {
            continue;
        }

        perm.push(*pos);
        count += count_legal_permutations_inner(rules, positions, perm, i+1);
        perm.pop();
    }

    count
}