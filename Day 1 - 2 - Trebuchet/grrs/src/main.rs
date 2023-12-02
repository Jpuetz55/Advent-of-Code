// ---------------------------------------------------------------- Day 1-2, Trebuchet ----------------------------------------------------------------
use std::fs::File;
use std::io::prelude:: * ;
use std::option;
use std::path::Path;
use circular_buffer::CircularBuffer;

fn main() {
    // open file with raw text

    let path = Path::new("./params.txt");
    let display = path.display();
    let mut calib_params_file = match File::open( & path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
    };

    // write opened filed to string

    let mut calib_params_string = String::new();
    match calib_params_file.read_to_string( & mut calib_params_string) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("{} contains:\n{}", display, calib_params_string),
    }
    // declare total to add calibration parameters to

    let mut total: u32 = 0;

    //declare variable to hold the final parameter calculation

    let mut add_first_last: u32 = 0;

    //counter
    let mut i = 0;

    // Split raw calibration parameters by the new line and put them in a vector

    let calib_params_split = calib_params_string.split("\n");

    let collect_params = calib_params_split.collect:: < Vec < & str >> ();

    // Circular Buffer to check for word numbers

    // Initialize a new, empty circular buffer with a capacity of 5 elements
    // when new value gets added, oldest value gets dropped and
    // Ex buf = [1, 2, 3, 4, 5] -> buf.push_back(6) -> buf = [2, 3, 4, 5, 6]
    let mut buf = CircularBuffer:: < 5,
        char > ::new();
    //buffers string representation
    let mut current_string: String = String::new();
    // Loop over each item in array

    for item in collect_params {

        // declare temporary variable to hold the first_number and the last_number and set them to NULL 
        let mut first_number: Option < u32 > = None;
        let mut last_number: Option < u32 > = None;
        // loop through characters in array item.

        //clear buffer
        buf.drain(0..);

        for letter in item.chars() {

            //push letter onto buffer if it isn't a new line
            if letter != '\r' {
                buf.push_back(letter);
            }
            current_string = String::from_iter(buf.range(0..));
            let zero = String::from("zero");
            let one = String::from("one");
            let two = String::from("two");
            let three = String::from("three");
            let four = String::from("four");
            let five = String::from("five");
            let six = String::from("six");
            let seven = String::from("seven");
            let eight = String::from("eight");
            let nine = String::from("nine");

            // if letter.is_numeric() && first_number.is_empty then save to first_number and continue to next char
            match current_string {
                ref zero =>
                if first_number.is_none() {
                    first_number = Some(0);
                }
                else {
                    last_number = Some(0);
                },
                ref one =>
                if first_number.is_none() {
                    first_number = Some(1);
                }
                else {
                    last_number = Some(1);
                },
                ref two =>
                if first_number.is_none() {
                        first_number = Some(2);
                    } else {
                        last_number = Some(2);
                    },
                ref three =>
                    if first_number.is_none() {
                        first_number = Some(3);
                    } else {
                        last_number = Some(3);
                    },
                ref four =>
                    if first_number.is_none() {
                        first_number = Some(4);
                    } else {
                        last_number = Some(4);
                    },
                ref five =>
                    if first_number.is_none() {
                        first_number = Some(5);
                    } else {
                        last_number = Some(5);
                    },
                ref six =>
                    if first_number.is_none() {
                        first_number = Some(6);
                    } else {
                        last_number = Some(6);
                    },
                ref seven =>
                    if first_number.is_none() {
                        first_number = Some(7);
                    } else {
                        last_number = Some(7);
                    },
                ref eight =>
                    if first_number.is_none() {
                        first_number = Some(8);
                    } else {
                        last_number = Some(8);
                    },
                ref nine =>
                    if first_number.is_none() {
                        first_number = Some(9);
                    } else {
                        last_number = Some(9);
                    },
                _ => {}
            }
            print!("{}", current_string);
            if letter.is_numeric() && first_number.is_none() {
                first_number = letter.to_digit(10)
            };
            // if character == number && first_number !== NULL then save to last_number and continue 

            if letter.is_numeric() && !first_number.is_none() {
                last_number = letter.to_digit(10)
            };

            //end loop -- when loop ends first_number should contain the first number and lastCharacter should contain the last number
        }

        // calculate parameters
        //(first number * 10) + finalNumber?
        add_first_last = (first_number.unwrap() * 10) + last_number.unwrap();
        //add them to total
        total += add_first_last;
        //set back to zero for next loop
        //print!("{} : {:?}, {:?} : {:?}\n", item, first_number.unwrap(), last_number.unwrap(), add_first_last);
        print!("{}{:?}\n", i, buf.range(0..));
        add_first_last = 0;
        i += 1;
        //end loop    
    }
    print!("Total is : {:?}", total); // answer is 52974    
    // function returns the value of total 
}