use std::cmp::min;

use aoc_runner_derive::*;

use Tile::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Floor,
    EmptySeat,
    OccupiedSeat
}

type TileMatrix = Vec<Vec<Tile>>;

#[aoc_generator(day11)]
fn generate(input: &str) -> TileMatrix {
    input.lines()
        .map(|line| line.chars().map(|c| match c {
                '.' => Floor,
                'L' => EmptySeat,
                '#' => OccupiedSeat,
                _ => unreachable!()
            }).collect())
        .collect()
}

#[aoc(day11, part1)]
fn solve_part1(input: &TileMatrix) -> usize {
    let stable = advance_until_stable(input, count_occupied_neighbors, 4);

    count_occupied(&stable)
}

#[aoc(day11, part2)]
fn solve_part2(input: &TileMatrix) -> usize {
    let stable = advance_until_stable(input, count_occupied_visible, 5);

    count_occupied(&stable)
}

fn advance_until_stable(matrix: &TileMatrix, count: impl Fn(usize, usize, &TileMatrix) -> u8, threshold: u8) -> TileMatrix {
    let mut current = matrix.clone();

    loop {
        let prev = current;
        let out = advance_matrix(&prev, &count, threshold);
        current = out.0;

        if !out.1 {
            break;
        }
    }
    current
}

fn advance_matrix(matrix: &TileMatrix, count: impl Fn(usize, usize, &TileMatrix) -> u8, threshold: u8) -> (TileMatrix, bool) {
    let mut output = matrix.clone();
    let mut changed = false;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] != Floor {
                let occupied = count(i, j, matrix);
                if matrix[i][j] == EmptySeat && occupied == 0 {
                    output[i][j] = OccupiedSeat;
                    changed = true;
                } else if matrix[i][j] == OccupiedSeat && occupied >= threshold {
                    output[i][j] = EmptySeat;
                    changed = true;
                }
            }
        }
    }

    (output, changed)
}

fn count_occupied_neighbors(i: usize, j: usize, matrix: &TileMatrix) -> u8 {
    let mut count = 0;
    
    for new_i in sub_or_zero(i, 1)..=min(i+1, matrix.len()-1) {
        for new_j in sub_or_zero(j, 1)..=min(j+1, matrix[0].len()-1) {
            if new_i == i && new_j == j {
                continue;
            }

            if matrix[new_i][new_j] == OccupiedSeat {
                count += 1;
            }
        }
    }

    count
}

fn count_occupied_visible(i: usize, j: usize, matrix: &TileMatrix) -> u8 {
    let mut count = 0;
    let leni = matrix.len() as isize;
    let lenj = matrix[0].len() as isize;

    for offset_i in -1..=1 {
        for offset_j in -1..=1 {
            if offset_i == 0 && offset_j == 0 {
                continue;
            }

            let mut new_i = i as isize + offset_i;
            let mut new_j = j as isize + offset_j;
            while new_i >= 0 && new_i < leni && new_j >= 0 && new_j < lenj {
                match matrix[new_i as usize][new_j as usize] {
                    EmptySeat => break,
                    OccupiedSeat => { count += 1; break },
                    Floor => { new_i += offset_i; new_j += offset_j }
                }
            }
        }
    }

    count
}

fn sub_or_zero(a: usize, b: usize) -> usize {
    a.checked_sub(b).unwrap_or(0)
}

fn count_occupied(matrix: &TileMatrix) -> usize {
    matrix.iter()
        .flatten()
        .filter(|x| **x == OccupiedSeat)
        .count()
}

#[allow(dead_code)]
fn print_matrix(matrix: &[Vec<Tile>]) {
    let mut out = String::new();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            out.push(match matrix[i][j] {
                Floor => '.',
                EmptySeat => 'L',
                OccupiedSeat => '#'
            });
        }
        out.push('\n');
    }

    println!("{}", out);
}