use aoc_2023::solutions::day082::puzzle;

const EXAMPLE_INPUT: &str =
    "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

#[test]
fn example_1_puzzle_1() {
    assert_eq!(puzzle(self::EXAMPLE_INPUT), Ok(6));
}
