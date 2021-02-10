use aoc_runner_derive::*;

type TreeMatrix = [Vec<bool>];

#[aoc_generator(day3)]
fn generate_tree_matrix(input: &str) -> Vec<Vec<bool>> {
    let mut matrix = Vec::new();
    for line in input.lines() {
        matrix.push(Vec::new());
        let row = matrix.last_mut().unwrap();
        for c in line.chars() {
            row.push(c == '#');
        }
    }

    matrix
}

#[aoc(day3, part1)]
fn solve_part1(input: &TreeMatrix) -> u32 {
    count_trees_slope(input, 1, 3)
}

fn count_trees_slope(input: &TreeMatrix, down: usize, right: usize) -> u32 {
    let mut trees = 0;
    let mut i = 0;
    let mut j = 0;

    while i < input.len() {
        if input[i][j] {
            trees += 1;
        }
        i += down;
        j = (j + right) % input[0].len();
    }
    trees
}

#[aoc(day3, part2)]
fn solve_part2(input: &TreeMatrix) -> u32 {
    count_trees_slope(input, 1, 1)
        * count_trees_slope(input, 1, 3)
        * count_trees_slope(input, 1, 5)
        * count_trees_slope(input, 1, 7)
        * count_trees_slope(input, 2, 1)
}
