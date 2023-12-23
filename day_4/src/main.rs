use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

struct GameTracker {
    tracked_games: HashMap<u32, u32>,
    current_game: u32,
    games: Vec<Game>,
}

impl GameTracker {
    fn new(games: Vec<Game>) -> Self {
        // We go ahead populate the tracked games. We start
        // with one copy of each card.
        let mut tracked_games = HashMap::new();
        let starting_game = 1;

        for id in starting_game as usize..=games.len() {
            tracked_games.insert(id as u32, 1);
        }

        Self {
            tracked_games,
            current_game: starting_game,
            games,
        }
    }

    /// Steps through the next game. Returns true if another step can
    /// be made, and returns false if the [`GameTracker`] has terminated.
    fn step(&mut self) -> bool {
        let game = &self.games[self.current_game as usize - 1];

        // We increment the next instances of the upcoming card ids by the amount
        // of cards of the game we just scratched off.
        //
        // We can unwrap these because we already populated the HashMap
        // on initialization of [`GameTracker`].
        let increment_amount = *self.tracked_games.get(&self.current_game).unwrap();

        let matches = game.calculate_matches();

        // We get the ids of the cards that we need to scratch off after this
        let upcoming_card_ids = (1..=matches).map(|game_index| game_index + self.current_game);

        for upcoming_card_id in upcoming_card_ids {
            let card_count = self.tracked_games.get_mut(&upcoming_card_id).unwrap();
            *card_count += increment_amount
        }

        self.current_game += 1;

        self.current_game != self.games.len() as u32
    }

    fn scratchcards_used(&self) -> u32 {
        self.tracked_games.values().sum()
    }
}

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

    fn calculate_matches(&self) -> u32 {
        let winning_numbers_hashset = {
            let mut winning_numbers_hashset = HashSet::new();

            for number in &self.winning_numbers {
                winning_numbers_hashset.insert(*number);
            }

            winning_numbers_hashset
        };

        self.available_numbers
            .iter()
            .filter_map(|number| winning_numbers_hashset.get(number))
            .count() as u32
    }
}

fn main() {
    // Part 1
    let games = INPUT.lines().map(Game::new).collect::<Vec<Game>>();

    let sum_of_game_values: u32 = games
        .iter()
        .map(Game::calculate_matches)
        .map(|matches| match matches {
            0 => 0,
            _ => 2_u32.pow(matches - 1),
        })
        .sum();

    println!("Sum of Game Values: {sum_of_game_values}");

    // Part 2
    let mut game_tracker = GameTracker::new(games);

    // Loop through all the steps.
    while game_tracker.step() {}

    let scratchcards_used = game_tracker.scratchcards_used();

    println!("Scratchcards Used: {scratchcards_used}")
}
