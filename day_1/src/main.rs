const INPUT: &str = include_str!("../input.txt");

// This specifies our number system for use in `.to_digit()`
const NUMBER_SYSTEM_BASE: u32 = 10;

/// Finds the first "standard" digit. "standard" in this context means a numerical representation
/// of the number, like "1".
fn find_first_standard_digit(line: &str) -> u32 {
    for c in line.chars() {
        if let Some(digit) = c.to_digit(NUMBER_SYSTEM_BASE) {
            return digit;
        }
    }

    panic!("If this point is reached, something is wrong with the input data.")
}

/// Finds the last "standard" digit. "standard" in this context means a numerical representation
/// of the number, like "1".
fn find_last_standard_digit(line: &str) -> u32 {
    for c in line.chars().rev() {
        if let Some(digit) = c.to_digit(NUMBER_SYSTEM_BASE) {
            return digit;
        }
    }

    panic!("If this point is reached, something is wrong with the input data.")
}

/// Concatenates two standard digits. For example, 2 and 7 becomes 27.
/// "standard" in this context means a numerical representation of the number, like "1".
fn concat_standard_digits(first_digit: u32, last_digit: u32) -> u32 {
    (first_digit.to_string() + &last_digit.to_string())
        .parse()
        .unwrap()
}

fn main() {
    let lines = INPUT.lines().collect::<Vec<&str>>();

    // Part 1
    let mut accumulator = 0;

    for line in lines {
        let first_digit = find_first_standard_digit(line);
        let last_digit = find_last_standard_digit(line);

        let concatenated_digits = concat_standard_digits(first_digit, last_digit);

        accumulator += concatenated_digits;
    }

    println!("Sum of calibration values: {accumulator}");
}
