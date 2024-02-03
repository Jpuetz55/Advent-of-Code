use aoc_2023::solutions::day091::puzzle;

const EXAMPLE_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

#[test]
fn example_1_puzzle_1() {
    assert_eq!(puzzle(self::EXAMPLE_INPUT), Ok(114));
}
