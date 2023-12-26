// ---------------------------------------------------------------- Day 4-2, Scratchcards ----------------------------------------------------------------

/* This time, the above example goes differently:

Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

Card 1 has four matching numbers, so you win one copy each of the next four cards: cards 2, 3, 4, and 5.
Your original card 2 has two matching numbers, so you win one copy each of cards 3 and 4.
Your copy of card 2 also wins one copy each of cards 3 and 4.
Your four instances of card 3 (one original and three copies) have two matching numbers, 
so you win four copies each of cards 4 and 5. Your eight instances of card 4 (one original 
and seven copies) have one matching number, so you win eight copies of card 5.
Your fourteen instances of card 5 (one original and thirteen copies) have no matching numbers 
and win no more cards.Your one instance of card 6 (one original) has no matching numbers and 
wins no more cards. Once all of the originals and copies have been processed, you end up with 
1 instance of card 1, 2 instances of card 2, 4 instances of card 3, 8 instances of card 4, 
14 instances of card 5, and 1 instance of card 6. In total, this example pile of scratchcards 
causes you to ultimately have 30 scratchcards!

Process all of the original and copied scratchcards until no more scratchcards are won. Including the original set of scratchcards, how many total scratchcards do you end up with? */

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;


//this function takes references to the lines arr, a card, the overall_total, and the current_index for state management during recursion
fn process_card(lines: &Vec<&str>, card: &str, overall_total: &u32) {
    let parts: Vec<&str> = card.split(":").collect();
    let win_count = 0;

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
    //need to count the amount of total card


    // Check if held numbers are winning numbers and calculate points
    for number in &held_numbers {
        if winning_numbers.contains(number) {
            win_count += 1;
        }
    }
    if win_count == 0 {
        //base case - no more winning numbers in card
        //exit function and go back up the stack
    }
    else {
        // Make an array with all the copies to be added and iterate through it,
        // calling process_card on each one
        overall_total += win_count;
        let copy_arr: Vec<&str> = (current_index + 1..lines.len()).map(|i| lines[i]).collect();
        for copy_card in copy_arr {
            process_card(lines, copy_card, overall_total, current_index);
        }
    }
    

    
}

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
    let mut indexed_lines: HashMap<usize, &str> = HashMap::new();
    //create hash map with index value for each card for state management during recursion. zero-based index
    for (index, card) in lines.iter().enumerate() {
        indexed_lines.insert(index, *card);
    }
    //start count wtih the number of original cards and add copies to the total in process_card func
    let mut overall_total = lines.len();

    // Process each scratchcard
    for card in lines {
    }

    // Print the overall total
    println!("Overall Total: {}", overall_total);
}
