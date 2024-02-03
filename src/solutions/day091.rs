use crate::{data::load, Error};

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
    // Computed differences in original 
    let mut running_total = 0;

    for line in input_data.lines() {
        let mut start_vec: Vec<isize> = line
            .split_whitespace()
            .map(|s| s.parse::<isize>().unwrap_or(0))
            .collect();

        let original_vec = start_vec.clone();
        let mut difference_vecs: Vec<Vec<isize>> = Vec::new();
        let mut difference_vec: Vec<isize> = vec![1 as isize];

        while !difference_vec.iter().all(|&x| x == 0) {
            difference_vec.clear();
            let mut current_index = 0;
            while current_index < start_vec.len() - 1 {
                let diff = start_vec[current_index + 1] - start_vec[current_index];
                difference_vec.push(diff);
                current_index += 1;
            }
            difference_vecs.push(difference_vec.clone());
            start_vec = difference_vec.clone();
        }

        // Now calculate the amount to add to the running total
        let mut value_to_add_to_total = 0;

        // Iterate through difference_vecs starting from the second-to-last vector
        for i in (1..difference_vecs.len()).rev() {
            // Get the last value of the current vector
            let last_value_current = value_to_add_to_total;

            // Get the last value of the previous vector
            let last_value_previous = difference_vecs[i - 1].last().cloned().unwrap_or_default();

            // Add the last value of the current vector to the last value of the previous vector
            value_to_add_to_total = last_value_current + last_value_previous;
        }

        // Add the last value of the original_vec to the running total
        value_to_add_to_total += original_vec.last().cloned().unwrap_or_default();

        // Add the calculated value to the running total
        running_total += value_to_add_to_total;
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
