/*--- Day 7 - 1: Camel Cards ---
Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an airship. (At least it's a cool airship!) It drops you off at the edge of a vast desert and descends back to Island Island.

"Did you bring the parts?"

You turn around to see an Elf completely covered in white clothing, wearing goggles, and riding a large camel.

"Did you bring the parts?" she asks again, louder this time. You aren't sure what parts she's looking for; you're here to figure out why the sand stopped.

"The parts! For the sand, yes! Come with me; I will show you." She beckons you onto the camel.

After riding a bit across the sands of Desert Island, you can see what look like very large rocks covering half of the horizon. The Elf explains that the rocks are all along the part of Desert Island that is directly above Island Island, making it hard to even get there. Normally, they use big machines to move the rocks and filter the sand, but the machines have broken down because Desert Island recently stopped receiving the parts they need to fix the machines.

You've already assumed it'll be your job to figure out why the parts stopped when she asks if you can help. You agree automatically.

Because the journey will take a few days, she offers to teach you the game of Camel Cards. Camel Cards is sort of similar to poker except it's designed to be easier to play while riding a camel.

In Camel Cards, you get a list of hands, and your goal is to order them based on the strength of each hand. A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2. The relative strength of each card follows this order, where A is the highest and 2 is the lowest.

Every hand is exactly one type. From strongest to weakest, they are:

Five of a kind, where all five cards have the same label: AAAAA
Four of a kind, where four cards have the same label and one card has a different label: AA8AA
Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
High card, where all cards' labels are distinct: 23456
Hands are primarily ordered based on type; for example, every full house is stronger than any three of a kind.

If two hands have the same type, a second ordering rule takes effect. Start by comparing the first card in each hand. If these cards are different, the hand with the stronger first card is considered stronger. If the first card in each hand have the same label, however, then move on to considering the second card in each hand. If they differ, the hand with the higher second card wins; otherwise, continue with the third card in each hand, then the fourth, then the fifth.

So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its first card is stronger. Similarly, 77888 and 77788 are both a full house, but 77888 is stronger because its third card is stronger (and both hands have the same first and second card).

To play Camel Cards, you are given a list of hands and their corresponding bid (your puzzle input). For example:

32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
This example shows five hands; each hand is followed by its bid amount. Each hand wins an amount equal to its bid multiplied by its rank, where the weakest hand gets rank 1, the second-weakest hand gets rank 2, and so on up to the strongest hand. Because there are five hands in this example, the strongest hand will have rank 5 and its bid will be multiplied by 5.

So, the first step is to put the hands in order of strength:

32T3K is the only one pair and the other hands are all a stronger type, so it gets rank 1.
KK677 and KTJJT are both two pair. Their first cards both have the same label, but the second card of KK677 is stronger (K vs T), so KTJJT gets rank 2 and KK677 gets rank 3.
T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first card, so it gets rank 5 and T55J5 gets rank 4.
Now, you can determine the total winnings of this set of hands by adding up the result of multiplying each hand's bid with its rank (765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5). So the total winnings in this example are 6440.

Find the rank of every hand in your set. What are the total winnings?

*/

use std::collections::HashMap;

use crate::data::load;
use thiserror::Error;
use permutator::{ Combination, Permutation };

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Could not locate digit.")]
    NoDigits,
}

// Define a constant array for card rankings
const PERMUTE_CARDS: [char; 12] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A']; // no j

// Function to generate permutations when 'J' appears once in a hand
fn generate_permutations_for_two_j() -> Vec<(char, char)> {
    let mut combinations_fn: Vec<(char, char)> = Vec::new();
    let mut counter = 1;
    PERMUTE_CARDS.combination(2).for_each(|mut c| {
        c.permutation().for_each(|p| {
            combinations_fn.push((*p[0], *p[1]));
            counter += 1;
        });
    });

    // Include combinations of 2 identical cards
    PERMUTE_CARDS.iter().for_each(|&card| {
        combinations_fn.push((card, card));
        counter += 1;
    });
    combinations_fn
}

// Struct to represent a hand entry
#[derive(Debug, Clone)]
struct HandEntry {
    hand: (String, [usize; 13], usize),
    bid: usize,
    overall_rank: usize,
    total_score: usize,
}

// Function to convert card character to sorting rank
fn char_to_rank(card: char) -> u32 {
    match card {
        'A' => 1,
        'K' => 2,
        'Q' => 3,
        'T' => 4,
        '9' => 5,
        '8' => 6,
        '7' => 7,
        '6' => 8,
        '5' => 9,
        '4' => 10,
        '3' => 11,
        '2' => 12,
        'J' => 13,
        _ => panic!("Invalid card"),
    }
}

// Function to sort hands alphabetically within a rank
fn sort_hands_alphabetically_within_rank(hands: &mut Vec<HandEntry>) {
    hands.sort_by(|a, b| {
        for (char_a, char_b) in a.hand.0.chars().zip(b.hand.0.chars()) {
            let rank_a = char_to_rank(char_a);
            let rank_b = char_to_rank(char_b);

            match rank_a.cmp(&rank_b) {
                std::cmp::Ordering::Less => {
                    return std::cmp::Ordering::Less;
                }
                std::cmp::Ordering::Greater => {
                    return std::cmp::Ordering::Greater;
                }
                _ => {
                    continue;
                }
            }
        }
        std::cmp::Ordering::Equal
    });
}

// Function to update overall rank and total score for hands
fn update_overall_rank_and_total_score(sorted_hands: &mut Vec<HandEntry>) {
    for (index, entry) in sorted_hands.iter_mut().enumerate() {
        entry.overall_rank = 1000 - index;
        entry.total_score = entry.bid * entry.overall_rank;
    }
}

// Function to generate hand arrays when 'J' appears once in a hand
fn generate_hand_array_vec_one_j(hand_array: &[usize; 13]) -> Vec<[usize; 13]> {
    let mut hand_array_vec: Vec<[usize; 13]> = Vec::new();
    for c in PERMUTE_CARDS {
        let mut hand_array_copy = *hand_array;
        match c {
            '2'..='9' => {
                let index = (c as usize) - ('2' as usize);
                hand_array_copy[index] += 1;
            }
            'T' => {
                hand_array_copy[8] += 1; // Index for 'T'
            }
            'Q' => {
                hand_array_copy[10] += 1; // Index for 'Q'
            }
            'K' => {
                hand_array_copy[11] += 1; // Index for 'K'
            }
            'A' => {
                hand_array_copy[12] += 1; // Index for 'A'
            }
            _ => panic!("Invalid card"),
        }
        hand_array_vec.push(hand_array_copy);
    }
    hand_array_vec
}

// Function to generate hand arrays when 'J' appears twice in a hand
fn generate_hand_array_combos_two_j(hand_array: &[usize; 13]) -> Vec<[usize; 13]> {
    let mut hand_array_vec: Vec<[usize; 13]> = Vec::new();
    let combinations: Vec<(char, char)> = generate_permutations_for_two_j();
    for c in combinations {
        let mut hand_array_copy = *hand_array;
        match c.0 {
            '2'..='9' => {
                let index = (c.0 as usize) - ('2' as usize);
                hand_array_copy[index] += 1;
            }
            'T' => {
                hand_array_copy[8] += 1; // Index for 'T'
            }
            'Q' => {
                hand_array_copy[10] += 1; // Index for 'Q'
            }
            'K' => {
                hand_array_copy[11] += 1; // Index for 'K'
            }
            'A' => {
                hand_array_copy[12] += 1; // Index for 'A'
            }
            _ => panic!("Invalid card"),
        }
        match c.1 {
            '2'..='9' => {
                let index = (c.1 as usize) - ('2' as usize);
                hand_array_copy[index] += 1;
            }
            'T' => {
                hand_array_copy[8] += 1; // Index for 'T'
            }
            'Q' => {
                hand_array_copy[10] += 1; // Index for 'Q'
            }
            'K' => {
                hand_array_copy[11] += 1; // Index for 'K'
            }
            'A' => {
                hand_array_copy[12] += 1; // Index for 'A'
            }
            _ => panic!("Invalid card"),
        }
        hand_array_vec.push(hand_array_copy);
    }
    hand_array_vec
}

// Function to get the maximum rank for a hand without 'J'
fn get_max_rank(hand: [usize; 13]) -> usize {
    // Initialize an array to count the number of matches for each bucket size
    let mut matches = [0; 4];

    // Iterate over each bucket in the hand
    for &bucket in hand.iter() {
        // Count matches for buckets with 2, 3, 4, or 5 cards
        if bucket == 2 {
            matches[0] += 1;
        } else if bucket == 3 {
            matches[1] += 1;
        } else if bucket == 4 {
            matches[2] += 1;
        } else if bucket == 5 {
            matches[3] += 1;
        }
    }

    // Determine the maximum rank based on the matches
    match matches {
        [0, 0, 0, 0] => 0,
        [1, 0, 0, 0] => 1,
        [2, 0, 0, 0] => 2,
        [0, 1, 0, 0] => 3,
        [1, 1, 0, 0] => 4,
        [0, 0, 1, 0] => 5,
        [0, 0, 0, 1] => 6,
        _ => panic!("Invalid hand"),
    }
}

// Function to get the maximum rank for a hand with one 'J'
fn get_max_rank_one_j(hand_array: &[usize; 13]) -> usize {
    let mut max_rank = 0;

    // Generate all possible hand arrays with one 'J'
    let hand_array_vec = generate_hand_array_vec_one_j(hand_array);

    // Iterate over each hand array and determine the maximum rank
    for hand_array in hand_array_vec {
        let rank = get_max_rank(hand_array);
        if rank > max_rank {
            max_rank = rank;
        }
    }

    // Return the maximum rank
    max_rank
}

// Function to get the maximum rank for a hand with two 'J's
fn get_max_rank_two_j(hand_array: &[usize; 13]) -> usize {
    let mut max_rank = 0;

    // Generate all possible hand arrays with two 'J's
    let hand_array_vec = generate_hand_array_combos_two_j(hand_array);

    // Iterate over each hand array and determine the maximum rank
    for hand_array in hand_array_vec {
        let rank = get_max_rank(hand_array);
        if rank > max_rank {
            max_rank = rank;
        }
    }

    // Return the maximum rank
    max_rank
}

// Function to get the maximum rank for a hand with three 'J's
fn get_max_rank_three_j(hand_array: &[usize; 13]) -> usize {
    // Check if there are two of any card in the hand
    for item in hand_array {
        if *item == 2 {
            return 6; // Return rank 6 if both cards are the same are present. 5 of a kind is the highest rank possible
        }
    }

    // Return rank 5 if both cards are different. 4 of a kind is the highest rank possible
    5
}

//4 J's is trivial. just make them the other card for 5 of a kind
//Do I need to explain why 5 J's is trivial?

// Function to parse input and create HandEntry instances
fn parse_input(input: &str) -> Vec<HandEntry> {
    let mut hands: Vec<HandEntry> = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let hand_str = iter.next().unwrap();
        let bid = iter.next().unwrap().parse::<usize>().unwrap();
        let mut hand_array: [usize; 13] = [0; 13];

        // Populate hand_array based on card labels
        hand_str.chars().for_each(|c| {
            match c {
                '2' => {
                    hand_array[0] += 1;
                }
                '3' => {
                    hand_array[1] += 1;
                }
                '4' => {
                    hand_array[2] += 1;
                }
                '5' => {
                    hand_array[3] += 1;
                }
                '6' => {
                    hand_array[4] += 1;
                }
                '7' => {
                    hand_array[5] += 1;
                }
                '8' => {
                    hand_array[6] += 1;
                }
                '9' => {
                    hand_array[7] += 1;
                }
                'T' => {
                    hand_array[8] += 1;
                }
                'J' => {
                    hand_array[9] += 1;
                }
                'Q' => {
                    hand_array[10] += 1;
                }
                'K' => {
                    hand_array[11] += 1;
                }
                'A' => {
                    hand_array[12] += 1;
                }
                _ => panic!("Invalid card"),
            }
        });

        // Create HandEntry instance and push to hands vector
        if hand_str.contains('J') {
            let j_count = hand_str
                .chars()
                .filter(|&c| c == 'J')
                .count();

            if j_count == 1 {
                // Remove 'J' from the hand_array
                hand_array[9] = 0;
                hands.push(HandEntry {
                    hand: (hand_str.to_owned(), hand_array, get_max_rank_one_j(&hand_array)),
                    bid,
                    overall_rank: 0,
                    total_score: 0,
                });
            } else if j_count == 2 {
                // Remove 'J' from the hand_array
                hand_array[9] = 0;
                hands.push(HandEntry {
                    hand: (hand_str.to_owned(), hand_array, get_max_rank_two_j(&hand_array)),
                    bid,
                    overall_rank: 0,
                    total_score: 0,
                });
            } else if j_count == 3 {
                // Remove 'J' from the hand_array
                hand_array[9] = 0;
                hands.push(HandEntry {
                    hand: (hand_str.to_owned(), hand_array, get_max_rank_three_j(&hand_array)),
                    bid,
                    overall_rank: 0,
                    total_score: 0,
                });
            } else {
                hands.push(HandEntry {
                    hand: (hand_str.to_owned(), hand_array, 6),
                    bid,
                    overall_rank: 0,
                    total_score: 0,
                });
            }
        } else {
            //no j's, normal processing
            hands.push(HandEntry {
                hand: (hand_str.to_owned(), hand_array, get_max_rank(hand_array)),
                bid,
                overall_rank: 0,
                total_score: 0,
            });
        }
    }

    hands
}

// Main puzzle-solving function
fn puzzle(input_data: &str) -> Result<isize, PuzzleErr> {
    calculate_total(input_data)
}

// Function to calculate total winnings
fn calculate_total(input_data: &str) -> Result<isize, PuzzleErr> {
    // Parse input and create HandEntry instances
    let hands = parse_input(input_data);

    // Create a HashMap to group hands by their rank
    let mut hands_by_rank: HashMap<usize, Vec<HandEntry>> = HashMap::new();
    for entry in hands.iter() {
        hands_by_rank.entry(entry.hand.2).or_insert(Vec::new()).push(entry.clone());
    }

    // Sort hands alphabetically within each rank group
    for (_, hands_list) in hands_by_rank.iter_mut() {
        sort_hands_alphabetically_within_rank(hands_list);
    }
    //This is readable but for optimization, I should just leave them in the hashmap and iterate over them in order for the total score
    // Combine hands from all ranks into a single sorted list
    let mut sorted_hands: Vec<HandEntry> = Vec::new();
    for rank in (0..=6).rev() {
        if let Some(mut hands_list) = hands_by_rank.remove(&rank) {
            sorted_hands.append(&mut hands_list);
        }
    }

    // Update overall rank and total score for each hand based on their index in the new sorted list
    update_overall_rank_and_total_score(&mut sorted_hands);

    // Calculate total winnings
    let mut total = 0;
    for entry in &sorted_hands {
        total += entry.total_score;
    }
    Ok(total as isize)
}

// Main function to execute the puzzle
pub fn main(data_dir: &str) {
    let data = load(data_dir, 072, None);

    let answer = puzzle(&data);
    match answer {
        Ok(x) => println!(" Puzzle 7-2: {}", x),
        Err(e) => panic!("No solution to puzzle: {}.", e),
    }
    assert_eq!(answer, Ok(253473930));
}
