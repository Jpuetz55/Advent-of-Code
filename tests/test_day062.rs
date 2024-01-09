use aoc_2023::solutions::day062::puzzle;

const EXAMPLE_INPUT: &str = "
Time:      7  15   30
Distance:  9  40  200
";

#[test]
fn example_1_puzzle() {
    assert_eq!(puzzle(self::EXAMPLE_INPUT), Ok(71503));
}
