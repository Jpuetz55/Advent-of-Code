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
fn parse_input(input: &str) -> Vec<Vec<usize>> {
    let mut maps = Vec::new();
    let input_to_vector: Vec<&str> = input.lines().collect();
    let mut current_map: Vec<usize> = Vec::new();
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
    //parse the seed numbers and put them in a vector
    let first_line: Option<&str> = params_string.lines().next();
    let mut seed_numbers: Vec<&str> = Vec::new();

    // Print the file content for debugging
    println!("{}", params_string);
    if let Some(line) = first_line {
        let parts: Vec<&str> = line.split(":").collect();
        // Now you can use the parts vector for further processing
        // ...
        seed_numbers = parts[1].split_whitespace().collect::<Vec<&str>>();
    } else {
        // Handle the case where the string is empty
        // ...
    }

    println!("Seed Numbers: {:?}", seed_numbers);

    // Parse the input into a vector of 2D arrays
    let maps = parse_input(&params_string);
    // Iterate through all maps and print their information
    for (index, map) in maps.iter().enumerate() {
        // Iterate through lines within the current map
        for (line_index, line_values) in map.iter().enumerate() {
            // Access individual values in the line without dereferencing
            let destination_start = *line_values;
            // For the next two lines, we'll need to adjust depending on the length of each line
            let source_start = *line_values; // Example: Assuming the line has at least 2 values
            let range_length = *line_values; // Example: Assuming the line has at least 3 values
            // Print values from the current line
            println!(
                "Map {}: Line {}: Destination Start: {}, Source Start: {}, Range Length: {}",
                index + 1,
                line_index + 1,
                destination_start,
                source_start,
                range_length
            );
            // Additional logic to map the seed numbers through the chain
            //Starting at the first line of the map
            //add RL to SRS and check if SN falls into the range (SRS - STS + RL)
            //if true
            //Find the distance between the SRS and the SN
            //Add above value to the DS to find the SN for the next map
            //go straight to the next map
            //if false
            //go to next line in map
        }
    }
}
