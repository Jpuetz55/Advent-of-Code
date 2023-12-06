// ---------------------------------------------------------------- Day 1-2, Trebuchet ----------------------------------------------------------------
use std::fs::File;
use std::io::prelude::*;
use std::option;
use std::path::Path;
use circular_buffer::CircularBuffer;

fn main() {
    // open file with raw text

    let path = Path::new("./params.txt");
    let display = path.display();
    let mut calib_params_file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // write opened filed to string

    let mut calib_params_string = String::new();
    match calib_params_file.read_to_string(&mut calib_params_string) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    }
    // declare total to add calibration parameters to

    let mut total: u32 = 0;

    //declare variable to hold the final parameter calculation

    let mut add_first_last: u32 = 0;

    //counter
    let mut i = 0;

    // Split raw calibration parameters by the new line and put them in a vector

    //let calib_params_string = "9ftzbdsdkd9plrrtwo";

    let calib_params_split = calib_params_string.split("\n");

    let collect_params = calib_params_split.collect::<Vec<&str>>();

    // Circular Buffer to check for word numbers

    // Initialize a new, empty circular buffer with a capacity of 5 elements
    // when new value gets added, oldest value gets dropped and
    // Ex buf = [1, 2, 3, 4, 5] -> buf.push_back(6) -> buf = [2, 3, 4, 5, 6]
    let mut buf = CircularBuffer::<5, char>::new();
    // Loop over each item in array

    for item in collect_params {
        // declare temporary variable to hold the first_number and the last_number and set them to NULL
        let mut first_number: Option<u32> = None;
        let mut last_number: Option<u32> = None;
        // loop through characters in array item.

        //clear buffer
        let _ = buf.drain(0..);
        //initialize buffer
        for _ in 0..5 {
            buf.push_back('n'); // Pushing the placeholder value ('n' in this case)
        }

        for letter in item.chars() {
            if letter != '\r' {
                buf.push_back(letter);
            }
            //check 3 letter
            let buf3 = String::from_iter(buf.range(2..5));
            match buf3.as_str() {
                "one" => {
                    if first_number.is_none() {
                        first_number = Some(1);
                    }
                    last_number = Some(1);
                }
                "two" => {
                    if first_number.is_none() {
                        first_number = Some(2);
                    }
                    last_number = Some(2);
                }
                "six" => {
                    if first_number.is_none() {
                        first_number = Some(6);
                    }
                    last_number = Some(6);
                }
                _ => {}
            }
            //check 4 letter
            let buf4 = String::from_iter(buf.range(1..5));
            match buf4.as_str() {
                "zero" => {
                    if first_number.is_none() {
                        first_number = Some(0);
                    }
                    last_number = Some(0);
                }
                "four" => {
                    if first_number.is_none() {
                        first_number = Some(4);
                    }
                    last_number = Some(4);
                }
                "five" => {
                    if first_number.is_none() {
                        first_number = Some(5);
                    }
                    last_number = Some(5);
                }
                "nine" => {
                    if first_number.is_none() {
                        first_number = Some(9);
                    }
                    last_number = Some(9);
                }
                _ => {}
            }

            //check 5 letter

            let buf5 = String::from_iter(buf.range(0..5));
            match buf5.as_str() {
                "three" => {
                    if first_number.is_none() {
                        first_number = Some(3);
                    }
                    last_number = Some(3);
                }
                "seven" => {
                    if first_number.is_none() {
                        first_number = Some(7);
                    }
                    last_number = Some(7);
                }
                "eight" => {
                    if first_number.is_none() {
                        first_number = Some(8);
                    }
                    last_number = Some(8);
                }
                _ => {}
            }
            // if letter.is_numeric() && first_number.is_empty then save to first_number and continue to next char
            // if character == number
            if letter.is_numeric() {
                //only set first_number when it it's none
                if first_number.is_none() {
                    first_number = letter.to_digit(10);
                }
                //always set last_number when letter is numeric
                last_number = letter.to_digit(10);
            }

            //end loop -- when loop ends first_number should contain the first number and lastCharacter should contain the last number
        }
        // calculate parameters
        //(first number * 10) + finalNumber?
        add_first_last = first_number.unwrap() * 10 + last_number.unwrap();
        //add them to total
        total += add_first_last;
        print!(
            "{:?} - {:?}, {:?} - {:?} - {:?}\n",
            item,
            first_number.unwrap(),
            last_number.unwrap(),
            add_first_last,
            total
        );
        //print!("{} - \n", item);
        //print!("\t{}{:?}\n", i, buf.range(0..));
        //set back to zero for next loop
        add_first_last = 0;
        //increment counter
        i += 1;
        //end loop
    }
    print!("Total is : {:?}", total); //
}
