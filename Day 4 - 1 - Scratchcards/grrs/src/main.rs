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
    // Read scratchcards parameters from a file
    let path = Path::new("./params.txt");
    let display = path.display();
    let mut scratchcards_params_file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file content into a string
    let mut scratchcards_params_string = String::new();
    match scratchcards_params_file.read_to_string(&mut scratchcards_params_string) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    }

    // Split the file content into lines
    let lines: Vec<&str> = scratchcards_params_string.lines().collect();
    let mut overall_total = 0;

    // Process each scratchcard
    for card in lines {
        let parts: Vec<&str> = card.split(":").collect();

        // Split the string into winning and held numbers
        let held_numbers = parts[1].split("|").collect::<Vec<&str>>();
        let winning_numbers = held_numbers[0].split_whitespace().collect::<Vec<&str>>();
        let held_numbers = held_numbers[1].split_whitespace().collect::<Vec<&str>>();

        // Convert strings to vectors of u32
        let winning_numbers: Vec<u32> = winning_numbers
            .iter()
            .map(|s| s.parse().unwrap_or(0))
            .collect();
        let held_numbers: Vec<u32> = held_numbers
            .iter()
            .map(|s| s.parse().unwrap_or(0))
            .collect();

        println!("held numbers: {:?}\nwinning numbers: {:?}", held_numbers, winning_numbers);

        let mut card_total = 0;
        let mut is_first = true;

        // Check if held numbers are winning numbers and calculate points
        for number in &held_numbers {
            if winning_numbers.contains(number) {
                if is_first {
                    card_total += 1;
                    is_first = false;
                    println!(
                        "held numbers: {:?}\nwinning numbers: {:?}",
                        held_numbers,
                        winning_numbers
                    );
                } else {
                    card_total *= 2;
                }
            }
        }

        // Add the card total to the overall total
        overall_total += card_total;
    }

    // Print the overall total
    println!("Overall Total: {}", overall_total);
}
