use aoc_2023::solutions::day081::puzzle;
use std::collections::HashMap;

const EXAMPLE_INPUT: &str =
    "LLRRLRLR

DBQ = (RTP, ZZZ)
RTP = (PXX, BRV)
VBX = (BRV, BLH)
PXX = (DBQ, VBX)
BRV = (RTP, VBX)
FTS = (VBX, PXX)
BLH = (ZZZ, FTS)";

#[test]
fn example_1_puzzle_1() {
    assert_eq!(puzzle(self::EXAMPLE_INPUT), Ok(5));
}
