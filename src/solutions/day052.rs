use crate::data::load;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Could not locate digit.")]
    NoDigits,
}

// Function to parse a single map and return a 2D array
fn parse_map(map_str: &str) -> Vec<(usize, usize, usize)> {
    map_str
        .split_whitespace()
        .map(|num_str| num_str.parse().unwrap())
        .collect::<Vec<usize>>()
        .chunks_exact(3)
        .map(|chunk| (chunk[0], chunk[1], chunk[2]))
        .collect()
}

// Function to parse the entire input and return a vector of 2D arrays
fn parse_input(input: &str) -> Vec<Vec<(usize, usize, usize)>> {
    let mut maps = Vec::new();
    let input_to_vector: Vec<&str> = input.lines().collect();
    let mut current_map: Vec<(usize, usize, usize)> = Vec::new();

    for line in input_to_vector {
        if !line.is_empty() && line.chars().nth(0).unwrap().is_digit(10) {
            // Parse the line and add to the current map
            current_map.extend(parse_map(line));
        } else if line.is_empty() && !current_map.is_empty() {
            // Add the current map to the vector and clear it
            maps.push(current_map.clone());
            current_map.clear();
        }
    }

    // Add the last map if it's not empty
    if !current_map.is_empty() {
        maps.push(current_map);
    }

    maps
}

fn calculate_total(input_data: &str) -> Result<isize, PuzzleErr> {
    // Parse the seed numbers and put them in a vector of ranges
    let first_line: Option<&str> = input_data.lines().next();
    let mut seed_ranges: Vec<(usize, usize)> = Vec::new();
    let mut location_numbers: Vec<usize> = Vec::new();

    if let Some(line) = first_line {
        let parts: Vec<&str> = line.split(":").collect();
        seed_ranges = parts[1]
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .chunks_exact(2)
            .map(|chunk| (chunk[0], chunk[1]))
            .collect();
    }

    //... (Code to find and calculate seed values, explained in comments below)

    // Parse the input into a vector of 2D arrays
    let maps = parse_input(&input_data);

    // Iterate through all maps and print their information

    // Starting at the first line of the map
    // Iterate through the next maps
    // Need to create a loop for the lines inside each map

    // Need a mutable variable to store the initial seed number
    // and have it update with the value derived from each iteration through a map
    for seed_range in seed_ranges.iter() {
        let mut is_last_seed = false;
        let mut seed_initial = seed_range.0;
        let mut seed = seed_range.0;
        let mut check_overlap: i32;
        let mut seed_range_length = seed_range.1;
        let mut overlaps_to_eor_diffs: Vec<usize> = Vec::new();

        while !is_last_seed {
            seed = seed_initial;

            //set seed to the initial value for the seed range

            for (map_index, map) in maps.iter().enumerate() {
                for (line_index, line_values) in map.iter().enumerate() {
                    // Calculate the seed range start and end in the current map
                    // Access individual values in the line without dereferencing
                    let destination_start = line_values.0;
                    let source_start = line_values.1;
                    let mut range_length = line_values.2;

                    // Check if the seed range intersects with the source range
                    if seed >= source_start && seed <= source_start + range_length {
                        let distance = seed - source_start;
                        seed = destination_start + distance;

                        //need to get seed - source_start i.e. distance above when finding a match to adjust the range length of the map item
                        // Print debugging information
                        range_length = range_length - distance;

                        check_overlap = (seed_range_length as i32) - (range_length as i32);
                        if check_overlap > 0 {
                            overlaps_to_eor_diffs.push(range_length as usize);
                        }

                        // Go straight to the next map
                        break;
                    }
                }

                // If the seed range is not found in the current map, go to the next line in the map
            }

            // push the seed at the end of the loop. this is the location number for the
            // initial seed value
            location_numbers.push(seed - 1);
            if overlaps_to_eor_diffs.len() > 0 {
                //choose maximum value as this indicates the next change in mapping is closest on this overlap.
                //figured it out, the max_overlap value doesn't tell us which seed is updated first because
                //the range values for each map are variable in size
                //need to a different criteria for selecting which line of the map to update
                //the index value on. not necessarily the max_overlap value
                //more objective way to determine the index shift
                //need to figure out exactly how far from the end of the range the current_seed is
                //overlaps_to_eor_diffs should be the distance from the EOR for current seed
                let overlaps_to_eor_diff = overlaps_to_eor_diffs.iter().min().unwrap();
                //seed_range_length = it's length - minus the difference between it's length and the max overlap value
                //will correctly adjust the seed_range_length for the next iteration
                //next seed index will use the same value to calculate the next seed
                //bug has something to do with when range_length = distance, I think
                seed_initial = seed_initial + *overlaps_to_eor_diff + 1;
                seed_range_length = seed_range_length - *overlaps_to_eor_diff;

                overlaps_to_eor_diffs.clear();
            } else {
                is_last_seed = true;
            }
        }
    }

    // Get the lowest value from location_numbers vec
    let min_value = location_numbers.iter().min().unwrap();
    Ok(*min_value as isize)
}

pub fn puzzle(input_data: &str) -> Result<isize, PuzzleErr> {
    calculate_total(input_data)
}

pub fn main(data_dir: &str) {
    // open file with raw text

    let data = load(data_dir, 052, None);
    // declare total to add calibration parameters to

    let answer = puzzle(&data);
    match answer {
        Ok(x) => println!(" Puzzle 5-2: {}", x),
        Err(e) => panic!("No solution to puzzle: {}.", e),
    }
    assert_eq!(answer, Ok(69841803));
}
