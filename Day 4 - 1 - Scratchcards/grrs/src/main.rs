// ---------------------------------------------------------------- Day 4-1, Scratchcards ----------------------------------------------------------------

/* For example:

Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
In the above example, card 1 has five winning numbers (41, 48, 83, 86, and 17) and eight numbers you have (83, 86, 6, 31, 17, 9, 48, and 53). Of the numbers you have, four of them (48, 83, 17, and 86) are winning numbers! That means card 1 is worth 8 points (1 for the first match, then doubled three times for each of the three matches after the first).

Card 2 has two winning numbers (32 and 61), so it is worth 2 points.
Card 3 has two winning numbers (1 and 21), so it is worth 2 points.
Card 4 has one winning number (84), so it is worth 1 point.
Card 5 has no winning numbers, so it is worth no points.
Card 6 has no winning numbers, so it is worth no points.
So, in this example, the Elf's pile of scratchcards is worth 13 points.

Take a seat in the large pile of colorful cards. How many points are they worth in total? */

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
            
        

fn main() {
    //for each card 
    //parse numbers out of string and put them in separate arrays. One for the winning numbers and one for the numbers that you hold 
        //split string at the colon and take the second half
        //now split second half at the | and keep both arrays
        //now split these two arrays at the space
        //convert them into integers
        //put them into two u32 vectors
            //Once arrays are populated, loop over each number the in held_numbers array and check if it is in the winning numbers array
            //set card_total to zero
            //for number in heldArray        
                //if number is in winning numbers 
                    //if first match
                        //add 1 to the card_total
                    //any subsequent match
                        //multiply card_total by two
                //else do nothing and go to next number in held_numbers.                           
            //end for loop
            //add card_total to overall_total
    //end for loop

    //print overall_total

    //end
    let path = Path::new("./params.txt");
    let display = path.display();
    let mut scratchcards_params_file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // // write opened filed to string

    let mut scratchcards_params_string = String::new();
    match scratchcards_params_file.read_to_string(&mut scratchcards_params_string) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    }

    let lines: Vec<&str> = scratchcards_params_string.lines().collect();
    let mut overall_total = 0;

    for card in lines {
        let parts: Vec<&str> = card.split(":").collect();
        if parts.len() != 2 {
            // Skip invalid cards
            continue;
        }

        let held_numbers_str = parts[1].split("|").collect::<Vec<&str>>();
        if held_numbers_str.len() != 2 {
            // Skip invalid cards
            continue;
        }

        let winning_numbers_str = held_numbers_str[0].split_whitespace().collect::<Vec<&str>>();
        let held_numbers_str = held_numbers_str[1].split_whitespace().collect::<Vec<&str>>();

        let winning_numbers: Vec<u32> = winning_numbers_str.iter().map(|s| s.parse().unwrap_or(0)).collect();
        let held_numbers: Vec<u32> = held_numbers_str.iter().map(|s| s.parse().unwrap_or(0)).collect();

        let mut card_total = 0;

        for number in &held_numbers {
            if winning_numbers.contains(number) {
                if let Some(index) = winning_numbers.iter().position(|&x| x == *number) {
                    if index == 0 {
                        card_total += 1;
                    } else {
                        card_total *= 2;
                    }
                }
            }
        }

        overall_total += card_total;
    }

    println!("Overall Total: {}", overall_total);
}
