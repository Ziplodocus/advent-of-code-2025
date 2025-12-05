use std::{
    env, fs,
    io::{BufRead, BufReader, Error},
    rc::Rc,
};

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

            let invalid_ids = extract_invalid_ids_from_range(range);

            dbg!(&range, &invalid_ids);

            return invalid_ids.into_iter().sum();
        })
        .sum();

    println!("The answer is: {}", result);
}

/// Determines if
fn extract_invalid_ids_from_range(range: (&str, &str)) -> Vec<usize> {
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
    //
    //
    // dbg!(&range, &invalid_ids);

    let invalid_ids: Vec<usize> = subdivide_range(range)
        .into_iter()
        .map(|range| handle_even_range((&range.0, &range.1)))
        .flatten()
        .collect();

    return invalid_ids;
}

///
fn subdivide_range<'a, 'b>(range: (&'a str, &'a str)) -> Vec<(Box<str>, Box<str>)> {
    let min_digits = range.0.chars().count();
    let max_digits = range.1.chars().count();

    let mut ranges = Vec::new();

    // Streamline cases where there are the same number of digits
    if min_digits == max_digits {
        if min_digits % 2 == 0 {
            ranges.push((Box::from(range.0), Box::from(range.1)));
        }
        return ranges;
    }

    for i in min_digits..max_digits {
        if i % 2 == 1 {
            continue;
        }

        let range_max: Box<str> = Box::from("9".repeat(i));

        if min_digits == i {
            ranges.push((Box::from(range.0), Box::from("9".repeat(min_digits))));
            continue;
        }

        let range_min: Box<str> = Box::from("1".to_string() + &"0".repeat(i - 1));

        if max_digits == i {
            ranges.push((range_min, Box::from(range.1)));

            dbg!(&range, &invalid_ids);
            continue;
        }

        ranges.push((range_min, range_max));
    }

    return ranges;
}

/// Returns the invalid ids in a range of _even_ digit numbers
fn handle_even_range(range: (&str, &str)) -> Vec<usize> {
    let min_first_digits = &range.0[0..(range.0.chars().count() / 2)];
    let max_first_digits = &range.1[0..(range.1.chars().count() / 2)];

    // Unwrap since these are definitely numbers
    let min_first_numbers = min_first_digits.parse::<usize>().expect("Valid number");
    let max_first_numbers = max_first_digits.parse::<usize>().expect("Valid number");

    let actual_min = range.0.parse::<usize>().expect("Valid number");
    let actual_max = range.1.parse::<usize>().expect("Valid number");

    let range: usize = max_first_numbers - min_first_numbers;

    let mut invalid_ids = Vec::new();

    let potential_min = (min_first_numbers.to_string().repeat(2))
        .parse::<usize>()
        .expect("Valid number");

    if actual_min <= potential_min {
        invalid_ids.push(potential_min)
    }

    if range == 0 {
        return invalid_ids;
    }

    for i in 0..(range - 1) {
        let invalid_first_digits = (min_first_numbers + i).to_string();
        let invalid_id = invalid_first_digits.repeat(2);
        invalid_ids.push(invalid_id.parse::<usize>().expect("Valid number"));
    }

    let potential_max = (max_first_digits.to_string().repeat(2))
        .parse::<usize>()
        .expect("Valid number");

    if potential_max <= actual_max {
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
