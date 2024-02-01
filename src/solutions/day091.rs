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
    //intialize vecs Vec to store all number_vec's
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
    //return running_total

    let mut vecs: Vec<Vec<isize>> = Vec::new();
    let mut running_total = 0;

    for line in input_data.lines() {
        let mut start_vec: Vec<isize> = line
            .split_whitespace()
            .map(|s| s.parse::<isize>().unwrap_or(0))
            .collect();

        let mut current_index = 0;

        while current_index < start_vec.len() {
            let mut difference_vec: Vec<isize> = Vec::new();

            while current_index < start_vec.len() - 1 {
                let diff = start_vec[current_index + 1] - start_vec[current_index];
                difference_vec.push(diff);
                current_index += 1;
            }

            if difference_vec.iter().all(|&x| x == 0) {
                break;
            }

            vecs.push(start_vec.clone());
            start_vec = difference_vec;
            current_index = 0;
        }

        //ok, this is fucked. bad job robot
        for i in (0..vecs.len()).rev() {
            let last_index = vecs[i].len() - 1;
            if let Some(last_value) = vecs[i].get(last_index) {
                running_total += last_value;
                if i > 0 {
                    start_vec[last_index] += last_value;
                }
            }
        }

        vecs.clear();
    }

    Ok(running_total)
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
