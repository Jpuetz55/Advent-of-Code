use std::collections::HashMap;

use crate::data::load;
use thiserror::Error;
use permutator::{ Combination, Permutation };

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Could not locate digit.")]
    NoDigits,
}
// Main puzzle-solving function
fn puzzle(input_data: &str) -> Result<isize, PuzzleErr> {
    calculate_total(input_data)
}

// Function to calculate total winnings
fn calculate_total(input_data: &str) -> Result<isize, PuzzleErr> {}

// Main function to execute the puzzle
pub fn main(data_dir: &str) {
    let data = load(data_dir, 081, None);

    let answer = puzzle(&data);
    match answer {
        Ok(x) => println!(" Puzzle 8-1: {}", x),
        Err(e) => panic!("No solution to puzzle: {}.", e),
    }
    assert_eq!(answer, Ok(253473930));
}
