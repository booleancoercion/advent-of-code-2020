use std::cmp::{max, min};

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
    let mut current = input.clone();
    let mut i_vals = (0, current.len()-1);
    let mut j_vals = (0, current[0].len()-1);
    loop {
        // print_matrix(&current[i_vals.0..=i_vals.1]);
        let prev = current;
        let out = advance_matrix(&prev, i_vals, j_vals);
        current = out.0;
        i_vals = out.1;
        j_vals = out.2;

        if i_vals.0 > i_vals.1 {
            break;
        }
    }

    current.iter()
        .flatten()
        .filter(|x| **x == OccupiedSeat)
        .count()
}

fn advance_matrix(matrix: &TileMatrix, in_i_vals: (usize, usize), in_j_vals: (usize, usize))
-> (TileMatrix, (usize, usize), (usize, usize)) {
    let mut output = matrix.clone();

    let mut i_vals = (usize::MAX, 0usize);
    let mut j_vals = (usize::MAX, 0usize);

    for i in sub_or_zero(in_i_vals.0, 1)..=min(in_i_vals.1+1, matrix.len()-1) {
        for j in sub_or_zero(in_j_vals.0, 1)..=min(in_j_vals.1+1, matrix[0].len()-1) {
            if matrix[i][j] != Floor {
                let occupied = count_occupied_neighbors(i, j, matrix);
                if matrix[i][j] == EmptySeat && occupied == 0 {
                    output[i][j] = OccupiedSeat;
                } else if matrix[i][j] == OccupiedSeat && occupied >= 4 {
                    output[i][j] = EmptySeat;
                } else {
                    continue;
                }

                i_vals.0 = min(i_vals.0, i);
                i_vals.1 = max(i_vals.1, i);

                j_vals.0 = min(j_vals.0, j);
                j_vals.1 = max(j_vals.1, j);
            }
        }
    }

    (output, i_vals, j_vals)
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

fn sub_or_zero(a: usize, b: usize) -> usize {
    a.checked_sub(b).unwrap_or(0)
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