use std::collections::HashMap;

use aoc_runner_derive::*;

use PassportAttribute::*;
type Passport = HashMap<PassportAttribute, String>;

#[derive(PartialEq, Eq, Hash, Debug)]
enum PassportAttribute {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportID,
    CountryID,
}

macro_rules! handle_year {
    ($val:expr, $lower:expr, $upper:expr) => {{
        if $val.len() != 4 {
            return false;
        }
        let year = parse_or_return!($val, false);
        $lower <= year && year <= $upper
    }}
}

macro_rules! parse_or_return {
    ($e:expr, $ret:expr) => {
        match u16::from_str_radix($e, 10) {
            Ok(v) => v,
            Err(_) => return $ret
        }
    };
}

static EYE_COLORS: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

impl PassportAttribute {
    fn check(&self, val: &str) -> bool {
        match self {
            BirthYear => handle_year!(val, 1920, 2002),
            IssueYear => handle_year!(val, 2010, 2020),
            ExpirationYear => handle_year!(val, 2020, 2030),

            Height => {
                let height = parse_or_return!(&val[..val.len()-2], false);
                match &val[val.len()-2..] {
                    "cm" => 150 <= height && height <= 193,
                    "in" => 59 <= height && height <= 76,
                    _ => false
                }
            },
            HairColor => {
                if val.chars().nth(0).unwrap() != '#' || val.len() != 7 {
                    return false;
                }
                u32::from_str_radix(&val[1..], 16).is_ok()
            },
            EyeColor => EYE_COLORS.contains(&val),
            PassportID => val.len() == 9 && u32::from_str_radix(&val, 10).is_ok(),
            CountryID => true
        }
    }
}

#[aoc_generator(day4)]
fn generate(input: &str) -> Vec<Passport> {
    let mut output = Vec::new();

    let mut current = Passport::new();
    for line in input.lines() {
        if line.len() == 0 {
            output.push(current);
            current = Passport::new();
            continue;
        }

        for attr in line.split(' ') {
            let (attr, val) = parse_attribute(attr).expect("Passport attribute parsing error.");
            current.insert(attr, val);
        }
    }
    if current.len() != 0 {
        output.push(current);
    }

    output
}

fn parse_attribute(attr: &str) -> Option<(PassportAttribute, String)> {
    let mut splut = attr.split(':');
    let (first, second) = (splut.next()?, splut.next()?);

    let first = match first {
        "byr" => BirthYear,
        "iyr" => IssueYear,
        "eyr" => ExpirationYear,
        "hgt" => Height,
        "hcl" => HairColor,
        "ecl" => EyeColor,
        "pid" => PassportID,
        "cid" => CountryID,

        _ => return None,
    };

    Some((first, second.to_string()))
}


#[aoc(day4, part1)]
fn solve_part1(input: &[Passport]) -> usize {
    input.iter()
        .filter(|x| is_valid_passport(x))
        .count()
}

static TO_CHECK: [PassportAttribute; 7] = [BirthYear, IssueYear, ExpirationYear, Height, HairColor, EyeColor, PassportID];

fn is_valid_passport(passport: &Passport) -> bool {
    for attr in &TO_CHECK {
        if !passport.contains_key(attr) {
            return false;
        }
    }

    true
}

#[aoc(day4, part2)]
fn solve_part2(input: &[Passport]) -> usize {
    input.iter()
        .filter(|x| is_valid_passport_strict(x))
        .count()
}

fn is_valid_passport_strict(passport: &Passport) -> bool {
    for attr in &TO_CHECK {
        let val = match passport.get(attr) {
            Some(val) => val,
            None => return false
        };

        if !attr.check(val) {
            return false;
        }
    }

    true
}