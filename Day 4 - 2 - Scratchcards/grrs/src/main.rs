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

// Define a Card struct
#[derive(Debug)]
struct Card {
    value: String,
    index: usize,
}

//this function takes references to the lines arr, a card, the overall_total, and the current_index for state management during recursion
fn process_card(lines: &Vec<Card>, card: &Card, overall_total: &mut u32) {
    let parts: Vec<&str> = card.value.split(":").collect();
    let mut win_count = 0;

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
    if win_count == 0 || card.index + (win_count as usize) >= lines.len() {
        //potential problem here. if the number of copies to be
        //made is greater than the number of cards left in the deck
        //then the cards that are left at the end will be thrown out
        //base case - no more winning numbers in card, ergo no more copies
        //exit function and go back up the stack
        return;
    } else {
        // Make an array with all the copies to be added and iterate through it,
        // calling process_card on each one
        //I believe this is passing the same card to the copy array over and over again
        //further debugging is needed to make a determination
        //if win count is greater than the number of cards remaining, then add only the cards that are remaining as copies
        *overall_total += win_count;
        let copy_arr: Vec<&Card> = (card.index + 1..(win_count + 1) as usize)
            .map(|i| &lines[i])
            .collect();

        for copy_card in copy_arr {
            process_card(lines, copy_card, overall_total);
        }
    }
}

fn main() {
    //Read scratchcards parameters from a file
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

    //     let scratchcards_params_string_test =
    //         "Card   1: 99 46 62 92 60 37 52 56 41 31 | 83 40 31 33 46  3 10 39 82  8 64 35  5 63 60 72 48 87 11 81 95 34 97 37 99
    // Card   2: 98 96 50 60  7 40 83 93 55 26 | 45 38 47 98 32 50 55 35 93 11 97 53 74 83 99 60 73 56 40 58 96 66 90 26  7
    // Card   3: 82  8 12 15 53 23 29 61  5 21 | 21 73  5 65 44 29 61 97 15  4 90 76 53 91 13 27  9 11  2 75 22 92 95 82 86
    // Card   4: 68 22 77 52 23 60 57 31 74 38 | 22 38 68 79 52 23 40 57 10 74 31 83 24 60 95 17 78 89 39 37 87 26 77 63 54
    // Card   5: 94 23 38 14 31 47  8 15 19 79 | 94 45 21 70 43 95 79 38 19 67 24 62 71 84 14 87 63 47 36 26  8 66 31 15 23
    // Card   6: 37 56 14 26 46 19 58 99 41 55 | 95 25 39 19 26 99 21 11 56 46  3 53 33 36  4 15 55 28 58 37 14 50 94 49 44
    // Card   7: 68 13 44 81 92 43  9 78 85 62 | 19  8 91 71 76 54 72 87 68 16 46 94 92 63 62 67 28 84 18 88 24 14 37 30 81
    // Card   8: 73 86 24 66 38 95 71 90 88 22 | 44 88 30 24 97 34 99 66 94 26 86 90 52 55 84  7 78 40 71 73 50 95 61 38 22
    // Card   9: 98 71 24 85 11 74 22 35 65 59 | 18 43 54 36 71 86 22  7 85 78 46  6 77 62 38 98 83 65 88 37 84 70 72 59 23
    // Card  10:  5 87 77 60 62 86 42 33 43 76 | 86 19 26 77  5 60 21 34 44 76 33 85 78 67 79 13 18 42 87 59 54 43 62 65 16";

    //let lines: Vec<&str> = scratchcards_params_string_test.lines().collect();

    // Split the file content into lines
    let lines: Vec<&str> = scratchcards_params_string.lines().collect();
    // Create a vector of Card structs
    let mut cards: Vec<Card> = Vec::new();
    for (index, &card) in lines.iter().enumerate() {
        cards.push(Card {
            value: card.to_string(),
            index,
        });
    }
    //start count wtih the number of original cards and add copies to the total in process_card func
    let mut overall_total = lines.len() as u32;

    // Process each scratchcard
    for card in &cards {
        process_card(&cards, &card, &mut overall_total);
    }

    // Print the overall total
    println!("Overall Total: {}", overall_total);
}
