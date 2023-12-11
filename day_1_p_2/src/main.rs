use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

// This specifies our number system for use in `.to_digit()`
const NUMBER_SYSTEM_BASE: u32 = 10;

const WORD_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_word_digit(word_digit: &str) -> u32 {
    match word_digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("`word_digit` should be one of the words in WORD_DIGITS."),
    }
}

/// Returns a `HashMap` of the index of the digit for the key
/// and the digit for the value. Standard digit in this context refers
/// to the digits being strings like "1" and "7".
fn find_standard_digits(line: &str) -> HashMap<usize, u32> {
    let mut standard_digits_found = HashMap::new();

    for (i, c) in line.chars().enumerate() {
        if let Some(digit) = c.to_digit(NUMBER_SYSTEM_BASE) {
            standard_digits_found.insert(i, digit);
        }
    }

    standard_digits_found
}

/// Returns a `HashMap` of the index of the digit for the key
/// and the digit for the value. Word digit in this context refers to digits
/// found as strings in the form of "one" and "seven"
fn find_word_digits(line: &str) -> HashMap<usize, u32> {
    let mut word_digits_found = HashMap::new();

    for word_digit_str in WORD_DIGITS {
        // We check every possible slice of the string that is
        // the size of the word digit string we are checking for.
        let max_index = line.len() - 1;

        let mut start_index = 0;
        let mut end_index = word_digit_str.len() - 1;

        while end_index <= max_index {
            let slice = &line[start_index..=end_index];

            if slice == word_digit_str {
                let parsed_digit = parse_word_digit(slice);
                word_digits_found.insert(start_index, parsed_digit);
            }

            // I know I could've just used an offset for the loop
            // instead of incrementing both but this looks nicer.
            start_index += 1;
            end_index += 1;
        }
    }

    word_digits_found
}

fn concat_digits(first_digit: u32, last_digit: u32) -> u32 {
    (first_digit * 10) + last_digit
}

/// Finds the standard digits and word digits in a line.
/// Returns them as a `HashMap` where the key is the index where
/// it was found and the value is the digit.
fn find_digits(line: &str) -> HashMap<usize, u32> {
    let mut digits = HashMap::new();

    let standard_digits = find_standard_digits(line);
    let word_digits = find_word_digits(line);

    digits.extend(standard_digits);
    digits.extend(word_digits);

    digits
}

fn main() {
    let lines = INPUT.lines().collect::<Vec<&str>>();

    let mut accumulator = 0;

    for line in &lines {
        let digits = find_digits(line);

        let mut digit_indices = digits.keys().copied().collect::<Vec<usize>>();
        digit_indices.sort();

        let first_digit = digits[digit_indices.first().unwrap()];
        let last_digit = digits[digit_indices.last().unwrap()];

        let calibration_value = concat_digits(first_digit, last_digit);

        accumulator += calibration_value;
    }

    println!("Sum of calibration values including number words: {accumulator}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_edge_cases() {
        let input = r#"eightveight
v4"#;

        let mut answers = vec![HashMap::new(); 2];

        answers[0].insert(0, 8);
        answers[0].insert(6, 8);

        answers[1].insert(1, 4);

        let found_answers = input
            .lines()
            .map(find_digits)
            .collect::<Vec<HashMap<usize, u32>>>();

        assert_eq!(answers, found_answers);
    }
}
