use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Game {
    #[allow(dead_code)]
    id: u32,
    winning_numbers: Vec<u32>,
    available_numbers: Vec<u32>,
}

impl Game {
    fn new(line: &str) -> Self {
        // We separate the raw strings in the input.
        let (id_raw, winning_numbers_raw, available_numbers_raw) = {
            let split_colon = line.split(':').collect::<Vec<&str>>();
            let split_pipe = split_colon[1].split('|').collect::<Vec<&str>>();

            (split_colon[0], split_pipe[0], split_pipe[1])
        };

        let id = id_raw.split_whitespace().collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

        let winning_numbers = winning_numbers_raw
            .split_whitespace()
            // We filter out any blank strings that may have got in. We turn
            // the valid chunks into integers.
            .filter_map(|chunk| chunk.parse().ok())
            .collect();

        let available_numbers = available_numbers_raw
            .split_whitespace()
            .filter_map(|chunk| chunk.parse().ok())
            .collect();

        Self {
            id,
            winning_numbers,
            available_numbers,
        }
    }

    fn calculate_value(&self) -> u32 {
        let winning_numbers_hashset = {
            let mut winning_numbers_hashset = HashSet::new();

            for number in &self.winning_numbers {
                winning_numbers_hashset.insert(*number);
            }

            winning_numbers_hashset
        };

        let matches = self
            .available_numbers
            .iter()
            .filter_map(|number| winning_numbers_hashset.get(number))
            .count() as u32;

        match matches {
            0 => 0,
            _ => 2_u32.pow(matches - 1),
        }
    }
}

fn main() {
    let games = INPUT.lines().map(Game::new).collect::<Vec<Game>>();

    let sum_of_game_values: u32 = games.iter().map(Game::calculate_value).sum();

    println!("Sum of Game Values: {sum_of_game_values}")
}
