use std::{
    env, fs,
    io::{BufRead, BufReader},
    ops::Add,
    panic,
};

#[cfg(test)]
mod tests;

#[cfg(feature = "part1")]
const TO_ACTIVATE: usize = 2;
#[cfg(feature = "part2")]
const TO_ACTIVATE: usize = 12;

fn main() {
    let banks = open_input_file();

    let max_sum: usize = solve(banks);

    println!("Sum of all maximum banks is {}", max_sum);
}

fn solve(banks: BufReader<fs::File>) -> usize {
    let bank_maxes: Vec<usize> = banks
        .lines()
        .map(|bank| get_bank_max(bank.expect("Valid string")))
        .collect();

    return bank_maxes.into_iter().sum();
}

fn get_bank_max(bank: String) -> usize {
    let input_length = bank.chars().count();
    let mut activated: Vec<Battery> = Vec::new();
    // Convert to vec of 'Battery's so that the initial index is accessible, despite potential reordering.
    let batteries: Vec<Battery> = bank
        .chars()
        .enumerate()
        .map(|b| Battery {
            joltage: b.1.to_digit(10).expect("Number 0-9") as u8,
            index: b.0,
        })
        .collect();

    // We want to preserve the initial vec for taking sub slices after each viable biggest number is determined
    let mut sorted_bank = batteries.clone();
    sorted_bank.sort_by(|a, b| b.joltage.cmp(&a.joltage));

    for i in 0..TO_ACTIVATE {
        // Get the largest possible battery and then remove that battery form the sorted bank.
        let index = get_next_viable_biggest(&sorted_bank, TO_ACTIVATE - i, input_length);
        let battery = sorted_bank[index].clone();
        // Only items later in the bank can now be acticated
        sorted_bank = sorted_bank
            .into_iter()
            .filter(|b| b.index > battery.index)
            .collect();
        activated.push(battery);
    }

    // dbg!(activated.iter().fold("".to_string(), |mut acc, item| {
    //     acc += &item.joltage.to_string();
    //     acc
    // }));

    let sum = activated
        .into_iter()
        .enumerate()
        .fold(0u64, |mut acc, item| {
            let i = item.0;
            let battery: Battery = item.1;

            acc += battery.joltage as u64 * 10u64.pow((TO_ACTIVATE - i - 1) as u32);

            return acc;
        });

    return sum as usize;
}

/// Returns the index of the largest viable battery to activate
fn get_next_viable_biggest(
    sorted_bank: &Vec<Battery>,
    left_to_activate: usize,
    initial_length: usize,
) -> usize {
    // Make sure that there are enough batteries after
    // the biggest number to be activated.
    for (i, biggest) in sorted_bank.iter().enumerate() {
        if biggest.index <= initial_length - left_to_activate {
            return i;
        }
    }

    dbg!(sorted_bank);
    panic!("There should be a valid battery in this bank")
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

#[derive(PartialEq, PartialOrd, Clone, Debug)]
struct Battery {
    pub joltage: u8,
    pub index: usize,
}

impl Add<&Battery> for &Battery {
    type Output = usize;
    fn add(self, other: &Battery) -> Self::Output {
        return format!("{}{}", self.joltage, other.joltage)
            .parse::<usize>()
            .expect("Two numbers placed next to each other make a number");
    }
}
