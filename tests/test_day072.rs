use aoc_2023::solutions::day071::puzzle;

const EXAMPLE_INPUT: &str = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

#[test]
fn example_1_puzzle() {
    assert_eq!(puzzle(self::EXAMPLE_INPUT), Ok(5905));
}
