use std::io::BufRead;

use crate::open_input_file;

#[cfg(test)]
mod tests;

/// Brute force solution, checking every number within the ranges to see if it is invalid.
pub fn part_2() {
    let reader = open_input_file();

    let result: usize = reader
        .split(b',')
        .map(|bytes| -> Vec<usize> {
            let range: String = bytes
                .expect("Input is comma separated ranges as utf8")
                .try_into()
                .expect("Is string");

            let range = range.split_once('-').expect("Range is dash separated");
            let range = (
                range.0.parse().expect("Range start is a number"),
                range.1.parse().expect("Range end is a number"),
            );
            let invalid_ids = find_invalid_ids(range);

            invalid_ids
        })
        .flatten()
        .sum();

    println!("Part 2. Sum of all invalid IDs is: {}", result);
}

fn find_invalid_ids(range: (usize, usize)) -> Vec<usize> {
    let mut invalid_ids = Vec::new();

    for i in range.0..=range.1 {
        if is_invalid(i) {
            invalid_ids.push(i);
        }
    }

    return invalid_ids;
}

/// Checks an individual number to see if it is invalid
fn is_invalid(num: usize) -> bool {
    // Invalid Ids are made up entirely of a repeated sequence.

    let string = num.to_string();
    let digits = string.chars().count();

    // Loop through all possible sequence lengths (up to half of the max number of digits)
    for i in 1..=digits.div_euclid(2) {
        // We only need to consider sequence lengths that are factors of the number of digits
        let factor = digits.div_euclid(i);
        let rem = digits % i;
        if rem != 0 {
            continue;
        };

        // Check all lengths of seqence
        let sequence = &string[0..i];

        // Check if this sequence repeated matches the whole number
        // It is invalid if it does
        if sequence.repeat(factor) == string {
            return true;
        }
    }

    return false;
}
