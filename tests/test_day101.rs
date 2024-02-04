use aoc_2023::solutions::day101::puzzle;

const EXAMPLE_INPUT: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

#[test]
fn example_1_puzzle_1() {
  assert_eq!(puzzle(self::EXAMPLE_INPUT), Ok(4));
}
