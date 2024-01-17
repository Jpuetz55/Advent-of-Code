use aoc_2023::solutions::day081::puzzle;

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
    println!("EXAMPLE_INPUT: {}", self::EXAMPLE_INPUT);
    assert_eq!(puzzle(self::EXAMPLE_INPUT), Ok(6));
}
