use thiserror::Error;
use crate::data::load;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Could not locate digit.")]
    NoDigits,
}

fn calculate_total(input_data: &str) -> Result<isize, PuzzleErr> {
    let mut total: u32 = 0;

    //declare variable to hold the final parameter calculation

    let mut add_first_last: u32 = 0;

    let calib_params_split = input_data.split("\n");

    let collect_params = calib_params_split.collect::<Vec<&str>>();

    // Loop over each item in array

    for item in collect_params {
        // declare temporary variable to hold the first_number and the last_number and set them to NULL
        let mut first_number: Option<u32> = None;
        let mut last_number: Option<u32> = None;
        // loop through characters in array item.

        for letter in item.chars() {
            // if letter.is_numeric() && first_number.is_empty then save to first_number and continue to next char

            if letter.is_numeric() && first_number.is_none() {
                first_number = letter.to_digit(10);
            }
            // if character == number && first_number !== NULL then save to last_number and continue

            if letter.is_numeric() && !first_number.is_none() {
                last_number = letter.to_digit(10);
            }

            //end loop -- when loop ends first_number should contain the first number and lastCharacter should contain the last number
        }

        // calculate parameters
        //(first number * 10) + finalNumber?
        add_first_last = first_number.unwrap() * 10 + last_number.unwrap();
        //add them to total
        total += add_first_last;
        //set back to zero for next loop
        add_first_last = 0;
        //end loop
    }
    Ok(total as isize)
}

pub fn puzzle(input_data: &str) -> Result<isize, PuzzleErr> {
    calculate_total(input_data)
}

pub fn main(data_dir: &str) {
    // open file with raw text

    let data = load(data_dir, 011, None);
    // declare total to add calibration parameters to

    // Puzzle 1.
    let answer = puzzle(&data);
    match answer {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle: {}.", e),
    }
    assert_eq!(answer, Ok(52974));
}
