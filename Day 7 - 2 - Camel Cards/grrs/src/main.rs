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

use core::panic;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

// Define a constant array for card rankings
const CARD_RANKINGS: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

// Function to convert a card to its corresponding rank
fn card_to_rank(card: char) -> usize {
    CARD_RANKINGS.iter()
        .position(|&c| c == card)
        .unwrap() + 1
}

//need to convert each hand_string into a ([usize; 13], usize) tuple where the usize array is the
//card counts and the usize is the hand rank
//after getting hands in this representation, need to parse usize array into a usize hank rank and
//store these values for when we determine overall ranks
//we can determine overall ranks by sorting
// Function to calculate the hand score
fn hand_to_hand_rank(hand: [usize; 13]) -> usize {
    //count how many numbers are equal to 2, 3, 4, and 5 in hand array
    let mut matches = [0; 4];

    for &bucket in hand.iter() {
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
    // Calculate the hand score

    match matches {
        [0, 0, 0, 0] => 0, // High card
        [1, 0, 0, 0] => 1, // One pair
        [2, 0, 0, 0] => 2, // Two pair
        [0, 1, 0, 0] => 3, // Three of a kind
        [1, 1, 0, 0] => 4, // Full house
        [0, 0, 1, 0] => 5, // Four of a kind
        [0, 0, 0, 1] => 6, // Five of a kind

        _ => panic!("Invalid hand"),
    }
}

// Struct to represent a hand entry
#[derive(Debug, Clone)]
struct HandEntry {
    //need to convert each hand_string into a ([usize; 13], usize) tuple where the usize array is the
    //card counts and the usize is the hand rank
    hand: (String, [usize; 13], usize),
    bid: usize,
    overall_rank: usize,
    total_score: usize,
}

// Function to convert a card to its corresponding rank
fn char_to_rank(card: char) -> u32 {
    match card {
        'A' => 1,
        'K' => 2,
        'Q' => 3,
        'J' => 4,
        'T' => 5,
        '9' => 6,
        '8' => 7,
        '7' => 8,
        '6' => 9,
        '5' => 10,
        '4' => 11,
        '3' => 12,
        '2' => 13,
        _ => panic!("Invalid card"),
    }
}

// Function to sort hands alphabetically within each rank
fn sort_hands_alphabetically_within_rank(hands: &mut Vec<HandEntry>) {
    hands.sort_by(|a, b| {
        // Compare hands character by character using char_to_rank
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

// Function to update overall rank and total score based on the index in the sorted hands vector
fn update_overall_rank_and_total_score(sorted_hands: &mut Vec<HandEntry>) {
    for (index, entry) in sorted_hands.iter_mut().enumerate() {
        entry.overall_rank = 1000 - index;
        entry.total_score = entry.bid * entry.overall_rank;
    }
}

// Function to parse input into a vector of HandEntry
fn parse_input(input: &str) -> Vec<HandEntry> {
    let mut hands: Vec<HandEntry> = Vec::new();

    // Parse input into a vector of HandEntry
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let hand_str = iter.next().unwrap();
        let bid = iter.next().unwrap().parse::<usize>().unwrap();

        // Check if 'J' is present in the hand
        if hand_str.contains('J') {
            let j_count = hand_str
                .chars()
                .filter(|&c| c == 'J')
                .count();

            if j_count <= 2 {
                // 2 or fewer 'J's, consider all permutations
                let mut j_indices: Vec<usize> = Vec::new();
                for (i, c) in hand_str.chars().enumerate() {
                    if c == 'J' {
                        j_indices.push(i);
                    }
                }

                // Generate all permutations of hand_array with different 'J' values
                for perm in 0..1 << j_count {
                    let mut hand_array: [usize; 13] = [0; 13];

                    for i in 0..j_count {
                        // Set 'J' values based on the permutation
                        if (perm & (1 << i)) != 0 {
                            let index = j_indices[i];
                            hand_array[index] += 1;
                        }
                    }

                    // Increment other card values
                    hand_str.chars().for_each(|c| {
                        match c {
                            '2'..='9' => {
                                let index = (c as usize) - ('2' as usize);
                                hand_array[index] += 1;
                            }
                            'T' => {
                                hand_array[8] += 1; // Index for 'T'
                            }
                            'Q' => {
                                hand_array[10] += 1; // Index for 'Q'
                            }
                            'K' => {
                                hand_array[11] += 1; // Index for 'K'
                            }
                            'A' => {
                                hand_array[12] += 1; // Index for 'A'
                            }
                            _ => panic!("Invalid card"),
                        }
                    });

                    hands.push(HandEntry {
                        hand: (hand_str.to_owned(), hand_array, hand_to_hand_rank(hand_array)),
                        bid,
                        overall_rank: 0, // Initialize to 0, you can set this later if needed
                        total_score: 0, // Initialize to 0, you can set this later if needed
                    });
                }
            } else {
                // 3 or more 'J's, treat 'J' as 'A'
                let mut hand_array: [usize; 13] = [0; 13];
                hand_str.chars().for_each(|c| {
                    match c {
                        '2'..='9' => {
                            let index = (c as usize) - ('2' as usize);
                            hand_array[index] += 1;
                        }
                        'T' => {
                            hand_array[8] += 1; // Index for 'T'
                        }
                        'Q' => {
                            hand_array[10] += 1; // Index for 'Q'
                        }
                        'K' | 'J' => {
                            // Treat 'J' as 'A'
                            hand_array[11] += 1; // Index for 'K'/'A'
                        }
                        'A' => {
                            hand_array[12] += 1; // Index for 'A'
                        }
                        _ => panic!("Invalid card"),
                    }
                });

                hands.push(HandEntry {
                    hand: (hand_str.to_owned(), hand_array, hand_to_hand_rank(hand_array)),
                    bid,
                    overall_rank: 0, // Initialize to 0, you can set this later if needed
                    total_score: 0, // Initialize to 0, you can set this later if needed
                });
            }
        } else {
            // No 'J' in the hand, process normally
            let mut hand_array: [usize; 13] = [0; 13];

            hand_str.chars().for_each(|c| {
                match c {
                    '2'..='9' => {
                        let index = (c as usize) - ('2' as usize);
                        hand_array[index] += 1;
                    }
                    'T' => {
                        hand_array[8] += 1; // Index for 'T'
                    }
                    'Q' => {
                        hand_array[10] += 1; // Index for 'Q'
                    }
                    'K' => {
                        hand_array[11] += 1; // Index for 'K'
                    }
                    'A' => {
                        hand_array[12] += 1; // Index for 'A'
                    }
                    _ => panic!("Invalid card"),
                }
            });

            hands.push(HandEntry {
                hand: (hand_str.to_owned(), hand_array, hand_to_hand_rank(hand_array)),
                bid,
                overall_rank: 0, // Initialize to 0, you can set this later if needed
                total_score: 0, // Initialize to 0, you can set this later if needed
            });
        }
    }

    hands
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

    // Parse the input string to get the vector of HandEntry
    let mut hands = parse_input(&params_string);

    // Separate hands into lists based on their rank
    let mut hands_by_rank: HashMap<usize, Vec<HandEntry>> = HashMap::new();
    for entry in hands.iter() {
        hands_by_rank.entry(entry.hand.2).or_insert(Vec::new()).push(entry.clone());
    }

    // Sort each list based on the original string value
    for (_, hands_list) in hands_by_rank.iter_mut() {
        sort_hands_alphabetically_within_rank(hands_list);
    }

    // Combine lists back together
    let mut sorted_hands: Vec<HandEntry> = Vec::new();
    for rank in (0..=6).rev() {
        if let Some(mut hands_list) = hands_by_rank.remove(&rank) {
            sorted_hands.append(&mut hands_list);
        }
    }

    // Update overall rank and total score
    update_overall_rank_and_total_score(&mut sorted_hands);

    // Print the results
    for entry in &sorted_hands {
        println!(
            "Rank {}: Hand: {}, Bid: {}, Overall Rank: {}, Total Score: {}",
            entry.hand.2,
            entry.hand.0,
            entry.bid,
            entry.overall_rank,
            entry.total_score
        );
    }
    let mut total = 0;
    for entry in &sorted_hands {
        total += entry.total_score;
    }

    println!("{}", total);
}
