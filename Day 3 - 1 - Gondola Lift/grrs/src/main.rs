// ---------------------------------------------------------------- Day 3-1, Gondola Lift ----------------------------------------------------------------
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
//mod helpers;


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

        //                          .....y....
        //                          ......x...
        //                          .....z....

        //here, the distance from x to y is -11, which turns out to be (-(line length) - 1). This is the same for any pos x on the middle line
        //the distance to z is (line length - 1).  This is the same for any pos x on the middle line
        // so we can conclude that: 
            // backward = (-(line length) - 1)
            //forward = (line length - 1)
        //we can then use these as the basis for checcking the two chars to the right of them
    //EX. with the very center cell considered as the current anchor.

    // (i-(backward)) (i-(backward + 1)) (i-(backward + 2)) (i-(backward + 3)) (i-(backward + 4))
    //      (i-2)              1                  1                   1                (i+2)
    // (i+(forward)) (i+(forward + 1)) (i+(forward + 2)) (i+(forward + 3)) (i+(forward + 4))

    //once the values for forward and backward are obtained. Need to have algo to: 

    //crawl input by letter
    //open file   
fn main() 
{
    //loop by char in string   (start)
    //crawl input by letter
    //open file
    let path = Path::new("./params.txt");
    let display = path.display();
    let mut gondola_params_file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // // write opened filed to string

    let mut gondola_params_string = String::new();
    match gondola_params_file.read_to_string(&mut gondola_params_string) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    }
    //trim newlines, we don't need them
    gondola_params_string.retain(|c| !c.is_whitespace());

    let mut index = 0;
    //loop over chars in string
    loop {
        match gondola_params_string.chars().nth(index) {
            Some(letter) => {
                // Process the character here
                //detect a number - first digit
                if letter.unwrap().is_digit(10) {
                    //check validity
                        //find index of middle digit (anchor -> index + 1) 
                            //check idx -2 and + 2 from anchor
                                //on failure
                                    //set index pointer to char after last digit to start loop again from that point
                                    //break
                            //check backward 
                                //on failure
                                    //set index pointer to char after last digit to start loop again from that point
                                    //break
                            //check forward
                                //on failure
                                    //set index pointer to char after last digit to start loop again from that point
                                    //break
                         //if all periods around number
                            //calculate numbers value
                                //implement algo from previous excercise to calculate value and add it to total -- make into helper function -- make module?
                                    //parse_int_from_string()
                                //set index pointer to char after last digit to start loop again from that point


                }
                
                println!("Character at position {}: {}", index, letter);
            }
            None => {
                // Break the loop when reaching the end of the string
                break;
            }
        }

        // Increment the index for the next iteration
        index += 1;
    }               
}
