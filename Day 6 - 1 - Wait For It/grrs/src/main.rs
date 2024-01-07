                                                                                /*--- Day 6: Wait For It ---
The ferry quickly brings you across Island Island. After asking around, you discover that there is indeed normally a large pile of sand somewhere near here, but you don't see anything besides lots of water and the small island where the ferry has docked.

As you try to figure out what to do next, you notice a poster on a wall near the ferry dock. "Boat races! Open to the public! Grand prize is an all-expenses-paid trip to Desert Island!" That must be where the sand comes from! Best of all, the boat races are starting in just a few minutes.

You manage to sign up as a competitor in the boat races just in time. The organizer explains that it's not really a traditional race - instead, you will get a fixed amount of time during which your boat has to travel as far as it can, and you win if your boat goes the farthest.

As part of signing up, you get a sheet of paper (your puzzle input) that lists the time allowed for each race and also the best distance ever recorded in that race. To guarantee you win the grand prize, you need to make sure you go farther in each race than the current record holder.

The organizer brings you over to the area where the boat races are held. The boats are much smaller than you expected - they're actually toy boats, each with a big button on top. Holding down the button charges the boat, and releasing the button allows the boat to move. Boats move faster if their button was held longer, but time spent holding the button counts against the total race time. You can only hold the button at the start of the race, and boats don't move until the button is released.

For example:

Time:      7  15   30
Distance:  9  40  200
This document describes three races:

The first race lasts 7 milliseconds. The record distance in this race is 9 millimeters.
The second race lasts 15 milliseconds. The record distance in this race is 40 millimeters.
The third race lasts 30 milliseconds. The record distance in this race is 200 millimeters.
Your toy boat has a starting speed of zero millimeters per millisecond. For each whole millisecond you spend at the beginning of the race holding down the button, the boat's speed increases by one millimeter per millisecond.

So, because the first race lasts 7 milliseconds, you only have a few options:

Don't hold the button at all (that is, hold it for 0 milliseconds) at the start of the race. The boat won't move; it will have traveled 0 millimeters by the end of the race.
Hold the button for 1 millisecond at the start of the race. Then, the boat will travel at a speed of 1 millimeter per millisecond for 6 milliseconds, reaching a total distance traveled of 6 millimeters.
Hold the button for 2 milliseconds, giving the boat a speed of 2 millimeters per millisecond. It will then get 5 milliseconds to move, reaching a total distance of 10 millimeters.
Hold the button for 3 milliseconds. After its remaining 4 milliseconds of travel time, the boat will have gone 12 millimeters.
Hold the button for 4 milliseconds. After its remaining 3 milliseconds of travel time, the boat will have gone 12 millimeters.
Hold the button for 5 milliseconds, causing the boat to travel a total of 10 millimeters.
Hold the button for 6 milliseconds, causing the boat to travel a total of 6 millimeters.
Hold the button for 7 milliseconds. That's the entire duration of the race. You never let go of the button. The boat can't move until you let go of the button. Please make sure you let go of the button so the boat gets to move. 0 millimeters.
Since the current record for this race is 9 millimeters, there are actually 4 different ways you could win: you could hold the button for 2, 3, 4, or 5 milliseconds at the start of the race.

In the second race, you could hold the button for at least 4 milliseconds and at most 11 milliseconds and beat the record, a total of 8 different ways to win.

In the third race, you could hold the button for at least 11 milliseconds and no more than 19 milliseconds and still beat the record, a total of 9 ways you could win.

To see how much margin of error you have, determine the number of ways you can beat the record in each race; in this example, if you multiply these values together, you get 288 (4 * 8 * 9).

Determine the number of ways you could beat the record in each race. What do you get if you multiply these numbers together?*/

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

    // Parse the seed numbers and put them in a vector of ranges
    let first_line: Option<&str> = params_string.lines().next();
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

    println!("Seed Ranges: {:?}", seed_ranges);

    //... (Code to find and calculate seed values, explained in comments below)

    // Parse the input into a vector of 2D arrays
    let maps = parse_input(&params_string);

    // Iterate through all maps and print their information

    // Starting at the first line of the map
    // Iterate through the next maps
    // Need to create a loop for the lines inside each map

    // Need a mutable variable to store the initial seed number
    // and have it update with the value derived from each iteration through a map
    for seed_range in seed_ranges.iter() {
        println!("Seed Range: ({}, {})", seed_range.0, seed_range.1);

        let mut is_last_seed = false;
        let mut seed_initial = seed_range.0;
        let mut seed = seed_range.0;
        let mut check_overlap: i32;
        let mut seed_range_length = seed_range.1;
        let mut overlaps_to_eor_diffs: Vec<usize> = Vec::new();

        while !is_last_seed {
            seed = seed_initial;
            println!("  Current Seed: {}", seed);

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
                        println!(
                            "  Updated Loop Seed: {} (Map: {}, Line: {})",
                            seed,
                            map_index + 1,
                            line_index + 1
                        );

                        check_overlap = (seed_range_length as i32) - (range_length as i32);
                        if check_overlap > 0 {
                            overlaps_to_eor_diffs.push(range_length as usize);
                            println!("Qualified Overlap: {}", check_overlap);
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
            println!("  Pushed {} ----\n {}", seed, seed_range.0);
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
                println!("overlaps_to_eor_diff: {}", overlaps_to_eor_diff);
                //seed_range_length = it's length - minus the difference between it's length and the max overlap value
                //will correctly adjust the seed_range_length for the next iteration
                //next seed index will use the same value to calculate the next seed
                //bug has something to do with when range_length = distance, I think
                seed_initial = seed_initial + *overlaps_to_eor_diff + 1;
                println!("Seed Initial: {}", seed_initial);
                seed_range_length = seed_range_length - *overlaps_to_eor_diff;

                println!("Seed Range Length: {}", seed_range_length);
                overlaps_to_eor_diffs.clear();
            } else {
                is_last_seed = true;
            }
        }
    }

    // Get the lowest value from location_numbers vec
    if let Some(min_value) = location_numbers.iter().min() {
        println!("Location Numbers Length: {:?}", location_numbers.len());
        println!("Minimum value: {}", min_value);
    }
}
