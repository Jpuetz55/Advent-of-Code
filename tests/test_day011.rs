use aoc_2023::solutions::day011::puzzle;

const EXAMPLE_INPUT: &str = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

#[test]
fn example_1_puzzle_1() {
    assert_eq!(puzzle(self::EXAMPLE_INPUT), Ok(142));
}
