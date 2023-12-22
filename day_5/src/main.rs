use std::str::Lines;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct MapLine {
    destination_range_start: u64,
    source_range_start: u64,
    range: u64,
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
}

#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    map_lines: Vec<MapLine>,
}

impl Map {
    fn new(input_line_blocks: Vec<&str>) -> Self {
        let mut input_line_blocks = input_line_blocks.into_iter();

        let (from, to) = {
            let first_line = input_line_blocks.next().unwrap();
            let split = first_line.split('-').collect::<Vec<&str>>();
            (
                split[0].to_string(),
                split[2].split_whitespace().collect::<Vec<&str>>()[0].to_string(),
            )
        };

        let map_lines = input_line_blocks.map(MapLine::new).collect();

        Self {
            from,
            to,
            map_lines,
        }
    }
}

fn main() {
    // We create an iterator that contains vectors of lines in the input file.
    // This splits the file by the blank lines.
    let mut input_line_blocks = parse_input_line_blocks().into_iter();

    let seeds = parse_seeds(input_line_blocks.next().unwrap().first().unwrap());

    let maps = input_line_blocks.map(Map::new).collect::<Vec<Map>>();

    dbg!(maps);
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

    split_by_blank_line
}

/// Parses the seeds given in the first line in the file.
fn parse_seeds(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .filter_map(|raw| raw.parse().ok())
        .collect()
}