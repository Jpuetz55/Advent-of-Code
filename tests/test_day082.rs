use aoc_2023::solutions::day081::puzzle;
use std::collections::HashMap;

const EXAMPLE_INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

#[test]
fn example_1_puzzle_1() {
    assert_eq!(puzzle(self::EXAMPLE_INPUT), Ok(6));
}
