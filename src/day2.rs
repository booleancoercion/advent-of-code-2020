use aoc_runner_derive::*;
use regex::Regex;

pub struct Password {
    pub lower: u8,
    pub upper: u8,
    pub letter: char,
    pub password: String
}

impl Password {
    fn is_valid(&self) -> bool {
        let mut letter_count = 0;
        for c in self.password.chars() {
            if c == self.letter {
                letter_count += 1;
                if letter_count > self.upper {
                    return false;
                }
            }
        }
        letter_count >= self.lower
    }
}

#[aoc_generator(day2, part1)]
fn generate_passwords(input: &str) -> Vec<Password> {
    let rc = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

    let mut vec = Vec::new();
    for mat in rc.captures_iter(input) {
        let pwd = Password {
            lower: u8::from_str_radix(&mat[1], 10).unwrap(),
            upper: u8::from_str_radix(&mat[2], 10).unwrap(),
            letter: mat[3].chars().nth(0).unwrap(),
            password: mat[4].to_string()
        };

        vec.push(pwd);
    };
    vec
}

#[aoc(day2, part1)]
fn solve_part1(input: &[Password]) -> u32 {
    let mut counter = 0;

    for pwd in input {
        if pwd.is_valid() {
            counter += 1;
        }
    }
    counter
}