use std::{
    env, fs,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn main() {
    let input_filename: &str = &env::args()
        .skip(1)
        .next()
        .expect("This program must be passed an input filename.");

    let file =
        fs::File::open(input_filename).expect("The input filename to be an existing readable file");

    let reader = BufReader::new(file);

    // Dial starts pointing at 50, can be negative (considered a u8 starting at 100, but the range of values isn't enough to cover the potential distances)
    let mut dial: i16 = 50;
    // We want to count the number of times the dial points to 0
    let mut zero_end_counter: u16 = 0;
    let mut zero_pass_counter: u16 = 0;

    for line in reader.lines() {
        if line.is_err() {
            panic!("Failed to succesfully read the file");
        }

        let command: String = line.expect("Checked if line read failed");

        let direction = match command.get(..1) {
            Some("L") => Direction::Left,
            Some("R") => Direction::Right,
            _ => continue,
        };

        let distance: &str = command
            .get(1..)
            .expect("Should be a valid i16 number following the direction");
        let distance: i16 = distance
            .parse::<i16>()
            .expect("Should be a valid i16 number following the direction");

        // Part 2
        // Count the number of times the dial crosses 0

        // First off, we _know_ that for every 100 units of distance, the dial will pass zero.
        let (hundreds, remainder) = (distance.div_euclid(100), distance.rem_euclid(100));
        zero_pass_counter += hundreds as u16;

        dbg!((
            dial,
            &direction,
            hundreds,
            remainder,
            zero_end_counter,
            zero_pass_counter
        ));

        // Move the dial
        match direction {
            Direction::Left => {
                // Check if the dial will cross 0, if we're starting at 0, then it doesn't count as a cross
                if dial != 0 && dial - remainder < 0 {
                    zero_pass_counter += 1;
                }

                dial -= remainder;
            }
            Direction::Right => {
                // Check if the dial will cross 100
                if dial + remainder > 100 {
                    zero_pass_counter += 1;
                }

                dial += remainder;
            }
        }

        // Dial wraps around at 100
        dial = dial.rem_euclid(100);

        // Count how many times the dial points to 0
        if dial == 0 {
            zero_end_counter += 1;
        }
    }

    println!(
        "The dial points to zero {} times, passes zero {} times. For a grand total of {} times at zero.",
        zero_end_counter,
        zero_pass_counter,
        zero_end_counter + zero_pass_counter
    );
}
