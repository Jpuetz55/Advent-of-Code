// ---------------------------------------------------------------- Day 1-1, Trebuchet ----------------------------------------------------------------
use std::fs::File;
use std::io::prelude::*;
use std::option;
use std::path::Path;

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
        Ok(_) => print!("{} contains:\n{}", display, calib_params_string),
    }
    // declare total to add calibration parameters to

    let mut total: u32 = 0;

    //declare variable to hold the final parameter calculation

    let mut add_first_last: u32 = 0;

    // Split raw calibration parameters by the new line and put them in a vector

    let calib_params_split = calib_params_string.split("\n");

    let collect_params = calib_params_split.collect::<Vec<&str>>();

    // Loop over each item in array

    for item in collect_params {
        // declare temporary variable to hold the first_number and the last_number and set them to NULL
        let mut first_number: Option<u32> = None;
        let mut last_number: Option<u32> = None;
        // loop through characters in array item.

        for letter in item.chars() {
            // if letter.is_numeric() && first_number.is_empty then save to first_number and continue to next char

            if letter.is_numeric() && first_number.is_none() {
                first_number = letter.to_digit(10);
            }
            // if character == number && first_number !== NULL then save to last_number and continue

            if letter.is_numeric() && !first_number.is_none() {
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
            "{} : {}, {} : {}\n",
            item,
            first_number.unwrap(),
            last_number.unwrap(),
            add_first_last
        );
        //set back to zero for next loop
        add_first_last = 0;
        //end loop
    }
    print!("Total is : {:?}", total);
}
