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

    let map = parts[1];

    //parse input and create hashmap to hold the key value pairs
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

    let mut steps: isize = 0;

    //get all key values from map wherein the key ends with an A

    //intialize a vector to hold the key value pairs with the initial values (ends with A). select left first
    let mut value_vec: Vec<&str> = choices_map
        .iter()
        .filter(|(key, _value)| key.ends_with("A"))
        .map(|(key, value)| {
            println!("Initial: key={}, value={:?}", key, value);
            value.0
        })
        .collect();

    //iterate through the values and end the function and return the step count when all the values end with a z
    //initialize a boolean switch to right value
    let mut bool_switch = true;

    loop {
        if value_vec.iter().all(|value| value.ends_with("Z")) {
            break;
        } else {
            //populate new vec with all the values selected from the value_vec
            //choose value 1 if bool_switch is true, value2 otherwise
            if bool_switch {
                println!("Switch is true");
                value_vec = value_vec
                    .iter()
                    .map(|&value| {
                        let new_value = choices_map.get(value).map_or(value, |(v, _)| {
                            // println!("Choosing left: value={}", v);
                            v
                        });
                        new_value
                    })
                    .collect();
            } else {
                println!("Switch is false");
                value_vec = value_vec
                    .iter()
                    .map(|&value| {
                        let new_value = choices_map.get(value).map_or(value, |(_, v)| {
                            println!("Choosing right: value={}", v);
                            v
                        });
                        new_value
                    })
                    .collect();
            }

            //flip switch
            bool_switch = !bool_switch;
            steps += 1;
        }
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
    assert_eq!(answer, Ok(12737));
}
