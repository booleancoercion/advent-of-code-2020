use aoc_runner_derive::*;

struct Ticket {
    row: [bool; 7],
    col: [bool; 3],
}

macro_rules! binary_search {
    ($arr:expr, $start:expr, $end:expr) => {{
        let len = $arr.len();
        let mut a = $start;
        let mut b = $end;

        for i in 0..(len - 1) {
            let mid = (a + b) / 2;
            if $arr[i] {
                a = mid + 1;
            } else {
                b = mid;
            }
        }

        if $arr[len - 1] {
            b
        } else {
            a
        }
    }};
}

impl Ticket {
    fn new() -> Self {
        Ticket {
            row: [false; 7],
            col: [false; 3],
        }
    }

    fn row_num(&self) -> usize {
        binary_search!(self.row, 0, 127)
    }

    fn col_num(&self) -> usize {
        binary_search!(self.col, 0, 7)
    }

    fn id(&self) -> usize {
        self.row_num() * 8 + self.col_num()
    }
}

#[aoc_generator(day5)]
fn generate(input: &str) -> Vec<Ticket> {
    let mut output = Vec::new();

    for line in input.lines() {
        let mut line = line.chars();
        let mut temp = Ticket::new();
        for i in 0..7 {
            temp.row[i] = line.next().unwrap() == 'B';
        }

        for i in 0..3 {
            temp.col[i] = line.next().unwrap() == 'R';
        }
        output.push(temp);
    }

    output
}

#[aoc(day5, part1)]
fn solve_part1(input: &[Ticket]) -> usize {
    input.iter().map(|x| x.id()).max().unwrap()
}

#[aoc(day5, part2)]
fn solve_part2(input: &[Ticket]) -> usize {
    let sorted = {
        let mut temp = input.iter().map(|x| x.id()).collect::<Vec<usize>>();
        temp.sort_unstable();
        temp
    };

    for i in 0..sorted.len() - 1 {
        let gap = sorted[i + 1] - sorted[i];

        if gap == 2 {
            return sorted[i] + 1;
        }
    }
    unreachable!()
}
