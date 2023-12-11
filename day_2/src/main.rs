const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn is_valid(&self, game: &Game) -> bool {
        let valid_sets = game
            .0
            .iter()
            .filter(|set| {
                (set.red <= self.red) && (set.green <= self.green) && (set.blue <= self.blue)
            })
            .count();

        // If the amount of valid sets are the same as the amount of sets,
        // all sets are valid.
        valid_sets == game.0.len()
    }
}

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn new(raw: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for chunk in raw.split(", ") {
            // The chunks will look like "6 red". We split them up in the middle
            // and then parse them from there.
            let split = chunk.split_whitespace().collect::<Vec<&str>>();

            let amount: u32 = split[0].parse().unwrap();
            let color_string = split[1];

            match color_string {
                "red" => red += amount,
                "green" => green += amount,
                "blue" => blue += amount,
                _ => panic!(
                    "{color_string} is not a valid color. Please choose 'red', 'green', or 'blue'."
                ),
            }
        }

        Self { red, green, blue }
    }
}

#[derive(Debug)]
struct Game(Vec<Set>);

impl Game {
    fn new(line: &str) -> Self {
        // We truncate the "Game n:" part. We have to do it by finding the colon
        // first as n can be 1-3 digits.
        let colon_location = line.find(':').unwrap();
        let truncated_line = &line[colon_location + 2..];

        Self(truncated_line.split("; ").map(Set::new).collect())
    }
}

fn main() {
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    let sum_of_ids = INPUT
        .lines()
        .map(Game::new)
        .enumerate()
        .filter_map(|(i, game)| {
            let game_id = i as u32 + 1;

            match bag.is_valid(&game) {
                true => Some(game_id),
                false => None,
            }
        })
        .sum::<u32>();

    println!("Sum of IDs: {sum_of_ids}");
}
