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
    let mut choices_map: HashMap<&str, (&str, &str)> = HashMap::new();

    let parts: Vec<&str> = input_data
        .split("\n\n")
        .map(|s| s.trim())
        .collect();

    let pattern = parts[0];
    let map = parts[1];
    // hard set starting key
    let start_key = "DBQ";

    for line in map.lines() {
        let parts: Vec<&str> = line
            .split('=')
            .map(|s| s.trim())
            .collect();
        if parts.len() == 2 {
            let key = parts[0];
            let choices: Vec<&str> = parts[1]
                .trim_matches(|c| (c == '(' || c == ')'))
                .split(',')
                .map(|s| s.trim())
                .collect();
            if choices.len() == 2 {
                choices_map.insert(key, (choices[0], choices[1]));
            }
        }
    }

    let mut key = start_key;
    //iterator for pattern
    let mut i = 0;
    let mut select_choice = "";

    let mut steps: isize = 0;
    while key != "ZZZ" {
        if i >= pattern.len() {
            i = 0;
        }
        steps += 1;
        let (choice1, choice2) = choices_map.get(key).unwrap();
        if pattern.chars().nth(i).unwrap() == 'L' {
            select_choice = choice1;
        } else if pattern.chars().nth(i).unwrap() == 'R' {
            select_choice = choice2;
        }
        key = select_choice; // Fix here: use the choice directly as the new key
        i += 1;
    }
    Ok(steps)
}

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
