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

//Originally tried to brute force. After verifying that my algo was working correctly
//It would not arrive at a solution, implying that the number of steps is computationally infeasible to brute force

//A clever way to arrive at the solution more quickly is to calculate the number of steps for
//each starting key and then finding the Least Common Multiple of steps required for each key
//to arrive at the solution as each key forms their own pattern that repeats at the next multiple of steps
//the Least Common Multiple is essentially when the pattern for all these keys sync up

// Function to calculate total winnings
fn calculate_total(input_data: &str) -> Result<isize, PuzzleErr> {
    let mut choices_map: HashMap<&str, (&str, &str)> = HashMap::new();

    let parts: Vec<&str> = input_data
        .split("\n\n")
        .map(|s| s.trim())
        .collect();

    let pattern = parts[0];

    //println!("{}", pattern);

    let map = parts[1];

    //parse input and create hashmap to hold the key value pairs
    for line in map.lines() {
        let parts: Vec<&str> = line
            .split('=')
            .map(|s| s.trim())
            .collect();

        let key = parts[0];
        let choices: Vec<&str> = parts[1]
            .trim_matches(|c| (c == '(' || c == ')'))
            .split(',')
            .map(|s| s.trim())
            .collect();

        choices_map.insert(key, (&choices[0], &choices[1]));
        //println!("Key: {}, Value: {:?}", key, choices_map.get(key));
    }

    //println!("Choices Map: {:?}", choices_map);

    //get all key values from the map wherein the key ends with an A

    //initialize a vector to hold the key-value pairs with the initial values (ends with A). select left first
    let value_vec: Vec<&str> = choices_map
        .iter()
        .filter(|(key, _value)| key.ends_with("A"))
        .map(|(key, _value)| {
            //println!("Initial: key={}, value={:?}", key, value);
            *key
        })
        .collect();

    //println!("Initial Value Vec: {:?}", value_vec);

    let mut steps_vec: Vec<u64> = Vec::new();

    //iterate through the values and end the function and return the step count when all the values end with a z
    fn update_key_left<'a>(
        value: &'a str,
        choices_map: &'a HashMap<&'a str, (&'a str, &'a str)>
    ) -> &'a str {
        let new_value = choices_map.get(value);
        new_value.unwrap().0
    }

    fn update_key_right<'a>(
        value: &'a str,
        choices_map: &'a HashMap<&'a str, (&'a str, &'a str)>
    ) -> &'a str {
        let new_value = choices_map.get(value);
        new_value.unwrap().1
    }

    for key in value_vec.iter() {
        //println!("Calculating steps for key: {}", key);
        let mut i = 0;
        let mut steps: isize = 0;
        let mut temp_key = *key;
        loop {
            if i == pattern.len() {
                i = 0;
            }
            if key.ends_with('Z') || temp_key.ends_with('Z') {
                break;
            }
            if pattern.chars().nth(i).unwrap() == 'R' {
                //println!("Right");
                let new_temp_key = update_key_right(temp_key, &choices_map);
                temp_key = new_temp_key;
                //println!("New Temp Key: {}", temp_key);
            } else if pattern.chars().nth(i).unwrap() == 'L' {
                //println!("Left");
                let new_temp_key = update_key_left(temp_key, &choices_map);
                temp_key = new_temp_key;
                //println!("New Temp Key: {}", temp_key);
            }
            //println!("Value Vec: {:?}{}", value_vec, i);
            steps += 1;
            i += 1;
        }
        steps_vec.push(steps as u64);
        //println!("Steps for key {}: {:?}", key, steps);
    }

    //println!("Steps: {:?}", steps_vec);

    // Function to calculate the greatest common divisor (GCD) using Euclidean algorithm
    fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    // Function to calculate the least common multiple (LCM) of a vector of numbers
    fn lcm_of_vec(numbers: Vec<u64>) -> u64 {
        if numbers.len() == 0 {
            return 0; // LCM is undefined for an empty list, you may want to handle this case differently
        }

        let mut lcm_result = numbers[0];

        for &num in numbers.iter().skip(1) {
            let gcd_value = gcd(lcm_result, num);
            lcm_result = (lcm_result * num) / gcd_value;
        }

        lcm_result
    }

    println!("Steps: {:?}", steps_vec[0]);

    Ok(lcm_of_vec(steps_vec) as isize)
}

// Main function to execute the puzzle
pub fn main(data_dir: &str) {
    let data = load(data_dir, 082, None);

    let answer = puzzle(&data);
    match answer {
        Ok(x) => println!(" Puzzle 8-1: {}", x),
        Err(e) => panic!("No solution to puzzle: {}.", e),
    }
    assert_eq!(answer, Ok(9064949303801));
}
