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
    //     let test_input: String = String::from("
    // ...........@.913.....168....=909..431......=......@..976.......+.......*..........155............................620.......250......@.......
    // ......806.....*....................*...........@................45.....475...724..*......&45.........+202..-576.....*.........*.............
    // ...............383...........................372..................................474...................................432.471......729....");

    const LINELENGTH: i32 = 140;
    //declare vec to hold digits for calc
    let mut digits = Vec::new();
    //total
    let mut total = 0;
    //readability
    let backward = -1 * LINELENGTH - 2;
    let forward = LINELENGTH - 2;
    //sequence to check around number
    //hack to handle 2 digit. subtract 1 from [1], set [11], [6] to zero
    let mut move_arr: [i32; 12] = [
        -2,
        2,
        backward,
        backward + 1,
        backward + 2,
        backward + 3,
        backward + 4,
        forward,
        forward + 1,
        forward + 2,
        forward + 3,
        forward + 4,
    ];
    //indicate to outer loop that index was incremented by compute number protocol already
    let mut already_inc = false;
    //index of string
    //set to start on first line after padding
    let mut index = LINELENGTH;
    //crawl input by letter
    let mut input_data_string = input_data.to_string();

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
                if letter.is_numeric() {
                    //if letter is a digit, enter
                    //check validity
                    //find index of middle digit (anchor -> index + 1)
                    //declare array of all values to test
                    let mut i = 0;
                    let mut j = 0;
                    let mut anchor = index + 1;

                    //this was moved out of below loop because we need the digit vec
                    //to be populated regardless of whether or not the digit passes the test
                    //as the length of this vec is used to determine how far forward to move the iterator
                    //when this was in the loop below. The digit vector was only populated when the letter
                    //passed the test, so on a failed number. the iterator was incrementing and it was calculating
                    //again on the same number.
                    while
                        input_data_string_with_padding
                            .chars()
                            .nth((index + j).try_into().unwrap())
                            .expect("REASON")
                            .is_numeric()
                    {
                        digits.push(
                            input_data_string_with_padding
                                .chars()
                                .nth((index + j).try_into().unwrap())
                                .unwrap()
                                .to_digit(10)
                        );
                        j += 1;
                    }
                    //hack for 1 digit
                    if digits.len() == 1 {
                        anchor = index;
                        move_arr[0] += 1;
                        move_arr[1] -= 1;
                        move_arr[2] = 0;
                        move_arr[6] = 0;
                        move_arr[7] = 0;
                        move_arr[11] = 0;
                    }
                    //hack to handle 2 digit. subtract 1 from [1], set [11], [6] to zero
                    if digits.len() == 2 {
                        move_arr[1] -= 1;
                        move_arr[6] = 0;
                        move_arr[11] = 0;
                    }
                    loop {
                        //if any adjacent position have non-period char or non digit char
                        //if found, valid -> computer digit and add to total list
                        //move_arr contains values to move index for char check
                        if
                            input_data_string_with_padding
                                .chars()
                                .nth((anchor + move_arr[i]) as usize) != Some('.') &&
                            input_data_string_with_padding
                                .chars()
                                .nth((anchor + move_arr[i]) as usize)
                                .unwrap()
                                .to_digit(10) == None
                        {
                            //computer digits and add to total
                            //parse digits
                            //push letters as digits to vec
                            //compute from vec and add to total
                            let mut mult_casc = 1; //iterate backwards on vec, aka, starting from the ones and multiply multiplication factor by 10 for each digit. add results together
                            for &element in digits.iter().rev() {
                                let temp = element.unwrap() * mult_casc;
                                //multiply by mult_cascade
                                total += temp;
                                //multiply mult_casc by 10
                                mult_casc *= 10;
                            }

                            break;
                        }
                        i += 1;
                        //invalid number
                        if i >= 12 {
                            break;
                        }
                    }
                    //hack for 1 digit
                    if digits.len() == 1 {
                        move_arr[0] -= 1;
                        move_arr[1] += 1;
                        move_arr[2] = backward;
                        move_arr[6] = backward + 4;
                        move_arr[7] = forward;
                        move_arr[11] = forward + 4;
                    }
                    //fix hack 2 digit
                    if digits.len() == 2 {
                        move_arr[1] += 1;
                        move_arr[6] = backward + 4;
                        move_arr[11] = forward + 4;
                    }
                    //move index to next non digit character
                    index += digits.len() as i32;
                    digits.clear();
                    already_inc = true;
                }
            }
            None => {
                // Break the loop when reaching the end of the string
                break;
            }
        }
        //if compute loop didn't increment. flip switch back off
        if already_inc {
            // Increment the index for the next iteration
            already_inc = false;
        } else {
            index += 1;
        }
    }
    Ok(total as isize)
}

pub fn puzzle(input_data: &str) -> Result<isize, PuzzleErr> {
    calculate_total(input_data)
}

pub fn main(data_dir: &str) {
    // open file with raw text

    let data = load(data_dir, 031, None);
    // declare total to add calibration parameters to

    let answer = puzzle(&data);
    match answer {
        Ok(x) => println!(" Puzzle 3-1: {}", x),
        Err(e) => panic!("No solution to puzzle: {}.", e),
    }
    assert_eq!(answer, Ok(528799));
}
