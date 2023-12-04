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
fn main() 
{

    let test_input: String = String::from(".679.....662....71............................805..........862.680...................................................................687....
............*....-..811..........846..855......*.............*..$........230.92@............................=.....................92........
..........360..........#....664.....=.*...881...677...934.780.......426.*..........8......654.....*959.....539..........21.........*........
.....................+.........*......379..*.........*.........=.........969........*........*.976..............872....*....../....579......
.......566......652...809....482.394......492..303.650..../...38....%...............106...385..................#.....793..484.865...........
......*...........*................*..347.......*.........220.....349...691...392*..................18..797.......................+.........
.....11......890........870.......156............733.................../..........921.....................*......*.............203..........
.............#...238.....*...........................188......294./58........408...............677.......778....104.903............%........
.411+...828.......*.....706.....249..=.....638...848*...........%...............*948............*.....+......+...........917.......817...346
................69.................*..310.*....@..........118*......428=..785...................931...217.....934....475...@.145........*...");
    
    const LINELENGTH: i32 = 140;
    //declare vec to hold digits for calc
    let mut digits = Vec::new();
    //total
    let mut total = 0;
    //readability
    let backward = (-1 * LINELENGTH) - 2;
    let forward = LINELENGTH - 2;
    //sequence to check around number
    let move_arr: [i32; 12] = [-2,2,
                              backward, backward + 1,backward + 2, backward + 3, backward + 4,
                               forward, forward + 1, forward + 2, forward + 3, forward + 4
                              ];
    //index of string 
    //set to start on first line after padding                           
    let mut index = LINELENGTH;
    //crawl input by letter
    //open file
    // let path = Path::new("./params.txt");
    // let display = path.display();
    // let mut gondola_params_file = match File::open(&path) {
    //     Err(why) => panic!("couldn't open {}: {}", display, why),
    //     Ok(file) => file,
    // };

    // // write opened filed to string

    // let mut gondola_params_string = String::new();
    // match gondola_params_file.read_to_string(&mut gondola_params_string) {
    //     Err(why) => panic!("couldn't read {}: {}", display, why),
    //     Ok(_) => {}
    // }
    //trim newlines, we don't need them
    
    let mut gondola_params_string = test_input;
    gondola_params_string.retain(|c| !c.is_whitespace());
    //pad string with a line of periods on front and back for validating first and last line
    let periods = ".".repeat(LINELENGTH.try_into().unwrap());
    let gondola_params_string_with_padding = format!("{}{}{}", periods, gondola_params_string, periods);  
    //loop over chars in string
    loop 
    {
        match gondola_params_string_with_padding.chars().nth(index.try_into().unwrap()) 
        {
            
            Some(letter) => 
            {
                //detect a number - first digit         
                if letter.is_numeric() 
                { //if letter is a digit, enter
                    //check validity
                    //find index of middle digit (anchor -> index + 1)
                    //declare array of all values to test
                    let mut i = 0;
                    let anchor = index + 1;
                    //initialize failure flag
                    //set failure flag to signal outer loop
                    let mut failure_flag = false;                     
                    loop 
                    {
                        //if any adjacent position have non-period char
                        if gondola_params_string_with_padding.chars().nth((anchor + move_arr[i]) as usize) != Some('.') {
                            print!("validity test: {:?}", gondola_params_string_with_padding.chars().nth((anchor + move_arr[i]) as usize));
                            //set failure flag for outer loop
                            failure_flag = true;
                            //break
                            break;
                        }
                        i += 1;
                        //valid number
                        if i >= 12 { 
                            break; 
                        };
                    }
                    //validity confirmed
                    //calculate numbers value
                    //implement algo from previous excercise to calculate value and add it to total -- make into helper function -- make module?                                                         
                    if !failure_flag 
                    {
                        let mut j = 0;
                        //push current letter and next two letters t digits vec
                        while gondola_params_string_with_padding.chars().nth((index + j)
                                                                        .try_into()
                                                                            .unwrap())
                                                                            .expect("REASON")
                                                                            .is_numeric() 
                        {
                            digits.push(gondola_params_string_with_padding.chars().nth(((index + j))
                                                                                  .try_into()
                                                                                    .unwrap())                                  
                                                                                        .unwrap()
                                                                                        .to_digit(10));
                            j += 1;
                        }
                            
                        let mut mult_casc = 1;   //iterate backwards on vec, aka, starting from the ones and multiply multiplication factor by 10 for each digit. add results together
                        for &element in digits.iter().rev() 
                        {
                            let temp = element.unwrap() * mult_casc;
                            //multiply by mult_cascade
                            total += temp;
                            //multiply mult_casc by 10
                            mult_casc *= 10;
                        }     
                    }
                    //move index to next non digit character
                    index += digits.len() as i32;
                    digits.clear();
                }
            }
            None => {
                // Break the loop when reaching the end of the string
                break;
            }
        }
        // Increment the index for the next iteration
        index += 1;
    }    
    print!("{}", total);               
}
