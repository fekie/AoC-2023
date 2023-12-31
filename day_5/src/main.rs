use rayon::prelude::*;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct MapLine {
    destination_range_start: i64,
    source_range_start: i64,
    range: i64,
}

impl MapLine {
    fn new(line: &str) -> Self {
        let split = line.split_whitespace().collect::<Vec<&str>>();

        let destination_range_start = split[0].parse().unwrap();
        let source_range_start = split[1].parse().unwrap();
        let range = split[2].parse().unwrap();

        Self {
            destination_range_start,
            source_range_start,
            range,
        }
    }

    /// Converts an input according to the given map line.
    /// Returns Some(x) if the mapping changed the value,
    /// None otherwise.
    fn convert(&self, input: i64) -> Option<i64> {
        match (self.source_range_start..self.source_range_start + self.range).contains(&input) {
            true => {
                let offset = self.destination_range_start - self.source_range_start;
                Some(input + offset)
            }
            false => None,
        }
    }
}

#[derive(Debug)]
struct Map {
    map_lines: Vec<MapLine>,
}

impl Map {
    /// Converts a seed into the correct mapping of the seed.
    fn convert(&self, input: i64) -> i64 {
        self.map_lines
            .iter()
            .flat_map(|map_line| map_line.convert(input))
            .min()
            .unwrap_or(input)
    }
}

impl Map {
    fn new(input_line_blocks: Vec<&str>) -> Self {
        let input_line_blocks = input_line_blocks.into_iter();

        let map_lines = input_line_blocks.skip(1).map(MapLine::new).collect();

        Self { map_lines }
    }
}

fn main() {
    // Part 1
    // We create an iterator that contains vectors of lines in the input file.
    // This splits the file by the blank lines.
    let mut input_line_blocks = parse_input_line_blocks().into_iter();

    let mut seeds = parse_seeds(input_line_blocks.next().unwrap().first().unwrap());

    let maps = input_line_blocks.map(Map::new).collect::<Vec<Map>>();

    let outputs = seeds.iter_mut().map(|input| {
        maps.iter().for_each(|map| {
            *input = map.convert(*input);
        });

        *input
    });

    let lowest = outputs.min().unwrap();

    println!("Lowest Soil Value: {lowest}");

    // Part 2
    let mut input_line_blocks = parse_input_line_blocks().into_iter();

    let mut all_seeds_from_pairs = {
        let mut all_seeds_from_pairs = Vec::new();

        parse_seeds(input_line_blocks.next().unwrap().first().unwrap())
            .chunks(2)
            .for_each(|chunk| all_seeds_from_pairs.extend(chunk[0]..chunk[0] + chunk[1]));

        all_seeds_from_pairs
    };

    let maps = input_line_blocks.map(Map::new).collect::<Vec<Map>>();

    let outputs = all_seeds_from_pairs.par_iter_mut().map(|input| {
        maps.iter().for_each(|map| {
            *input = map.convert(*input);
        });

        *input
    });

    let lowest = outputs.min().unwrap();

    println!("Lowest Soil Value Using Pairs: {lowest}");
}

fn parse_input_line_blocks() -> Vec<Vec<&'static str>> {
    let mut split_by_blank_line = Vec::new();

    let mut line_buffer = Vec::new();

    for line in INPUT.lines() {
        match line.is_empty() {
            true => {
                split_by_blank_line.push(line_buffer.clone());
                line_buffer.clear();
            }
            false => line_buffer.push(line),
        }
    }

    // At the end of the loop, make sure the line buffer is added.
    split_by_blank_line.push(line_buffer);

    split_by_blank_line
}

/// Parses the seeds given in the first line in the file.
fn parse_seeds(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .filter_map(|raw| raw.parse().ok())
        .collect()
}
