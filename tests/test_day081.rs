use aoc_2023::solutions::day081::puzzle;
use std::collections::HashMap;

const EXAMPLE_INPUT: &str =
    "DBQ = (RTP, ZZZ)
RTP = (PXX, PLG)
VBX = (BRV, DKG)
PXX = (HLR, VBX)
BRV = (RTP, TCR)
FTS = (LJB, MDJ)
BLH = (ZZZ, GGG)";

#[test]
fn example_1_puzzle_1() {
    let mut choices_map: HashMap<&str, (&str, &str)> = HashMap::new();

    let mut current_key = "";

    for line in EXAMPLE_INPUT.lines() {
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            continue;
        }

        if trimmed_line.chars().all(|c| c.is_alphabetic()) {
            current_key = trimmed_line;
        } else {
            let parts: Vec<&str> = trimmed_line
                .split('=')
                .map(|s| s.trim())
                .collect();
            if parts.len() == 2 {
                let choices: Vec<&str> = parts[1]
                    .trim_matches(|c| (c == '(' || c == ')'))
                    .split(',')
                    .map(|s| s.trim())
                    .collect();
                if choices.len() == 2 {
                    choices_map.insert(current_key, (choices[0], choices[1]));
                }
            }
        }
    }

    // Print the resulting HashMap
    for (key, value) in &choices_map {
        println!("{} = {:?}", key, value);
    }
    assert_eq!(puzzle(self::EXAMPLE_INPUT), Ok(6));
}
