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

#[derive(Clone)]
struct IndexSet {
    size: usize,
    inner: Vec<bool>,
    len: usize
}

impl IndexSet {
    fn new(size: usize, initial: bool) -> IndexSet {
        IndexSet {
            size,
            inner: vec![initial; size],
            len: if initial { size } else { 0 }
        }
    }

    fn contains(&self, val: usize) -> bool {
        val < self.size && self.inner[val]
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item=usize> + 'a {
        self.inner
            .iter()
            .enumerate()
            .filter(|(_, b)| **b)
            .map(|(i, _)| i)
    }

    fn insert(&mut self, val: usize) {
        if val < self.size && !self.inner[val] {
            self.inner[val] = true;
            self.len += 1;
        }
    }

    fn remove(&mut self, val: usize) {
        if val < self.size && self.inner[val] {
            self.inner[val] = false;
            self.len -= 1;
        }
    }

    fn retain<T: FnMut(usize) -> bool>(&mut self, mut predicate: T) {
        for i in 0..self.size {
            if self.inner[i] && !predicate(i) && self.inner[i] {
                self.inner[i] = false;
                self.len -= 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }
}

impl std::fmt::Debug for IndexSet {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_list().entries(self.iter()).finish()
    }
}

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
    
    let mut rule_positions: Vec<IndexSet> = vec![];

    for rule in rules {
        let mut flags = IndexSet::new(rules.len(), true);
        for ticket in &valid_tickets {
            flags.retain(|i| rule.applies(ticket[i]))
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

fn find_rules_permutation(rules: &[Rule], positions: &Vec<IndexSet>) -> Vec<usize> {
    let mut new_positions: Vec<_> = positions.into_iter()
        .cloned()
        .enumerate()
        .collect();
    new_positions.sort_unstable_by_key(|x| x.1.len());

    let mut perm = vec![];
    let mut encountered = IndexSet::new(rules.len(), false);

    find_rules_permutation_inner(rules, &new_positions, &mut perm, &mut encountered);
    assert_eq!(perm.len(), rules.len());
    perm.sort_unstable_by_key(|x| x.0);
    perm.iter()
        .map(|x| x.1)
        .collect()
}

fn find_rules_permutation_inner(rules: &[Rule], positions: &Vec<(usize, IndexSet)>,
perm: &mut Vec<(usize, usize)>, enc: &mut IndexSet) -> bool {
    let i = perm.len();
    if i == rules.len() {
        return true;
    }
    
    for pos in positions[i].1.iter() {
        if enc.contains(pos) {
            continue;
        }

        perm.push((positions[i].0, pos));
        enc.insert(pos);
        let ret = find_rules_permutation_inner(rules, positions, perm, enc);
        if ret {
            return true;
        }
        enc.remove(pos);
        perm.pop();
    }

    false
}

#[allow(dead_code)]
fn count_legal_permutations(rules: &[Rule], positions: &Vec<IndexSet>) -> usize {
    let mut perm = vec![];
    count_legal_permutations_inner(rules, positions, &mut perm, 0)
}

#[allow(dead_code)]
fn count_legal_permutations_inner(rules: &[Rule], positions: &Vec<IndexSet>, perm: &mut Vec<usize>, i: usize) -> usize {
    if i == rules.len() {
        return 1;
    }

    let mut count = 0;

    for pos in positions[i].iter() {
        if perm.contains(&pos) {
            continue;
        }

        perm.push(pos);
        count += count_legal_permutations_inner(rules, positions, perm, i+1);
        perm.pop();
    }

    count
}