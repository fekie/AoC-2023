const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
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
}

fn main() {
    let races = Race::races_from_input(INPUT);

    dbg!(races);
}
