#![feature(iter_intersperse)]

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn races_from_input(input: &str) -> Vec<Self> {
        let mut lines = input.lines();

        let times = lines
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|chunk| chunk.parse().ok());

        let distances = lines
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|chunk| chunk.parse().ok());

        times
            .zip(distances)
            .map(|(time, distance)| Self { time, distance })
            .collect()
    }

    fn large_race_from_input(input: &str) -> Self {
        let mut lines = input.lines();

        let time = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .intersperse("")
            .collect::<String>()
            .parse()
            .unwrap();

        let distance = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .intersperse("")
            .collect::<String>()
            .parse()
            .unwrap();

        Self { time, distance }
    }

    fn calculate_winning_press_times_amount(&self) -> u64 {
        // The equation looks like `d < p * (t - p)`,
        // where d = distance, p = time pressed, and t = time in the race.
        // This can be written as `d < -p^2 + pt`.
        // We have d and t available to us.
        (0..=self.time)
            .filter(|p| (-(p * p) + p * self.time) > self.distance)
            .count() as u64
    }
}

fn main() {
    // Part 1
    let races = Race::races_from_input(INPUT);

    let winning_press_times_amount = races
        .iter()
        .map(Race::calculate_winning_press_times_amount)
        .product::<u64>();

    println!("Winning Press Times Amount: {winning_press_times_amount}");

    // Part 2
    let large_race = Race::large_race_from_input(INPUT);

    let winning_press_times_amount = large_race.calculate_winning_press_times_amount();

    println!("Large Race Winning Times Amount: {winning_press_times_amount}");
}
