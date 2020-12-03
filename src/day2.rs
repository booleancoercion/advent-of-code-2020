use aoc_runner_derive::*;

pub struct Password {
    pub lower: u8,
    pub upper: u8,
    pub letter: char,
    pub password: String
}

impl Password {
    pub fn parse(string: &str) -> Password {
        let hyphen_i = string.find('-').unwrap();
        let space_i = string.find(' ').unwrap();
    
        Password {
            lower: u8::from_str_radix(&string[..hyphen_i], 10).unwrap(),
            upper: u8::from_str_radix(&string[hyphen_i+1..space_i], 10).unwrap(),
            letter: string[space_i+1..].chars().nth(0).unwrap(),
            password: string[space_i+4..].to_string()
        }
    }

    pub fn is_valid1(&self) -> bool {
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

    pub fn is_valid2(&self) -> bool {
        let mut chars = self.password.chars();
        let a = self.lower as usize - 1;
        let b = self.upper as usize - 1;
        (chars.nth(a).unwrap() == self.letter) ^ (chars.nth(b - a - 1).unwrap() == self.letter)
    }
}

#[aoc_generator(day2)]
fn generate_passwords(input: &str) -> Vec<Password> {
    let mut vec = Vec::new();
    for password in input.lines() {
        vec.push(Password::parse(password));
    }
    vec
}

#[aoc(day2, part1)]
fn solve_part1(input: &[Password]) -> u32 {
    let mut counter = 0;

    for pwd in input {
        if pwd.is_valid1() {
            counter += 1;
        }
    }
    counter
}

#[aoc(day2, part2)]
fn solve_part2(input: &[Password]) -> u32 {
    let mut counter = 0;

    for pwd in input {
        if pwd.is_valid2() {
            counter += 1;
        }
    }
    counter
}