//! DISCLAIMER: Solution does not work. Works with the test data given.
//! Don't really have the time to debug it further as it's exam week and I'm
//! really tired of this puzzle. Code quality is not the best as this was worked on
//! past 3am over two nights with a lot of hacky code.

use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

// This specifies our number system for use in `.to_digit()`
const NUMBER_SYSTEM_BASE: u32 = 10;

const NUMBER_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// Finds the first "standard" digit. "standard" in this context means a numerical representation
/// of the number, like "1". Returns the digit and the index where it was located.
fn find_first_standard_digit(line: &str) -> (u32, usize) {
    for (i, c) in line.chars().enumerate() {
        if let Some(digit) = c.to_digit(NUMBER_SYSTEM_BASE) {
            return (digit, i);
        }
    }

    panic!("If this point is reached, something is wrong with the input data.")
}

/// Finds the last "standard" digit. "standard" in this context means a numerical representation
/// of the number, like "1". Returns the digit and the index where it was located.
fn find_last_standard_digit(line: &str) -> (u32, usize) {
    // As we are reversing the iterator here, we need to make sure that
    // we subtract the new index from the line max index to get the true index.
    let line_max_index = line.len() - 1;

    for (reversed_index, c) in line.chars().rev().enumerate() {
        if let Some(digit) = c.to_digit(NUMBER_SYSTEM_BASE) {
            let i = line_max_index - reversed_index;
            return (digit, i);
        }
    }

    panic!("If this point is reached, something is wrong with the input data.")
}

/// Finds the "non-standard" digits. "non-standard" in this context means
/// the word form of the number, like "one" for 1.
///
/// We return a HashMap of the index where the beginning of the word
/// was found as the key, and the digit as the value.
#[allow(clippy::type_complexity)]
fn find_non_standard_digits(line: &str) -> HashMap<usize, u32> {
    // We start with a hashmap containing the index of the number word, and the
    // digit that it parses to.
    let mut non_standard_digits_found: HashMap<usize, u32> = HashMap::new();

    for number_word in NUMBER_WORDS {
        if let Some(index_found_at) = line.find(number_word) {
            let digit = number_word_to_digit(number_word);

            non_standard_digits_found.insert(index_found_at, digit);
        }
    }

    non_standard_digits_found
}

fn concat_digits(first_digit: u32, last_digit: u32) -> u32 {
    (first_digit.to_string() + &last_digit.to_string())
        .parse()
        .unwrap()
}

fn number_word_to_digit(number_word: &str) -> u32 {
    match number_word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("`number_word` should be one of the words in NUMBER_WORDS."),
    }
}

/// Finds the first and last digits of a line. Sometimes the last digit does not exist
/// because first and last digit cannot have the same index. We do it in one function as
/// finding the words in these strings is a bit intensive. This will find both
/// word numbers and "standard" digits.
fn find_first_and_last_digit(line: &str) -> (u32, Option<u32>) {
    let non_standard_digits = find_non_standard_digits(line);

    let (first_standard_digit, first_standard_digit_index) = find_first_standard_digit(line);
    let (last_standard_digit, last_standard_digit_index) = find_last_standard_digit(line);

    // If we don't have any number words then we return the first and last standard digits.
    if non_standard_digits.is_empty() {
        // We make sure that the first and last index aren't the same.
        // If they arent, then we return the digits as normal. Otherwise,
        // we do not return the last digit.
        return match first_standard_digit_index != last_standard_digit_index {
            true => (first_standard_digit, Some(last_standard_digit)),
            false => (first_standard_digit, None),
        };
    }

    // We gather the indices of the number words and sort from smallest to largest.
    let mut number_word_indices = non_standard_digits.keys().copied().collect::<Vec<usize>>();
    number_word_indices.sort();

    let (first_digit, first_index) = {
        let first_number_word_index = number_word_indices.first().unwrap();

        match first_standard_digit_index < *first_number_word_index {
            // If the first standard digit index is lower than the first number, we return
            // the first standard digit.
            true => (first_standard_digit, first_standard_digit_index),
            // Otherwise, we return the first of the number words;
            false => (
                *non_standard_digits.get(first_number_word_index).unwrap(),
                *first_number_word_index,
            ),
        }
    };

    let (last_digit, last_index) = {
        let last_number_word_index = number_word_indices.last().unwrap();

        match last_standard_digit_index > *last_number_word_index {
            // If the last standard digit is higher then the first number,
            // we return the last standard digit.
            true => (last_standard_digit, last_standard_digit_index),
            // Otherwise, we return the last of the number words;
            false => (
                *non_standard_digits.get(last_number_word_index).unwrap(),
                *last_number_word_index,
            ),
        }
    };

    // We make sure that the first and last index aren't the same.
    // If they arent, then we return the digits as normal. Otherwise,
    // we do not return the last digit.
    match first_index != last_index {
        true => (first_digit, Some(last_digit)),
        false => (first_digit, None),
    }
}

fn main() {
    let lines = INPUT.lines().collect::<Vec<&str>>();

    let mut accumulator = 0;

    for line in &lines {
        let (first_digit, possible_last_digit) = find_first_and_last_digit(line);

        let concatenated_digits = match possible_last_digit {
            Some(last_digit) => concat_digits(first_digit, last_digit),
            None => first_digit,
        };

        accumulator += concatenated_digits;
    }

    println!("Sum of calibration values including number words: {accumulator}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_finding_first_standard_digit() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        let answers = vec![(1, 0), (3, 3), (1, 1), (7, 4)];

        let found_answers = input
            .lines()
            .map(find_first_standard_digit)
            .collect::<Vec<(u32, usize)>>();

        assert_eq!(answers, found_answers);
    }

    #[test]
    fn test_finding_last_standard_digit() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        let answers = vec![(2, 4), (8, 7), (5, 9), (7, 4)];

        let found_answers = input
            .lines()
            .map(find_last_standard_digit)
            .collect::<Vec<(u32, usize)>>();

        assert_eq!(answers, found_answers);
    }

    #[test]
    fn test_finding_non_standard_digits() {
        let input = r#"two1nine
eightwothree"#;

        let mut answers = vec![HashMap::new(); 2];

        answers[0].insert(0, 2);
        answers[0].insert(4, 9);

        answers[1].insert(0, 8);
        answers[1].insert(4, 2);
        answers[1].insert(7, 3);

        let found_answers = input
            .lines()
            .map(find_non_standard_digits)
            .collect::<Vec<HashMap<usize, u32>>>();

        assert_eq!(answers, found_answers);
    }

    #[test]
    fn test_finding_first_and_last_digits() {
        let input = r#"two1nine
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
v4"#;

        let answers = vec![29, 13, 24, 42, 14, 76, 4];

        let found_answers = input
            .lines()
            .map(|line| {
                let (first_digit, possible_last_digit) = find_first_and_last_digit(line);
                dbg!(first_digit, possible_last_digit);

                match possible_last_digit {
                    Some(last_digit) => concat_digits(first_digit, last_digit),
                    None => first_digit,
                }
            })
            .collect::<Vec<u32>>();

        assert_eq!(answers, found_answers)
    }
}
