use aoc_2023::solutions::day012::puzzle;

const EXAMPLE_INPUT: &str =
    "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

#[test]
fn example_1_puzzle() {
    assert_eq!(puzzle(self::EXAMPLE_INPUT), Ok(281));
}
