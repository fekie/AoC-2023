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
    for (i, c) in line.chars().rev().enumerate() {
        if let Some(digit) = c.to_digit(NUMBER_SYSTEM_BASE) {
            return (digit, i);
        }
    }

    panic!("If this point is reached, something is wrong with the input data.")
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

/// Finds the first and last digits of a line. We do it in one function as
/// finding the words in these strings is a bit intensive. This will find both
/// word numbers and "standard" digits.
fn find_first_and_last_digit(line: &str) -> (u32, u32) {
    let mut number_words_found: HashMap<usize, &str> = HashMap::new();

    for number_word in NUMBER_WORDS {
        if let Some(index_found_at) = line.find(number_word) {
            number_words_found.insert(index_found_at, number_word);
        }
    }

    let (first_standard_digit, first_standard_digit_index) = find_first_standard_digit(line);
    let (last_standard_digit, last_standard_digit_index) = find_last_standard_digit(line);

    // If we don't have any number words then we return the first and last standard digits.
    if number_words_found.is_empty() {
        return (first_standard_digit, last_standard_digit);
    }

    // We gather the indices of the number words and sort from smallest to largest.
    let mut number_word_indices = number_words_found.keys().copied().collect::<Vec<usize>>();
    number_word_indices.sort();

    let mut first_digit = None;
    let mut last_digit = None;

    let first_number_word_index = number_word_indices.first().unwrap();

    if first_standard_digit_index < *first_number_word_index {
        // If the first standard digit index is lower than the first number, we return
        // the first standard digit.
        first_digit = Some(first_standard_digit);
    } else {
        // Otherwise, we return the first of the number words;
        let number_word = number_words_found.get(first_number_word_index).unwrap();

        let digit = number_word_to_digit(number_word);

        first_digit = Some(digit)
    }

    let last_number_word_index = number_word_indices.last().unwrap();

    if last_standard_digit_index > *last_number_word_index {
        // If the last standard digit is higher then the first number,
        // we return the last standard digit.
        last_digit = Some(last_standard_digit)
    } else {
        // Otherwise, we return the last of the number words;
        let number_word = number_words_found.get(last_number_word_index).unwrap();

        let digit = number_word_to_digit(number_word);

        last_digit = Some(digit)
    }

    (first_digit.unwrap(), last_digit.unwrap())
}

fn main() {
    let lines = INPUT.lines().collect::<Vec<&str>>();

    // Part 1
    let mut accumulator = 0;

    for line in &lines {
        let (first_digit, _) = find_first_standard_digit(line);
        let (last_digit, _) = find_last_standard_digit(line);

        let concatenated_digits = concat_digits(first_digit, last_digit);

        accumulator += concatenated_digits;
    }

    println!("Sum of calibration values: {accumulator}");

    // Part 2

    let mut accumulator = 0;

    for line in &lines {
        let (first_digit, last_digit) = find_first_and_last_digit(line);

        let concatenated_digits = concat_digits(first_digit, last_digit);

        accumulator += concatenated_digits;
    }

    println!("Sum of calibration values including number words: {accumulator}")
}
