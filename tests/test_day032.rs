use aoc_2023::solutions::day032::puzzle;

const EXAMPLE_INPUT: &str =
    "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

#[test]
fn example_1_puzzle() {
    assert_eq!(puzzle(self::EXAMPLE_INPUT), Ok(467835));
}
