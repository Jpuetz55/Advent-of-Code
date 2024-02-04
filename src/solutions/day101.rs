use crate::{ data::load, Error };

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
  #[error("Could not locate digit.")]
  NoDigits,
}

// Main puzzle-solving function
pub fn puzzle(input_data: &str) -> Result<isize, PuzzleErr> {
  calculate_total(input_data)
}

// Function to calculate total winnings
fn calculate_total(input_data: &str) -> Result<isize, PuzzleErr> {
  //todo
  Ok(0)
}

// Main function to execute the puzzle
pub fn main(data_dir: &str) {
  let data = load(data_dir, 101, None);

  let answer = puzzle(&data);
  match answer {
    Ok(x) => println!(" Puzzle 9-2: {}", x),
    Err(e) => panic!("No solution to puzzle: {}.", e),
  }
  assert_eq!(answer, Ok(1953784198));
}
