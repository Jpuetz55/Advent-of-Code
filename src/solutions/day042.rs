use crate::data::load;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Could not locate digit.")]
    NoDigits,
}

// Define a Card struct
#[derive(Debug)]
struct Card {
    value: String,
    index: usize,
}

fn calculate_total(input_data: &str) -> Result<isize, PuzzleErr> {
    // Split the file content into lines
    let lines: Vec<&str> = input_data.lines().collect();
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

    Ok(overall_total as isize)
}

//this function takes references to the lines arr, a card, the overall_total, and the current_index for state management during recursion
fn process_card(lines: &Vec<Card>, card: &Card, overall_total: &mut u32) {
    let parts: Vec<&str> = card.value.split(":").collect();
    let mut win_count = 0;

    // Split the string into winning and held numbers
    let all_numbers = parts[1].split("|").collect::<Vec<&str>>();
    let winning_numbers = all_numbers[0].split_whitespace().collect::<Vec<&str>>();
    let held_numbers = all_numbers[1].split_whitespace().collect::<Vec<&str>>();

    // Convert strings to vectors of u32
    let winning_numbers: Vec<u32> = winning_numbers
        .iter()
        .map(|s| s.parse().unwrap_or(0))
        .collect();
    let held_numbers: Vec<u32> = held_numbers
        .iter()
        .map(|s| s.parse().unwrap_or(0))
        .collect();
    //need to count the amount of total card

    // Check if held numbers are winning numbers and calculate points
    for number in &held_numbers {
        if winning_numbers.contains(number) {
            win_count += 1;
        }
    }
    if win_count == 0 {
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
        if card.index + (win_count as usize) >= lines.len() {
            win_count = (lines.len() as u32) - (card.index as u32) - 1;
        }
        *overall_total += win_count;
        let copy_arr: Vec<&Card> = (card.index + 1..=card.index + (win_count as usize))
            .map(|i| &lines[i])
            .collect();

        for copy_card in copy_arr {
            process_card(lines, copy_card, overall_total);
        }
    }
}

pub fn puzzle(input_data: &str) -> Result<isize, PuzzleErr> {
    calculate_total(input_data)
}

pub fn main(data_dir: &str) {
    // open file with raw text

    let data = load(data_dir, 042, None);
    // declare total to add calibration parameters to

    let answer = puzzle(&data);
    match answer {
        Ok(x) => println!(" Puzzle 4-2: {}", x),
        Err(e) => panic!("No solution to puzzle: {}.", e),
    }
    assert_eq!(answer, Ok(6050769));
}
