use std::collections::HashMap;

use grid::Grid;

const INPUT: &str = include_str!("../input.txt");

/// Contains clusters of numbers horizontally adjacent to each other,
/// along with the bordering units. Naming it Ships cause it reminds
/// me of Battleship.
#[derive(Debug)]
struct Ship {
    body: Vec<Unit>,
    borders: Vec<Unit>,
}

/// A type wrapper over a [`Grid`]. Our (0,0) on the grid is the top left corner,
/// with the y value increasing as you go down.
struct EngineSchematic(Grid<Unit>);

impl EngineSchematic {
    fn find_ships(&self) -> Vec<Ship> {
        let mut ships = Vec::new();

        for y in 0..self.0.rows() {
            let row = self.0.iter_row(y);

            // HACK ALERT. I know I could've done this other ways
            // but this may look a bit better?
            let mut skip_next = 0;

            for (x, unit) in row.enumerate() {
                if skip_next >= 1 {
                    skip_next -= 1;
                    continue;
                }

                // If the unit isnt a digit then we skip over it.
                match unit {
                    Unit::Digit(_) => {}
                    _ => continue,
                }

                // dbg!(unit);
                //dbg!(x, y);
                let ship = self.build_out_ship(x, y);

                // We skip the next parts of the body.
                skip_next += ship.body.len() - 1;

                ships.push(ship);
            }
        }

        ships
    }

    /// Takes the coordinates of a place where a digit was located,
    /// finds the rest of the digits (reading to the right, since we start from
    /// the left), and finds the borders, and puts it into a [`Ship`].\
    ///
    /// Panics if [`Self::get_unit(&self, x, y)`] does not give a [`Unit`].
    fn build_out_ship(&self, mut x: usize, y: usize) -> Ship {
        // We collect the body of the Ship here. Well, a tuple
        // that represents the unit and its coordinates like ((x, y), Unit).
        // I'm sorry it had to be this way, it's a bit hacky.
        let mut trailing_digits_with_locations = Vec::new();

        // We have to go ahead and add the original unit.
        if let Some(unit) = self.get_unit(x, y) {
            //dbg!(unit);
            //dbg!(x, y);
            trailing_digits_with_locations.push(((x, y), unit));
        }

        loop {
            // If we find a trailing digit, we add it to the ship
            match self.get_unit_right(x, y) {
                Some(unit) => match unit {
                    Unit::Digit(foo) => trailing_digits_with_locations.push(((x, y), unit)),
                    _ => break,
                },
                None => break,
            }

            // After this, we increment x by 1.
            x += 1;
        }

        // Now, we add the border of the ship. Basically, we scan the units
        // adjacent to the body of the ship, and keep them if they aren't `None`.
        let border_units = trailing_digits_with_locations
            .iter()
            .enumerate()
            .flat_map(|(i, ((x, y), _))| {
                let mut border_units = Vec::new();

                // If this is the first unit, we want to include the left unit
                // into the border.
                if i == 0 {
                    border_units.push(self.get_unit_left(*x, *y));
                }

                // If this is the last unit, we want to include the right unit.
                if i == trailing_digits_with_locations.len() {
                    border_units.push(self.get_unit_right(*x, *y))
                }

                // For all units, even on the ends, we want to include
                // the higher and lower ones.
                border_units.push(self.get_unit_up(*x, *y));
                border_units.push(self.get_unit_down(*x, *y));

                dbg!(x, y);
                dbg!(&border_units);

                border_units
            })
            .filter_map(|unit_opt| unit_opt)
            .collect();

        let body_units = trailing_digits_with_locations
            .iter()
            .map(|(_, unit)| *unit)
            .collect();

        // dbg!(&body_units);

        Ship {
            body: body_units,
            borders: border_units,
        }
    }

    fn get_unit(&self, x: usize, y: usize) -> Option<Unit> {
        self.0.get(y, x).cloned()
    }

    fn get_unit_left(&self, x: usize, y: usize) -> Option<Unit> {
        if x == 0 {
            return None;
        }

        self.0.get(y, x - 1).cloned()
    }

    fn get_unit_up(&self, x: usize, y: usize) -> Option<Unit> {
        if y == 0 {
            return None;
        }

        self.0.get(y - 1, x).cloned()
    }

    fn get_unit_right(&self, x: usize, y: usize) -> Option<Unit> {
        let x_max = self.0.cols() - 1;

        if x == x_max {
            return None;
        }

        self.0.get(y, x + 1).cloned()
    }

    fn get_unit_down(&self, x: usize, y: usize) -> Option<Unit> {
        let y_max = self.0.rows() - 1;

        if y == y_max {
            return None;
        }

        self.0.get(y + 1, x).cloned()
    }
}

#[derive(Debug, Clone, Copy)]
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

fn generate_engine_schematic() -> EngineSchematic {
    let mut lines = INPUT.lines().peekable();

    let line_length = lines.peek().unwrap().len();

    let parsed_and_flattened = lines
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .map(Unit::new)
        .collect::<Vec<Unit>>();

    EngineSchematic(Grid::from_vec(parsed_and_flattened, line_length))
}

fn main() {
    let engine_schematic = generate_engine_schematic();
    let ships = engine_schematic.find_ships();
    dbg!(ships);
}
