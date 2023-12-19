use grid::Grid;

const INPUT: &str = include_str!("../input.txt");

/// Contains clusters of numbers horizontally adjacent to each other,
/// along with the bordering units. Naming it Ships cause it reminds
/// me of Battleship.
#[derive(Debug)]
struct Ship {
    body: Vec<UnitWithCoords>,
    borders: Vec<UnitWithCoords>,
}

impl Ship {
    fn is_valid(&self) -> bool {
        for unit_with_coords in &self.borders {
            if let Unit::Symbol(_) = unit_with_coords.unit {
                return true;
            }
        }

        false
    }

    fn as_number(&self) -> usize {
        let digits_as_string =
            self.body
                .iter()
                .map(|unit_with_coords| match unit_with_coords.unit {
                    Unit::Digit(digit) => digit.to_string(),
                    _ => panic!("The body should only be comprised of digits."),
                });

        let mut concatenated_digits_string = String::new();

        for digit_string in digits_as_string {
            concatenated_digits_string.push_str(&digit_string);
        }

        concatenated_digits_string.parse().unwrap()
    }
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
        if let Some(unit_with_coords) = self.get_unit_with_coords(x, y) {
            trailing_digits_with_locations.push(unit_with_coords);
        }

        while let Some(unit_with_coords) = self.get_unit_with_coords_right(x, y) {
            match unit_with_coords.unit {
                Unit::Digit(_) => trailing_digits_with_locations.push(unit_with_coords),
                _ => break,
            }

            x += 1
        }

        // We grab the coords of the body in (x, y) tuples.
        let body_coords = trailing_digits_with_locations
            .iter()
            .map(|unit_with_coords| (unit_with_coords.x, unit_with_coords.y))
            .collect::<Vec<(usize, usize)>>();

        // We grab the left and right edges of the body
        let horizontal_edges_coords = [
            self.get_unit_with_coords_left(
                body_coords.first().unwrap().0,
                body_coords.first().unwrap().1,
            ),
            self.get_unit_with_coords_right(
                body_coords.last().unwrap().0,
                body_coords.last().unwrap().1,
            ),
        ]
        .iter()
        .flatten()
        .map(|unit_with_coords| (unit_with_coords.x, unit_with_coords.y))
        .collect::<Vec<(usize, usize)>>();

        let mut border_units_options = Vec::new();

        // Before we combine the coords we collected above, we need to
        // add the edges to the borders.
        border_units_options.extend(
            horizontal_edges_coords
                .iter()
                .map(|(x, y)| self.get_unit_with_coords(*x, *y)),
        );

        // Now we want to combine the coords vectors, and add everything above and below to
        // the border_units vector
        let all_middle_row_coords = {
            let mut middle_row_coords = Vec::new();
            middle_row_coords.extend(body_coords);
            middle_row_coords.extend(horizontal_edges_coords);
            middle_row_coords
        };

        border_units_options.extend(all_middle_row_coords.iter().flat_map(|(x, y)| {
            vec![
                self.get_unit_with_coords_up(*x, *y),
                self.get_unit_with_coords_down(*x, *y),
            ]
        }));

        let border_units = border_units_options.into_iter().flatten().collect();

        // We used `.to_vec()` here because it clones it? the compiler told me to.
        let body_units = trailing_digits_with_locations.to_vec();

        Ship {
            body: body_units,
            borders: border_units,
        }
    }

    fn get_unit_with_coords(&self, x: usize, y: usize) -> Option<UnitWithCoords> {
        self.0
            .get(y, x)
            .cloned()
            .map(|unit| UnitWithCoords { x, y, unit })
    }

    fn get_unit_with_coords_left(&self, x: usize, y: usize) -> Option<UnitWithCoords> {
        if x == 0 {
            return None;
        }

        let new_x = x - 1;

        self.0
            .get(y, new_x)
            .cloned()
            .map(|unit| UnitWithCoords { x: new_x, y, unit })
    }

    fn get_unit_with_coords_up(&self, x: usize, y: usize) -> Option<UnitWithCoords> {
        if y == 0 {
            return None;
        }

        let new_y = y - 1;

        self.0
            .get(new_y, x)
            .cloned()
            .map(|unit| UnitWithCoords { x, y: new_y, unit })
    }

    fn get_unit_with_coords_right(&self, x: usize, y: usize) -> Option<UnitWithCoords> {
        let x_max = self.0.cols() - 1;

        if x == x_max {
            return None;
        }

        let new_x = x + 1;

        self.0
            .get(y, new_x)
            .cloned()
            .map(|unit| UnitWithCoords { x: new_x, y, unit })
    }

    fn get_unit_with_coords_down(&self, x: usize, y: usize) -> Option<UnitWithCoords> {
        let y_max = self.0.rows() - 1;

        if y == y_max {
            return None;
        }

        let new_y = y + 1;

        self.0
            .get(new_y, x)
            .cloned()
            .map(|unit| UnitWithCoords { x, y: new_y, unit })
    }
}

#[derive(Debug, Clone, Copy)]
struct UnitWithCoords {
    x: usize,
    y: usize,
    unit: Unit,
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

    let valid_ship_number_sum: usize = ships
        .iter()
        .filter(|ship| ship.is_valid())
        .map(Ship::as_number)
        .sum();

    println!("Valid Ship Number Sum: {}", valid_ship_number_sum);
}
