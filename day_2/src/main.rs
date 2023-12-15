const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn generate_smallest_bag(game: &Game) -> Self {
        let mut highest_red = 0;
        let mut highest_green = 0;
        let mut highest_blue = 0;

        for set in &game.0 {
            if set.red > highest_red {
                highest_red = set.red;
            }

            if set.green > highest_green {
                highest_green = set.green;
            }

            if set.blue > highest_blue {
                highest_blue = set.blue;
            }
        }

        Self {
            red: highest_red,
            green: highest_green,
            blue: highest_blue,
        }
    }

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
    // Part 1
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

    // Part 2
    let power_sum = INPUT
        .lines()
        .map(Game::new)
        .map(|game| {
            let smallest_bag = Bag::generate_smallest_bag(&game);

            smallest_bag.red * smallest_bag.green * smallest_bag.blue
        })
        .sum::<u32>();

    println!("Sum of powers of bags: {power_sum}");
}
