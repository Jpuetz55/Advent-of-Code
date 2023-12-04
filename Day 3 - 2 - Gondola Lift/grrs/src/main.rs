// ---------------------------------------------------------------- Day 3-2, Gondola Lift ----------------------------------------------------------------
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
    //EX. with the very center cell considered as the current index.

    // (i-(backward)) (i-(backward + 1)) (i-(backward + 2)) (i-(backward + 3)) (i-(backward + 4))
    //      (i-2)              1                  1                   1                (i+2)
    // (i+(forward)) (i+(forward + 1)) (i+(forward + 2)) (i+(forward + 3)) (i+(forward + 4))

fn main() 
{

//     let test_input: String = String::from("
// ...........@.913.....168....=909..431......=......@..976.......+.......*..........155............................620.......250......@.......
// ......806.....*....................*...........@................45.....475...724..*......&45.........+202..-576.....*.........*.............
// ...............383...........................372..................................474...................................432.471......729....");
    
    const LINELENGTH: i32 = 140;
    //declare vec to hold digits for calc
    let mut numbers_vec: Vec<String>= Vec::new();
    //vec to hold index values of found numbers
    let mut found_indexes = Vec::new();
    //hold numbers to multiply
    let mut multiplicand_vec = Vec::new();
    //multiplication cascade for digit computation
    let mut mult_casc = 1;
    //total
    let mut total = 0;
    //readability
    let backward = (-1 * LINELENGTH) - 1;
    let forward = LINELENGTH - 1;
    //sequence to check around number
    let move_arr: [i32; 8] = [-1,1,
                                  backward, backward + 1,backward + 2,
                                  forward, forward + 1, forward + 2
                              ];
    //index for crawling letter by letter                           
    let mut index = 0;
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
    
    // let mut gondola_params_string = test_input;

    //loop over chars in string
    loop 
    {
        match gondola_params_string.chars().nth(index.try_into().unwrap()) 
        {
            
            Some(letter) => 
            {
                //detect a number - first digit         
                if letter == '*' //if letter is a *, enter
                { 
                    let mut i = 0; //check surroundings iter                           
                    loop 
                    {
                        //check around *
                        //todo -- add logic to determine
                            // A - that the digit found is the left most digit of the number
                            // B - the digit found is a digit from a previously found number
                        if gondola_params_string.chars()
                                                .nth((index + move_arr[i]) as usize)
                                                .unwrap().to_digit(10) 
                                                != None
                        {
                            loop 
                            {

                                let mut j = 1;
                                //digit found
                                //check left of digit to see if its left most digit in number
                                //if left most
                                if gondola_params_string.chars()
                                                .nth((index + move_arr[i] - j) as usize)
                                                .unwrap().to_digit(10) == None 
                                {
                                    let mut k = 1;
                                    //find right most digit to figure out number length
                                    //start index of number
                                    let start_index = index + move_arr[i];
                                    let end_index = index + move_arr[i];   //initialize to same value in case of one digit number
                                    while gondola_params_string.chars()
                                                .nth((index + move_arr[i] - j) as usize)
                                                .unwrap().to_digit(10) != None 
                                    {      //while the next number is not a digit
                                        end_index = index + move_arr[i] + k;
                                        k += 1
                                    }
                                    //end index of number
                                    //if first digit index == forward || backward
                                    //zero out forward + 1, forward + 2, respectively to stop algo from checking already found number
                                        if index + move_arr[i] ==  forward + move_arr[i] {
                                            move_arr[6] = 0;
                                            move_arr[7] = 0;
                                        }
                                        else if index + move_arr[i] ==  backward + move_arr[i]{
                                            move_arr[3] = 0;
                                            move_arr[4] = 0;
                                        }
                                        else{

                                        }

                                }
                                   
                                else {
                                    j += 1;
                                }               
                                
                                    
                                            //find number length                                       
                                        //push index to index vector
                                    //if not left most
                                        //move char pointer left one, check again
                                print!("Index of Number: {:?}\tNumber Found: {:?}", index + move_arr[i], 
                                                                                    gondola_params_string.chars()
                                                                                                        .nth((index + move_arr[i]) as usize));
                                //push index of found number to found_indexes vector
                                numbers_vec.push(index + move_arr[i]);
                                }
                        }  
                        }
                        i += 1;
                        if i >= 8 { 
                            break; 
                        };
                        //if only two digits are found around * - compute
                        if found_indexes.len() == 2 
                        {
                            //go to first digit by using value stored in found_indexes
                            let mut k = 0; //increment to next vector in digits vector
                            for idx in &found_indexes {
                                let mut j = 0;   //push to digit vec on digits vector iter  
                                while gondola_params_string_with_padding.chars()
                                                                        .nth((idx + j)
                                                                        .try_into()
                                                                        .unwrap())
                                                                        .expect("REASON")
                                                                        .is_numeric()
                                {
                                        digits_vec[k].push(gondola_params_string_with_padding.chars()
                                                                                             .nth((index + j).try_into().unwrap())
                                                                                             .map(|c| c.to_digit(10).unwrap_or(0))
                                                                                             .unwrap_or(0 as u32));
                                        j += 1;                                   
                                }
                                k += 1;
                            } 
                            //digits vector now populated with two digit vectors to multiply
                            //compute value of each digit 
                            //multiply them together and add them to running total                      
                            print!("\tTotal Previous: {}", total);
                            //compute from vec and add to total
                            //iterate backwards on vec, aka, starting from the ones and multiply multiplication factor by 10 for each digit. add results together                  
                            for element in digits_vec.iter()
                            {
                                let loop_total = 0;                               
                                for &number in element.iter().rev() {
                                    let temp = number * mult_casc;
                                    //multiply by mult_cascade
                                    total += temp;
                                    //multiply mult_casc by 10
                                    mult_casc *= 10;
                                }
                                mult_casc = 1;
                                multiplicand_vec.push(loop_total);     //push final computed value                           
                            }
                            //we now have a vector with the two numbers we need to multiply
                            //set the total equal to the last number
                            let mut product_for_total = multiplicand_vec.pop().unwrap();
                            //multiply the rest of the numbers by this numbers --- scalability
                            for &number in multiplicand_vec.iter() {
                                product_for_total *= number;
                            }
                            //have total now, add it to running total
                            total += product_for_total;
                        }
                    }        
                            print!("\t{:?}\tTotal: {}\n", digits_vec.as_mut_slice(), total);                      
                } 
            None => {
                // Break the loop when reaching the end of the string
                break;
            }
        }
        index += 1;                 
    }     
    print!("{}", total);               
}
