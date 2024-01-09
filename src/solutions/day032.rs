//have to think of each number being in the center of a 3x5 matrix
//all 3 digit number
// EX.

// X X X X X
// x 1 1 1 x
// x x x x x

//if there is anything other than a period in the x cells, then the number in the middle is
//a part number and should be added to the sum

//all lines are equal length, so I should be able to calculate and check the location of each cell
//in the 5 x 5 matrix by moving forward or backward in the file a set amount for each position

// EX.   ----               .....y....
//                          ......x...
//                          .....z....

//considering a fixed length line. The distance moved backward or forward from x to get to y or z is the same for any pos x

//                          ....y.....
//                          .....xxx..
//                          ....z.....

//here, the distance from x to y is -11, which turns out to be (-(line length) - 2). This is the same for any pos x on the middle line
//the distance to z is (line length - 2).  This is the same for any pos x on the middle line
// so we can conclude that:
// backward = (-(line length) - 2)
//forward = (line length - 2)
//we can then use these as the basis for checcking the two chars to the right of them
//EX. with the very center cell considered as the current anchor.

// (i-(backward)) (i-(backward + 1)) (i-(backward + 2)) (i-(backward + 3)) (i-(backward + 4))
//      (i-2)              1                  1                   1                (i+2)
// (i+(forward)) (i+(forward + 1)) (i+(forward + 2)) (i+(forward + 3)) (i+(forward + 4))
use crate::data::load;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Could not locate digit.")]
    NoDigits,
}

fn calculate_total(input_data: &str) -> Result<isize, PuzzleErr> {
    const LINELENGTH: i32 = 140;
    //declare vec to hold digits for calc
    let mut numbers_vec: Vec<u32> = Vec::new();
    //total
    let mut total = 0;
    //readability
    let backward = -1 * LINELENGTH - 1;
    let forward = LINELENGTH - 1;
    //sequence to check around number
    let mut move_arr: [i32; 8] = [
        -1,
        1,
        backward,
        backward + 1,
        backward + 2,
        forward,
        forward + 1,
        forward + 2,
    ];
    //index for crawling letter by letter
    let mut index = LINELENGTH;
    //crawl input by letter
    // // write opened filed to string

    let mut input_data_string = input_data.to_string();
    //trim newlines, we don't need them

    // let mut input_data_string = test_input;

    input_data_string.retain(|c| !c.is_whitespace());
    //pad string with a line of periods on front and back for validating first and last line
    let periods = ".".repeat(LINELENGTH.try_into().unwrap());
    let input_data_string_with_padding = format!("{}{}{}", periods, input_data_string, periods);

    //loop over chars in string
    loop {
        match input_data_string_with_padding.chars().nth(index.try_into().unwrap()) {
            Some(letter) => {
                //detect a number - first digit
                if
                    letter == '*' //if letter is a *, enter
                {
                    let mut i = 0; //check surroundings iter
                    loop {
                        //check around *
                        //todo -- add logic to determine
                        // A - that the digit found is the left most digit of the number
                        // B - the digit found is a digit from a previously found number
                        if
                            input_data_string_with_padding
                                .chars()
                                .nth((index + move_arr[i]) as usize)
                                .unwrap()
                                .to_digit(10) != None
                        {
                            //digit found
                            //digit check loop
                            let mut j = 1;
                            loop {
                                //digit found
                                //check left of digit to see if its left most digit in number
                                //if left most
                                //this if block returns start index and end index of the digit
                                if
                                    input_data_string_with_padding
                                        .chars()
                                        .nth((index + move_arr[i] - j) as usize)
                                        .unwrap()
                                        .to_digit(10) == None
                                {
                                    let mut k = 1;
                                    //find right most digit to figure out number length
                                    //start index of number
                                    let start_index = index + move_arr[i] - j + 1; // if move_arr[i] - j is where the next non digit car is. if move_arr[i] - (j + 1) is the digit to the right of it
                                    let mut end_index = start_index; //initialize to same value in case of one digit number
                                    while
                                        input_data_string_with_padding
                                            .chars()
                                            .nth((start_index + k) as usize)
                                            .unwrap()
                                            .to_digit(10) != None
                                    {
                                        //while the next number is a digit
                                        end_index += 1;
                                        k += 1;
                                    }
                                    //end index of number
                                    //now have start_index and end_index of digit
                                    //if first digit index == forward || backward
                                    //zero out forward + 1, forward + 2, respectively to stop algo from checking already found number
                                    //need to check the number of digits in number, then decide how many positions to zero out
                                    //only need two and three digit
                                    //todo ****** Fix this logic ******
                                    //on the positions checked by this loop, we need to determine how many digits are to the right of the found digit
                                    //we can simplify the logic we need to implement by only caring about digits to the right
                                    //of the position as the digits to the left are not checked anyway.
                                    //about which indexes in the move arr to zero out
                                    //the loop below is what determines which indexes to zero out. We must fix it here.

                                    match i {
                                        //if pos being checked is one of following positions
                                        //todo: this loop needs fixed so that the algo is figuring out how many digits are to the right
                                        //of the current position being checked and zeroing out the correct number of positions in the move_arr
                                        //right now, it just figures out the length of the number and zeros out that many postitions to the right of the first found digit
                                        //this doesn't calculate correctly for numbers whose first number is not the digit in the current position
                                        2 => {
                                            //backward
                                            match end_index - (index + backward) {
                                                1 => {
                                                    //dont need to handle 1 digit
                                                    // 1 digit to right
                                                    move_arr[3] = 0;
                                                }
                                                2 => {
                                                    // 2 digits to
                                                    move_arr[3] = 0;
                                                    move_arr[4] = 0;
                                                }

                                                default => {}
                                            }
                                        }
                                        3 => {
                                            //backward + 1
                                            if end_index - (index + backward + 1) > 0 {
                                                {
                                                    // 2 digit or 3 digit to the right still only blanks 1 from this position
                                                    move_arr[4] = 0;
                                                }
                                            }
                                        }
                                        5 => {
                                            //forward
                                            match end_index - (index + forward) {
                                                1 => {
                                                    //dont need to handle 1 digit
                                                    // 2 digit
                                                    move_arr[6] = 0;
                                                }
                                                2 => {
                                                    // 3 digit
                                                    move_arr[6] = 0;
                                                    move_arr[7] = 0;
                                                }

                                                default => {}
                                            }
                                        }

                                        6 => {
                                            //forward + 1
                                            if end_index - (index + forward + 1) > 0 {
                                                // 2 digit or 3 digit still only blanks 1 from this position

                                                move_arr[7] = 0;
                                            }
                                        }

                                        default => {}
                                    }

                                    let slice: String = input_data_string_with_padding
                                        .char_indices()
                                        .skip(start_index as usize)
                                        .take((end_index - start_index + 1) as usize)
                                        .map(|(i, _)|
                                            input_data_string_with_padding.chars().nth(i).unwrap()
                                        )
                                        .collect();

                                    let parsed_integer: Result<u32, _> = slice.parse();

                                    // Check if parsing was successful
                                    match parsed_integer {
                                        Ok(parsed_value) => {
                                            // Create a vector and push the parsed value
                                            numbers_vec.push(parsed_value);

                                            // Print the vector
                                        }
                                        Err(e) => {
                                            // Handle parsing error
                                            eprintln!("Error parsing integer: {}", e);
                                        }
                                    }
                                    break;
                                } else {
                                    //check one more digit to left
                                    j += 1;
                                }
                            }
                        }
                        i += 1;
                        if i >= 8 {
                            break;
                        }
                    }
                }
                //set move_arr back to normal
                move_arr = [
                    -1,
                    1,
                    backward,
                    backward + 1,
                    backward + 2,
                    forward,
                    forward + 1,
                    forward + 2,
                ];

                //if only two digits are found around * ---  compute
                if numbers_vec.len() == 2 {
                    total += numbers_vec[0] * numbers_vec[1];
                }
                numbers_vec.clear();
            }
            None => {
                // Break the loop when reaching the end of the string
                break;
            }
        }
        index += 1;
    }
    Ok(total as isize)
}

pub fn puzzle(input_data: &str) -> Result<isize, PuzzleErr> {
    calculate_total(input_data)
}

pub fn main(data_dir: &str) {
    // open file with raw text

    let data = load(data_dir, 032, None);
    // declare total to add calibration parameters to

    let answer = puzzle(&data);
    match answer {
        Ok(x) => println!(" Puzzle 3-2: {}", x),
        Err(e) => panic!("No solution to puzzle: {}.", e),
    }
    assert_eq!(answer, Ok(84907174));
}
