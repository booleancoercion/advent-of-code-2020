use std::collections::HashMap;

use aoc_runner_derive::*;

#[derive(Debug)]
struct Bag {
    description: String,
    contains: Vec<(usize, String)>,
}

impl Bag {
    fn parse(from: &str) -> Bag {
        let mut s = from.split(" contain ");
        let part1 = s.next().unwrap();
        let part2 = s.next().unwrap();

        let desc = {
            let mut part1_spaces = part1.split(' ');
            format!(
                "{} {}",
                part1_spaces.next().unwrap(),
                part1_spaces.next().unwrap()
            )
        };

        let mut contains = Vec::new();
        for bag in part2.split(", ") {
            let mut spaces = bag.split(' ');
            let num = match usize::from_str_radix(spaces.next().unwrap(), 10) {
                Ok(val) => val,
                Err(_) => break,
            };
            let desc = format!("{} {}", spaces.next().unwrap(), spaces.next().unwrap());

            contains.push((num, desc));
        }

        Bag {
            description: desc,
            contains,
        }
    }
}

type Bags = HashMap<String, Bag>;

#[aoc_generator(day7)]
fn generate(input: &str) -> Bags {
    let mut bags = HashMap::new();

    for line in input.lines() {
        let bag = Bag::parse(line);
        bags.insert(bag.description.clone(), bag);
    }

    bags
}

#[aoc(day7, part1)]
fn solve_part1(input: &Bags) -> usize {
    let mut results: HashMap<String, bool> = HashMap::new();
    let mut count = 0;

    for bag in input.values() {
        if contains_shiny_gold(bag, input, &mut results) {
            count += 1;
        }
    }

    count - 1 // to account for the extra counting of shiny gold (the function returns true on the bag itself)
}

fn contains_shiny_gold(bag: &Bag, input: &Bags, results: &mut HashMap<String, bool>) -> bool {
    if let Some(&result) = results.get(&bag.description) {
        return result;
    } else if bag.description == "shiny gold" {
        return true;
    }

    for (_, inner) in &bag.contains {
        if contains_shiny_gold(input.get(inner).unwrap(), input, results) {
            results.insert(bag.description.clone(), true);
            return true;
        }
    }
    results.insert(bag.description.clone(), false);
    false
}

#[aoc(day7, part2)]
fn solve_part2(input: &Bags) -> usize {
    let mut results: HashMap<String, usize> = HashMap::new();
    count_inner_bags(input.get("shiny gold").unwrap(), input, &mut results)
}

fn count_inner_bags(bag: &Bag, input: &Bags, results: &mut HashMap<String, usize>) -> usize {
    if let Some(&result) = results.get(&bag.description) {
        return result;
    }

    let res = bag
        .contains
        .iter()
        .map(|(num, desc)| num * (1 + count_inner_bags(input.get(desc).unwrap(), input, results)))
        .sum();

    results.insert(bag.description.clone(), res);
    res
}
