use aoc_2023::solutions::day071::puzzle;
use permutator::{ Combination, Permutation };

const EXAMPLE_INPUT: &str = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

const PERMUTE_CARDS: [char; 12] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

#[test]
fn example_1_puzzle() {
    assert_eq!(puzzle(self::EXAMPLE_INPUT), Ok(5905));
}

//generate permutations when j == 2
//returns a vector of tuples for each combo
fn generate_permutations_two_j() -> Vec<(char, char)> {
    let mut combinations: Vec<(char, char)> = Vec::new();
    let mut counter = 1;
    PERMUTE_CARDS.combination(2).for_each(|mut c| {
        c.permutation().for_each(|p| {
            println!("k-permutation@{}={:?}", counter, p);
            combinations.push((*p[0], *p[1]));
            counter += 1;
        });
    });
    PERMUTE_CARDS.iter().for_each(|&card| {
        combinations.push((card, card));
        counter += 1;
    });
    combinations
}

fn get_max_rank(hand: [usize; 13]) -> usize {
    //count how many numbers are equal to 2, 3, 4, and 5 in hand array
    println!("{:?}", hand);
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

//param: hand_array with the j's taken out
fn hand_array_vec_two_j() -> Vec<[usize; 13]> {
    let combinations = generate_permutations_two_j();
    let hand_array = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1]; //hand_array with j's taken out
    let mut hand_array_vec: Vec<[usize; 13]> = Vec::new();
    for c in combinations {
        let mut hand_array_copy = hand_array;
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
    println!("{:?} {:?} test", hand_array_vec, hand_array_vec.len());
    hand_array_vec
}
fn hand_array_vec_one_j() -> Vec<[usize; 13]> {
    let hand_array = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 1]; //hand_array with j's taken out
    let mut hand_array_vec: Vec<[usize; 13]> = Vec::new();
    for c in PERMUTE_CARDS {
        let mut hand_array_copy = hand_array;
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
    println!("{:?} {:?}", hand_array_vec, hand_array_vec.len());
    hand_array_vec
}
#[test]
fn get_max_rank_one_j() {
    let mut max_rank = 0;
    let hand_array_vec = hand_array_vec_one_j();
    for hand_array in hand_array_vec {
        let rank = get_max_rank(hand_array);
        if rank > max_rank {
            max_rank = rank;
        }
    }
    println!("max rank: {}", max_rank);
}
#[test]
fn get_max_rank_three_j() {
    let hand_array = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0];
    // if there is a 2 in hand array, max_rank is 6
    for item in hand_array {
        if item == 2 {
            println!("max rank: {}", 6);
        } else {
            println!("max rank: {}", 5);
        }
    }
    //if there is no 2 in hand array, max_rank is 5
}
