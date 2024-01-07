/*--- Day 6 - 2: Wait For It ---
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

Determine the number of ways you could beat the record in each race. What do you get if you multiply these numbers together?

--- Part Two ---
As the race is about to start, you realize the piece of paper with race times and record distances you got earlier actually just has very bad kerning. There's really only one race - ignore the spaces between the numbers on each line.

So, the example from before:

Time:      7  15   30
Distance:  9  40  200
...now instead means this:

Time:      71530
Distance:  940200
Now, you have to figure out how many ways there are to win this single race. In this example, the race lasts for 71530 milliseconds and the record distance you need to beat is 940200 millimeters. You could hold the button anywhere from 14 to 71516 milliseconds and beat the record, a total of 71503 ways!

How many ways can you beat the record in this one much longer race?
*/

use std::fs::File;
use std::io::{ self, Read };
use std::path::Path;

// Function to parse a single map and return a tuple of vectors for time and distance
fn parse_map(map_str: &str) -> (Vec<usize>, Vec<usize>) {
    let mut iter = map_str.split_whitespace().map(|num_str| num_str.parse().unwrap());

    let time_values: Vec<usize> = iter
        .by_ref()
        .take_while(|_| true)
        .collect();
    let distance_values: Vec<usize> = iter.collect();

    (time_values, distance_values)
}

// Function to parse the entire input and return a vector of tuples for time and distance
fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let mut lines = input.lines();
    let time_line = lines.next().unwrap();
    let record_line = lines.next().unwrap();

    let time_parts: Vec<&str> = time_line.split(":").collect();
    let time_values: usize = time_parts[1].replace(" ", "").parse().unwrap();

    let record_parts: Vec<&str> = record_line.split(":").collect();
    let record_values: usize = record_parts[1].replace(" ", "").parse().unwrap();

    let game_vec: Vec<(usize, usize)> = time_values
        .iter()
        .zip(record_values.iter())
        .map(|(&time, &record)| (time, record))
        .collect();

    println!("Parsed input: {:?}", game_vec);

    game_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hold_time_iterations() {
        let input = "Time: 7 15 30\nDistance: 9 40 200";
        let races = parse_input(input);

        for &(t, _) in &races {
            let mut iterations = 0;

            for _ in 1..t {
                iterations += 1;
            }

            assert_eq!(iterations, t - 1);
        }
    }
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

    println!("Read input string: {:?}", params_string);

    // Parse the input string to get the vector of tuples for time and distance
    let races = parse_input(&params_string);

    // Iterate over races to calculate the number of ways to beat the record
    let result: usize = races
        .iter()
        .map(|&(t, record)| {
            let mut ways_to_beat_record = 0;

            for hold_time in 1..t {
                let velocity = hold_time;
                let roll_time = t - velocity;
                let distance = velocity * roll_time;

                println!(
                    "Time: {}, Record: {}, Hold Time: {}, Velocity: {}, Roll Time: {}, Distance: {}, Ways to Beat Record: {}",
                    t,
                    record,
                    hold_time,
                    velocity,
                    roll_time,
                    distance,
                    ways_to_beat_record
                );

                if distance > record {
                    ways_to_beat_record += 1;
                    println!("Updated Ways to Beat Record: {}", ways_to_beat_record);
                }
            }

            ways_to_beat_record
        })
        .product();

    // Print the result
    println!("The answer is: {}", result);
}
