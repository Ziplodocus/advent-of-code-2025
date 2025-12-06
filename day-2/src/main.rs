#[cfg(feature = "part1")]
use part_1::part_1;
// #[cfg(feature = "part2")]
use part_2::part_2;
use std::{env, fs, io::BufReader};

mod bound;

#[cfg(feature = "part1")]
mod part_1;
// #[cfg(feature = "part2")]
mod part_2;

fn main() {
    #[cfg(feature = "part1")]
    part_1();
    // #[cfg(feature = "part2")]
    part_2();
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
