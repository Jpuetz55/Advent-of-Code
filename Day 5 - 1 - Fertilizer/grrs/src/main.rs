use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// Function to parse a single map and return a 2D array
fn parse_map(map_str: &str) -> Vec<usize> {
    map_str
        .split_whitespace()
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

// Function to parse the entire input and return a vector of 2D arrays
fn parse_input(input: &str) -> Vec<Vec<Vec<usize>>> {
    let mut maps = Vec::new();
    let input_to_vector: Vec<&str> = input.lines().collect();
    let mut current_map: Vec<Vec<usize>> = Vec::new();

    for line in input_to_vector {
        if !line.is_empty() && line.chars().nth(0).unwrap().is_digit(10) {
            // Parse the line and add to the current map
            current_map.push(parse_map(line));
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

fn main() {
    // Read parameters from a file
    let path = Path::new("./params.txt");
    let display = path.display();
    let mut params_file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file content into a string
    let mut params_string = String::new();
    match params_file.read_to_string(&mut params_string) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    }

    // Parse the seed numbers and put them in a vector
    let first_line: Option<&str> = params_string.lines().next();
    let mut seed_numbers: Vec<usize> = Vec::new();
    let mut location_numbers: Vec<usize> = Vec::new();

    if let Some(line) = first_line {
        let parts: Vec<&str> = line.split(":").collect();
        seed_numbers = parts[1]
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
    }

    println!("Seed Numbers: {:?}", seed_numbers);

    // Parse the input into a vector of 2D arrays
    let maps = parse_input(&params_string);

    // Iterate through all maps and print their information
    for (index, map) in maps.iter().enumerate() {
        // Iterate through lines within the current map
        for (line_index, line_values) in map.iter().enumerate() {
            // Access individual values in the line without dereferencing
            let destination_start = line_values[0];
            // For the next two lines, we'll need to adjust depending on the length of each line
            let source_start = line_values[1]; // Example: Assuming the line has at least 2 values
            let range_length = line_values[2]; // Example: Assuming the line has at least 3 values

            // Print values from the current line
            println!(
                "Map {}: Line {}: Destination Start: {}, Source Start: {}, Range Length: {}",
                index + 1,
                line_index + 1,
                destination_start,
                source_start,
                range_length
            );

            // Starting at the first line of the map
            // Iterate through the next maps
            // Need to create a loop for the lines inside each map

            // Need a mutable variable to store the initial seed number
            // and have it update with the value derived from each iteration through a map
            for seed in seed_numbers.iter() {
                let mut loop_seed = *seed;
                for (map_index, map) in maps.iter().enumerate() {
                    for (line_index, line_values) in map.iter().enumerate() {
                        // Calculate the seed range start and end in the current map
                        // Access individual values in the line without dereferencing
                        let destination_start = line_values[0];
                        // For the next two lines, we'll need to adjust depending on the length of each line
                        let source_start = line_values[1]; // Example: Assuming the line has at least 2 values
                        let range_length = line_values[2]; // Example: Assuming the line has at least 3 values
                        // Check if the destination seed falls within the range
                        if loop_seed >= source_start && loop_seed <= source_start + range_length {
                            // Calculate the distance between the source seed in the current map and the destination seed
                            let distance = loop_seed - source_start;
                            loop_seed = destination_start + distance;
                            // Print debugging information
                            println!(
                                "Updated Loop Seed: {} (Map: {}, Line: {})",
                                loop_seed,
                                map_index + 1,
                                line_index + 1
                            );

                            // Go straight to the next map
                            break;
                        }
                    }

                    // If the destination seed is not found in the current map, go to the next line in the map
                }
                //push the loop_seed at the end of the loop. this is the location number for the
                //intial seed value
                location_numbers.push(loop_seed);
            }
        }
    }

    // Get the lowest value
    if let Some(min_value) = location_numbers.iter().min() {
        println!("Minimum value: {}", min_value);
    }
}
