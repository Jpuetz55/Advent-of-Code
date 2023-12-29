use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// Function to parse a string representation of a map and return a HashMap
// The map format is "dest_start source_start range_length"
fn parse_map(map_str: &str) -> HashMap<usize, (usize, usize)> {
    // Initialize an empty HashMap to store the mapping
    let mut map = HashMap::new();
    // Split the input string into numbers and create an iterator
    let mut iter = map_str.split_whitespace().map(|num_str| num_str.parse().unwrap());

    // Iterate over the numbers in groups of three, representing dest_start, source_start, and range_length
    while let Some(dest_start) = iter.next() {
        let source_start = iter.next().unwrap();
        let range_length = iter.next().unwrap();

        // Output the parsed values for debugging
        println!(
            "dest_start: {}, source_start: {}, range_length: {}",
            dest_start,
            source_start,
            range_length
        );

        // Create a mapping for each value in the specified range

        let dest_number = dest_start;
        let source_number = source_start;
        // Insert the mapping into the HashMap
        map.insert(source_number, (dest_number, source_number));
    }

    // Output the final parsed map for debugging
    println!("Parsed Map: {:?}", map);
    map
}

// Function to parse the entire input and return a map of maps
fn parse_input(input: &str) -> HashMap<String, HashMap<usize, (usize, usize)>> {
    // Initialize an empty HashMap to store category maps
    let mut maps = HashMap::new();
    // Initialize an empty HashMap to store the current map being processed
    let mut current_map = HashMap::new();
    // Create an iterator over the lines of the input
    let mut iter = input.lines();

    // Iterate over the lines in the input
    while let Some(line) = iter.next() {
        // Check if the line is not empty and starts with a digit
        if !line.is_empty() && line.chars().nth(0).unwrap().is_digit(10) {
            // Parse the line and add the map to the current map
            current_map = parse_map(line)
                .into_iter()
                .map(|(key, value)| (key, value.clone()))
                .collect();
            // Check if the line is empty and the current map is not empty
        } else if line.is_empty() && !current_map.is_empty() {
            // Add the current map to the overall maps
            let category_name = current_map.remove(&0).unwrap().0.to_string();
            maps.insert(category_name.clone(), current_map.clone());
            println!("Added Map for Category: {}", category_name);
            // Clear the current map for the next iteration
            current_map.clear();
        }
    }

    // Add the last map if it's not empty
    if !current_map.is_empty() {
        let category_name = current_map.remove(&0).unwrap().0.to_string();
        maps.insert(category_name.clone(), current_map);
        println!("Added Last Map for Category: {}", category_name);
    }

    // Output the final parsed maps for debugging
    println!("Parsed Input Maps: {:?}", maps);
    maps
}

// Function to convert seed numbers to location numbers
fn convert_seed_to_location(
    seed_numbers: &[&str],
    mappings: &HashMap<String, HashMap<usize, (usize, usize)>>
) -> usize {
    // Initialize a HashMap to store the seed-to-location mapping
    let mut seed_to_location: HashMap<usize, usize> = HashMap::new();

    // Initialize initial seed numbers
    for (index, seed_number_str) in seed_numbers.iter().enumerate() {
        let seed_number: usize = seed_number_str.parse().unwrap();
        // Insert the seed-to-location mapping, assuming location numbers start from 1
        seed_to_location.insert(seed_number, index + 1);
    }

    // Define categories in a specific order
    let categories = vec![
        "seed",
        "soil",
        "fertilizer",
        "water",
        "light",
        "temperature",
        "humidity",
        "location"
    ];

    // Iterate through the categories, skipping the "seed" category
    for category in categories.iter().skip(1) {
        // Skip "seed" category since we start with seed numbers
        if let Some(mapping) = mappings.get(*category) {
            // Temporary mapping for the current category
            let mut new_seed_to_location = seed_to_location.clone();

            // Iterate through seed numbers
            for (seed_number, &location) in seed_to_location.iter() {
                // If the seed number is not in the mapping, it corresponds to the same location number
                let dest_number = mapping
                    .get(seed_number)
                    .map(|&t| t.0)
                    .unwrap_or(location);
                // Update the seed-to-location mapping for the next iteration
                new_seed_to_location.insert(*seed_number, dest_number);
            }

            // Update the seed-to-location mapping for the next iteration
            seed_to_location = new_seed_to_location;
        }
    }

    // Output the final seed-to-location mapping for debugging
    println!("Seed to Location Mapping: {:?}", seed_to_location);
    // Find the lowest location number
    *seed_to_location.values().min().unwrap_or(&0)
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

    // Check if the first line is present
    if let Some(line) = first_line {
        let parts: Vec<&str> = line.split(":").collect();
        // Now you can use the parts vector for further processing
        // Split the seed numbers and collect them into a vector
        seed_numbers = parts[1].split_whitespace().collect::<Vec<&str>>();
    } else {
        // Handle the case where the string is empty
        // ...
    }

    // Output the parsed seed numbers for debugging
    println!("Seed Numbers: {:?}", seed_numbers);

    // Parse the input into a map of maps
    let mappings = parse_input(&params_string);

    // Convert seed numbers to location numbers and print the result
    let lowest_location = convert_seed_to_location(&seed_numbers, &mappings);
    println!("Lowest Location Number: {:?}", lowest_location);
}
