use std::collections::HashMap;

use crate::{ data::load, Error };

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Could not locate digit.")]
    NoDigits,
}

// Main puzzle-solving function
pub fn puzzle(input_data: &str) -> Result<isize, PuzzleErr> {
    calculate_total(input_data)
}

// Function to calculate total winnings
fn calculate_total(input_data: &str) -> Result<isize, PuzzleErr> {
    //for each line in input_data
    //declare vecs Vec to store all number_vec's
    //set start_vec from input_data
    //set index to first item
    //compare with next item
    //push the difference to difference_vec (scope: loop)
    //do this until the current_index is the last value in the start_vec (scope: loop)
    //check to see if all the values are zero
    //if false
    //push start_vec to vecs
    //restart the process with the difference_vec as the start_vec
    //if true
    //start at the last vec in vecs, get the value of the last index, add it to the last value in the next vec, repeat this process
    //until the next vec is the first vec in vecs. add this value to running_total (scope:global)

    Ok(0 as isize)
}

// Main function to execute the puzzle
pub fn main(data_dir: &str) {
    let data = load(data_dir, 091, None);

    let answer = puzzle(&data);
    match answer {
        Ok(x) => println!(" Puzzle 9-1: {}", x),
        Err(e) => panic!("No solution to puzzle: {}.", e),
    }
    assert_eq!(answer, Ok(9064949303801));
}
