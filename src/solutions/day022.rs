// ---------------------------------------------------------------- Day 2-1, Bag Game ----------------------------------------------------------------
use crate::data::load;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Could not locate digit.")]
    NoDigits,
}

fn calculate_total(input_data: &str) -> Result<isize, PuzzleErr> {
    const REDMAX: u32 = 12;
    const GREENMAX: u32 = 13;
    const BLUEMAX: u32 = 14;

    //vector to hold digits
    let mut digits = Vec::new();

    //crawl input by letter
    //open file
    // ---------------------------------------------------------------- Test ----------------------------------------------------------------
    //     let test_params: &str = "Game 1: 4 blue, 4 red, 16 green; 14 green, 5 red; 1 blue, 3 red, 5 green
    // Game 2: 3 green, 8 red, 1 blue; 5 green, 6 blue; 4 green, 4 blue, 10 red; 2 green, 6 red, 4 blue; 8 red, 11 blue, 4 green; 10 red, 10 blue
    // Game 3: 7 blue, 2 green; 9 blue, 2 green, 4 red; 5 blue, 2 red; 1 red, 1 green, 10 blue; 1 green, 5 blue, 1 red
    // Game 4: 5 green, 4 blue, 15 red; 1 green, 5 blue, 2 red; 14 red, 3 blue, 2 green; 6 red, 12 green, 1 blue; 1 blue, 6 green, 16 red
    // Game 5: 1 red, 1 blue, 4 green; 3 blue, 2 green, 4 red; 4 blue, 1 red, 2 green; 1 green, 3 red, 4 blue; 1 green, 2 blue
    // Game 6: 17 red, 2 blue, 18 green; 4 green, 10 blue, 14 red; 10 blue, 15 green, 14 red; 6 blue, 9 red; 5 blue, 7 red, 10 green
    // Game 7: 2 green, 3 red, 14 blue; 3 red, 2 green, 6 blue; 3 blue, 1 red; 10 blue, 1 green; 3 green, 17 blue
    // Game 8: 9 blue, 13 green, 2 red; 3 red, 10 green, 18 blue; 8 blue, 8 green
    // Game 9: 11 red, 2 blue; 1 blue, 9 green, 13 red; 2 blue, 17 red, 6 green
    // Game 10: 13 green, 8 red, 8 blue; 10 red, 5 blue, 9 green; 3 blue, 2 green, 1 red; 5 blue, 1 red, 10 green; 10 red, 8 blue; 8 blue, 1 green
    // Game 11: 14 red, 19 green; 2 blue, 6 red, 17 green; 12 green, 9 red, 6 blue
    // Game 12: 19 green, 3 blue, 10 red; 8 red, 2 blue, 19 green; 3 blue, 6 red, 2 green; 8 red, 5 blue; 1 blue, 15 green; 8 green, 7 red
    // Game 13: 2 red, 8 green, 1 blue; 4 green, 3 blue, 2 red; 4 red, 1 green; 1 red, 1 green; 2 green, 1 blue
    // Game 14: 4 blue, 2 green; 2 blue, 6 red, 2 green; 6 red, 16 blue; 5 blue, 1 green, 5 red
    // Game 15: 2 red, 4 green, 4 blue; 5 red; 5 green, 2 red, 2 blue; 5 green, 1 blue, 7 red";
    //let game_params_split = test_params.split("\n");

    let game_params_split = input_data.split("\n");

    let games = game_params_split.collect::<Vec<&str>>();

    // print!("{:?}\n", games);

    //game counter
    let mut game_count: u32 = 1;
    //game total
    let mut game_total: u32 = 0;
    //iterators to help read input/debug
    let mut i: u32 = 0; //letter iterator
    let mut j: u32 = 1; //game iterator

    let mut temp_color_count: u32 = 0; //hold total value for digit computation
    //red max counter per game
    let mut red_max_count: u32 = 0;
    //green max counter per game
    let mut green_max_count: u32 = 0;
    //blue max counter per game
    let mut blue_max_count: u32 = 0;
    //signal failure from letter loop to game loop
    let mut fail_flag: u32 = 0;

    for game in games {
        red_max_count = 0;
        green_max_count = 0;
        blue_max_count = 0;

        for letter in game.chars() {
            if i >= 7 {
                if letter.is_numeric() {
                    digits.push(letter.to_digit(10).unwrap()); //add to vec
                    if
                        !game
                            .chars()
                            .nth((i + 1).try_into().unwrap())
                            .unwrap()
                            .is_digit(10) //next character sin't a digit. we have all digits now.
                    {
                        let mut mult_casc = 1; //iterate backwards on vec, aka, starting from the ones and multiply multiplication factor by 10 for each digit. add results together
                        for &element in digits.iter().rev() {
                            let temp = element * mult_casc;
                            //multiply by mult_cascade
                            temp_color_count += temp;
                            //multiply mult_casc by 10
                            mult_casc *= 10;
                        }
                        //have quantity, now find color
                        //character value of current pos + 2
                        //find corresponding color
                        //skip forward 2, get letter (r, b, g)
                        let color_char: char = game
                            .chars()
                            .nth((i + 2).try_into().unwrap())
                            .unwrap();

                        // check max with new value obtained above
                        match color_char {
                            'r' => {
                                if temp_color_count > red_max_count {
                                    red_max_count = temp_color_count;
                                }
                            }
                            'g' => {
                                if temp_color_count > green_max_count {
                                    green_max_count = temp_color_count;
                                }
                            }
                            'b' => {
                                if temp_color_count > blue_max_count {
                                    blue_max_count = temp_color_count;
                                }
                            }
                            _ => {}
                        }
                        //clear for next number
                        digits.clear();
                        //reset for next number
                        temp_color_count = 0;
                    }
                }
            }
            i += 1;
        }
        //have all max color counts for game now
        // multiply them together with result added to running total
        game_total += red_max_count * green_max_count * blue_max_count;
        j += 1;
        i = 0; //reset i for next game
    }
    Ok(game_total as isize)
}

pub fn puzzle(input_data: &str) -> Result<isize, PuzzleErr> {
    calculate_total(input_data)
}

pub fn main(data_dir: &str) {
    // open file with raw text

    let data = load(data_dir, 022, None);
    // declare total to add calibration parameters to

    let answer = puzzle(&data);
    match answer {
        Ok(x) => println!(" Puzzle 2-2: {}", x),
        Err(e) => panic!("No solution to puzzle: {}.", e),
    }
    assert_eq!(answer, Ok(66909));
}
