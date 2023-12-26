//! DISCLAIMER: this currently does not work and may crash your pc if you run it.

use std::collections::VecDeque;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Field(VecDeque<Vec<i64>>);

impl Field {
    // Turns a line into a field with a single starting row.
    fn new(line: &str) -> Self {
        let mut rows = VecDeque::new();

        // Parse and put the first row of numbers into `rows`
        rows.push_back(
            line.split_whitespace()
                .filter_map(|chunk| chunk.parse().ok())
                .collect(),
        );

        Self(rows)
    }

    // Populates the rows below the starting row, all the way
    // until the last line is 0 0 0 0...
    fn populate(&mut self) {
        for i in 0.. {
            let current_row = self.0[0].clone();
            let mut new_row = Vec::new();

            // This loop does loop, not sure why clippy says it doesn't.
            #[allow(clippy::never_loop)]
            for window in current_row.windows(2) {
                let left = window[0];
                let right = window[1];

                let new = right - left;

                new_row.push(new);
            }

            self.0.push_back(new_row);

            // We check to see if the last row is all zeros yet.
            let non_zero_number_count = self
                .0
                .iter()
                .last()
                .unwrap()
                .iter()
                .filter(|number| **number != 0)
                .count();

            if non_zero_number_count == 0 {
                break;
            }
        }
    }
}

fn main() {
    let mut fields = INPUT.lines().map(Field::new).collect::<Vec<_>>();

    fields[0].populate();

    dbg!(&fields[0]);
}
