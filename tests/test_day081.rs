use aoc_2023::solutions::day081::puzzle;
use std::collections::HashMap;

const EXAMPLE_INPUT: &str =
    "LLR

DBQ = (RTP, ZZZ)
RTP = (PXX, PLG)
VBX = (BRV, DKG)
PXX = (HLR, VBX)
BRV = (RTP, TCR)
FTS = (LJB, MDJ)
BLH = (ZZZ, GGG)";

#[test]
fn example_1_puzzle_1() {
    let mut choices_map: HashMap<&str, (&str, &str)> = HashMap::new();

    let parts: Vec<&str> = EXAMPLE_INPUT.split("\n\n")
        .map(|s| s.trim())
        .collect();

    println!("{:?}", parts);

    let pattern = parts[0];
    let map = parts[1];
    // hard set starting key
    let start_key = "DBQ";

    for line in map.lines() {
        let parts: Vec<&str> = line
            .split('=')
            .map(|s| s.trim())
            .collect();
        if parts.len() == 2 {
            let key = parts[0];
            let choices: Vec<&str> = parts[1]
                .trim_matches(|c| (c == '(' || c == ')'))
                .split(',')
                .map(|s| s.trim())
                .collect();
            if choices.len() == 2 {
                choices_map.insert(key, (choices[0], choices[1]));
            }
        }
    }

    // Print the resulting HashMap
    for (key, value) in &choices_map {
        println!("{} = {:?}", key, value);
    }
    println!("{:?}", &choices_map);
    assert_eq!(puzzle(self::EXAMPLE_INPUT), Ok(6));
}
