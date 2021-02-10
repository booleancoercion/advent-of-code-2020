use aoc_runner_derive::*;

use Direction::*;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,

    Forward,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Direction> {
        Some(match c {
            'N' => North,
            'E' => East,
            'S' => South,
            'W' => West,
            'F' => Forward,
            'L' => Left,
            'R' => Right,
            _ => return None,
        })
    }

    fn turn(&self, dir: Direction) -> Option<Direction> {
        Some(match (self, dir) {
            (North, Left) => West,
            (West, Left) => South,
            (South, Left) => East,
            (East, Left) => North,

            (North, Right) => East,
            (East, Right) => South,
            (South, Right) => West,
            (West, Right) => North,

            _ => return None,
        })
    }

    fn turn_degs(&self, dir: Direction, degs: i32) -> Option<Direction> {
        let times = degs / 90;
        let mut curr = *self;

        for _ in 0..times {
            let out = curr.turn(dir);
            curr = match out {
                Some(val) => val,
                None => return None,
            }
        }

        Some(curr)
    }

    fn to_coords(&self) -> Option<(i32, i32)> {
        Some(match self {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),

            _ => return None,
        })
    }
}

struct Instruction(Direction, i32);

#[aoc_generator(day12)]
fn generate(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let c = line.chars().next().unwrap();
            let num: i32 = line[1..].parse().unwrap();

            Instruction(Direction::from_char(c).unwrap(), num)
        })
        .collect()
}

#[aoc(day12, part1)]
fn solve_part1(input: &[Instruction]) -> i32 {
    let mut pos: (i32, i32) = (0, 0);

    let mut facing = East;
    for Instruction(dir, num) in input {
        let dir = match dir {
            Left | Right => {
                facing = facing.turn_degs(*dir, *num).unwrap();
                continue;
            }

            Forward => facing,
            x => *x,
        };

        let coords = dir.to_coords().unwrap();
        pos.0 += num * coords.0;
        pos.1 += num * coords.1;
    }

    pos.0.abs() + pos.1.abs()
}

#[aoc(day12, part2)]
fn solve_part2(input: &[Instruction]) -> i32 {
    let mut pos: (i32, i32) = (0, 0);
    let mut waypoint: (i32, i32) = (10, 1);

    for Instruction(dir, num) in input {
        match dir {
            Left | Right => turn_waypoint(&mut waypoint, *dir, *num),
            Forward => {
                pos.0 += waypoint.0 * num;
                pos.1 += waypoint.1 * num;
            }

            x => {
                let coords = x.to_coords().unwrap();
                waypoint.0 += coords.0 * num;
                waypoint.1 += coords.1 * num;
            }
        }
    }

    println!("({}, {})", pos.0, pos.1);
    pos.0.abs() + pos.1.abs()
}

fn turn_waypoint(waypoint: &mut (i32, i32), dir: Direction, deg: i32) {
    let times = deg / 90;

    for _ in 0..times {
        let temp = waypoint.0;
        match dir {
            Left => {
                waypoint.0 = -waypoint.1;
                waypoint.1 = temp;
            }
            Right => {
                waypoint.0 = waypoint.1;
                waypoint.1 = -temp;
            }

            _ => {}
        }
    }
}
