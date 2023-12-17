use grid::Grid;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
enum Unit {
    Blank,
    Symbol(char),
    Digit(u8),
}

impl Unit {
    fn new(c: char) -> Self {
        if let Some(parsed_digit) = c.to_digit(10) {
            return Self::Digit(parsed_digit as u8);
        }

        match c {
            '.' => Self::Blank,
            _ => Self::Symbol(c),
        }
    }
}

fn generate_engine_schematic() -> Grid<Unit> {
    let mut lines = INPUT.lines().peekable();

    let line_length = lines.peek().unwrap().len();

    let parsed_and_flattened = lines
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .map(Unit::new)
        .collect::<Vec<Unit>>();

    Grid::from_vec(parsed_and_flattened, line_length)
}

fn main() {
    let engine_schematic = generate_engine_schematic();
}
