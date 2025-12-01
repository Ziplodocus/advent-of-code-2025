use std::{
    env, fs,
    io::{BufRead, BufReader},
    str::Chars,
};

enum Direction {
    Left,
    Right,
}

fn main() {
    let input_filename: &str = &env::args()
        .skip(1)
        .next()
        .expect("This program must be passed an input filename.");

    dbg!(input_filename);

    let file =
        fs::File::open(input_filename).expect("The input filename to be an existing readable file");

    let reader = BufReader::new(file);

    let current: u16;

    for line in reader.lines() {
        if line.is_err() {
            panic!("Failed to succesfully read the file");
        }

        let mut command: Chars = line.expect("Checked if line read failed").chars();

        let direction = match command.next() {
            Some('L') => Some(Direction::Left),
            Some('R') => Some(Direction::Right),
            _ => None,
        };

        // Skip this line if the input is not as expected
        if direction.is_none() {
            continue;
        }

        let direction = direction.expect("None value is not possible due to previous check");

        let distance = command.as_str();

        dbg!(distance);
    }
}
