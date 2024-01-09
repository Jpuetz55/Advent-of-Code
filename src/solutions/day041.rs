use crate::data::load;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Could not locate digit.")]
    NoDigits,
}

fn calculate_total(input_data: &str) -> Result<isize, PuzzleErr> {
    let mut input_data_string = input_data.to_string();

    // Split the file content into lines
    let lines: Vec<&str> = input_data_string.lines().collect();
    let mut overall_total = 0;

    // Process each scratchcard
    for card in lines {
        let parts: Vec<&str> = card.split(":").collect();

        // Split the string into winning and held numbers
        let held_numbers = parts[1].split("|").collect::<Vec<&str>>();
        let winning_numbers = held_numbers[0].split_whitespace().collect::<Vec<&str>>();
        let held_numbers = held_numbers[1].split_whitespace().collect::<Vec<&str>>();

        // Convert strings to vectors of u32
        let winning_numbers: Vec<u32> = winning_numbers
            .iter()
            .map(|s| s.parse().unwrap_or(0))
            .collect();
        let held_numbers: Vec<u32> = held_numbers
            .iter()
            .map(|s| s.parse().unwrap_or(0))
            .collect();

        let mut card_total = 0;
        let mut is_first = true;

        // Check if held numbers are winning numbers and calculate points
        for number in &held_numbers {
            if winning_numbers.contains(number) {
                if is_first {
                    card_total += 1;
                    is_first = false;
                } else {
                    card_total *= 2;
                }
            }
        }

        // Add the card total to the overall total
        overall_total += card_total;
    }

    Ok(overall_total as isize)
}

pub fn puzzle(input_data: &str) -> Result<isize, PuzzleErr> {
    calculate_total(input_data)
}

pub fn main(data_dir: &str) {
    // open file with raw text

    let data = load(data_dir, 041, None);
    // declare total to add calibration parameters to

    let answer = puzzle(&data);
    match answer {
        Ok(x) => println!(" Puzzle 4-1: {}", x),
        Err(e) => panic!("No solution to puzzle: {}.", e),
    }
    assert_eq!(answer, Ok(21158));
}
