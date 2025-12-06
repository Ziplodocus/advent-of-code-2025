use std::{fs, io::BufReader};

use crate::{get_bank_max, solve};

#[test]
fn check_max_finder_1() {
    let bank: String = "987654321111111".to_string();
    let bank_max = get_bank_max(bank);

    #[cfg(feature = "part1")]
    let solution = 98;
    #[cfg(feature = "part2")]
    let solution = 987654321111;

    assert_eq!(bank_max, solution);
}

#[test]
fn check_max_finder_2() {
    let bank = "811111111111119".to_string();
    let bank_max = get_bank_max(bank);

    #[cfg(feature = "part1")]
    let solution = 89;
    #[cfg(feature = "part2")]
    let solution = 811111111119;

    assert_eq!(bank_max, solution);
}

#[test]
fn check_max_finder_3() {
    let bank = "234234234234278".to_string();
    let bank_max = get_bank_max(bank);

    #[cfg(feature = "part1")]
    let solution = 78;
    #[cfg(feature = "part2")]
    let solution = 434234234278;

    assert_eq!(bank_max, solution);
}

#[test]
fn check_max_finder_4() {
    let bank = "818181911112111".to_string();
    let bank_max = get_bank_max(bank);

    #[cfg(feature = "part1")]
    let solution = 92;
    #[cfg(feature = "part2")]
    let solution = 888911112111;

    assert_eq!(bank_max, solution);
}

#[test]
fn check_test_solution() {
    let file = fs::File::open("test-input.txt")
        .expect("The input filename to be an existing readable file");

    let reader = BufReader::new(file);

    let max = solve(reader);

    #[cfg(feature = "part1")]
    let solution = 357;
    #[cfg(feature = "part2")]
    let solution = 3121910778619;

    assert_eq!(max, solution)
}
