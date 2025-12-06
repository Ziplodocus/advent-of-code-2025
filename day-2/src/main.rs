use std::{
    env, fs,
    io::{BufRead, BufReader, Error},
};

use crate::bound::{Bound, Range};

mod bound;

#[cfg(test)]
mod tests;

fn main() {
    let reader = open_input_file();

    let result: usize = reader
        .split(b',')
        .map(|bytes| -> usize {
            let range: String = bytes
                .expect("Input is comma separated ranges as utf8")
                .try_into()
                .expect("Is string");

            let range = range.split_once('-').expect("Range is dash separated");
            let range = Range::new(range.0.into(), range.1.into());

            let invalid_ids: Vec<usize> = find_invalid_ids(range);

            dbg!(&invalid_ids);

            return invalid_ids.into_iter().sum();
        })
        .sum();

    println!("The answer is: {}", result);
}

/// Determines if
fn find_invalid_ids(range: bound::Range) -> Vec<usize> {
    // Invalid if the number is, in it's entirety, just 2 numbers repeated...
    //
    // Some cases...
    // When the min && max both have odd num of digits there are no invalid IDS in the range
    //
    // When min even, max even, the range of the first half of the digits determines how many invalid IDs there could be...
    // e.g 1160xxxx 1164xxxx There could be up to 5 invalid IDs in this range, since that's how many numbers could possibly be repeated
    //
    // so for both even case, get a list of potential digits... in the case of the above:
    // [1160, 1161, 1162, 1163, 1164],
    //
    // so maybe I can identify all possibles, then actually check if those numbers are in the range...
    //
    // if range is 11604000 - 11641000, we have the potential invalid numbers = [11601160, 11611161, 11621162, 11631163, 11641164].
    // so we can filter the invalid numbers by seeing if they are actually in the total range.
    // 11604000 < potential < 11641000
    //
    // In fact.. the only ones we actually need to check are the lowest and highest
    // of these potentials as the others are guranteed to fall in this range
    //
    // In the case where 1 of min or max is odd and the other is even, we can subdivide this into smaller ranges of even - even numbers
    // We can already solve an even - even range.
    //
    // We always want to be working with even, even ranges of the same length.
    //
    // How to extract all even,even ranges from any range?
    //
    // if 100 - 200000, then we want the ranges, 1000 - 9999, 100000 - 200000

    let invalid_ids: Vec<usize> = subdivide_range(range)
        .into_iter()
        .map(handle_even_range)
        .flatten()
        .collect();

    return invalid_ids;
}

/// Divides a range into sub ranges of even digited numbers
fn subdivide_range<'a, 'b>(range: bound::Range) -> Vec<bound::Range> {
    let mut ranges: Vec<bound::Range> = Vec::new();

    // Streamline cases where there are the same number of digits
    if range.has_equal_digits() {
        if range.min.has_even_digits() {
            ranges.push(range);
        }
        return ranges;
    }

    // Generating sub ranges
    for i in range.min.digits..=range.max.digits {
        if i % 2 == 1 {
            continue;
        }

        let range_max: usize = 10usize.pow(i as u32) - 1;
        if range.min.digits == i {
            ranges.push(Range::new(range.min.clone(), range_max.into()));
            continue;
        }

        let range_min: usize = 10usize.pow((i - 1) as u32);
        if range.max.digits == i {
            ranges.push(Range::new(range_min.into(), range.max.clone()));
            continue;
        }

        ranges.push(Range::new(range_min.into(), range_max.into()));
    }

    return ranges;
}

/// Returns the invalid ids in a range of _even_ digit numbers
fn handle_even_range(range: bound::Range) -> Vec<usize> {
    let min_first_digits = &range.min.str[0..(range.max.digits / 2)];
    let max_first_digits = &range.max.str[0..(range.max.digits / 2)];

    // Unwrap since these are definitely numbers
    let min_first_numbers = min_first_digits.parse::<usize>().expect("Valid number");
    let max_first_numbers = max_first_digits.parse::<usize>().expect("Valid number");

    let span: usize = max_first_numbers - min_first_numbers;

    let mut invalid_ids = Vec::new();

    let potential_min = (min_first_numbers.to_string().repeat(2))
        .parse::<usize>()
        .expect("Valid number");

    dbg!(&potential_min, &range.min.value);

    if range.min.value <= potential_min && potential_min <= range.max.value {
        invalid_ids.push(potential_min)
    }

    if span == 0 {
        return invalid_ids;
    }

    for i in 1..span {
        let invalid_first_digits = (min_first_numbers + i).to_string();
        let invalid_id = invalid_first_digits.repeat(2);
        invalid_ids.push(invalid_id.parse::<usize>().expect("Valid number"));
    }

    let potential_max = (max_first_digits.to_string().repeat(2))
        .parse::<usize>()
        .expect("Valid number");

    if potential_max <= range.max.value {
        invalid_ids.push(potential_max);
    }

    return invalid_ids;
}

fn open_input_file() -> BufReader<fs::File> {
    let input_filename: &str = &env::args()
        .skip(1)
        .next()
        .expect("This program must be passed an input filename.");

    let file =
        fs::File::open(input_filename).expect("The input filename to be an existing readable file");

    let reader = BufReader::new(file);

    return reader;
}
