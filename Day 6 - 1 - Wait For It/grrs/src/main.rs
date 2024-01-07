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



velocity = hold_time

distance = velocity * (time - velocity)

can ignore first and last value of time range ( 0 and max ) as both produce 0 distance

put time values and distance values in tuple vector
make a vector game_vec (time, distance_record, win_count)

iterate over the range of valid time values (i.e. excluding 0 and time.max())
for each value game_vec
    iterate over (0..i, game_vec.0)
        in each iteration, calculate ---- distance = velocity * (game_vec.0 - velocity) ---- and check
            iteration values:
            velocity = i; //hold_time
            roll_time = game_vec.0 - velocity
            win_count = 0
            distance = velocity * roll_time
            if distance > game_vec.1
                win_count++
            else
                go next
   game_vec.2 = win_count

after finding wins for each game

multiply the 3 win_count values in in the game_vec to get the answer


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

    // Function to calculate distance based on velocity and roll time
    fn calculate_distance(velocity: f64, roll_time: f64) -> f64 {
        velocity * roll_time
    }

    fn main() {
        // Define the maximum time value; replace with your specific value
        let max_time = 10.0;

        // Generate a vector of valid time values, excluding 0 and max_time
        let time_values: Vec<f64> = (1..max_time as usize).map(|i| i as f64).collect();

        // Initialize the game vector to store time, distance record, and win count
        let mut game_vec: Vec<(f64, f64, usize)> = Vec::new();

        // Iterate over the range of valid time values
        for &t in &time_values {
            let mut win_count = 0;
            let mut distance_record = 0.0;

            // Iterate over the range (0..t) to calculate distance and check win conditions
            for i in 1..t as usize {
                let velocity = i as f64;
                let roll_time = t - velocity;

                // Calculate distance based on velocity and roll time
                let distance = calculate_distance(velocity, roll_time);

                // Check if the calculated distance is greater than the distance record
                if distance > distance_record {
                    win_count += 1;
                }
            }

            // Store the time, distance record, and win count in game_vec
            game_vec.push((t, distance_record, win_count));
        }

        // Calculate the final answer by multiplying all win counts in game_vec
        let answer: usize = game_vec.iter().map(|&(_, _, win_count)| win_count).product();

        // Print the final answer
        println!("The answer is: {}", answer);
    }




}
