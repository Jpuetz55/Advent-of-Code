use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// Function to parse a single map and return a 2D array
fn parse_map(map_str: &str) -> Vec<usize> {
    map_str
        .lines()
        .flat_map(|line| line.split_whitespace().map(|num_str| num_str.parse().unwrap()))
        .collect()
}

// Function to parse the entire input and return a vector of 2D arrays
fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .split("\n\n") // Assume maps are separated by double newlines
        .filter(|map_str| !map_str.is_empty())
        .map(|map_str| parse_map(map_str))
        .collect()
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

    // Split the file content into lines
    let lines: Vec<&str> = params_string.lines().collect();

    // Print the file content for debugging
    println!("{}", params_string);

    // Parse the input into a vector of 2D arrays
    let maps = parse_input(&params_string);

    // Accessing the first map in the seed-to-soil category
    let seed_to_soil_map = &maps[0];

    // Access individual values in the map without dereferencing
    let destination_start = seed_to_soil_map[0];
    let source_start = seed_to_soil_map[1];
    let range_length = seed_to_soil_map[2];

    // Print values from the first line of the seed-to-soil map
    println!(
        "Destination Start: {}, Source Start: {}, Range Length: {}",
        destination_start,
        source_start,
        range_length
    );

    // Logic to map the seed numbers through the chain
    // ...
}
