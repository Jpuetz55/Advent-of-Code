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

fn main() {
  let test_input: String = String::from(
    "
  ...........@.913.....168....=909..431......=......@..976.......+.......*..........155............................620.......250......@.......
  ......806.....*....................*...........@................45.....475...724..*......&45.........+202..-576.....*.........*.............
  ...............383...........................372..................................474...................................432.471......729...."
  );

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
  let mut index = 0;

  let mut number_counter = 0;
  //crawl input by letter
  //open file
  // let path = Path::new("./params.txt");
  // let display = path.display();
  // let mut gondola_params_file = match File::open(&path) {
  //   Err(why) => panic!("couldn't open {}: {}", display, why),
  //   Ok(file) => file,
  // };

  // // write opened filed to string

  // let mut gondola_params_string = String::new();
  // match gondola_params_file.read_to_string(&mut gondola_params_string_with_padding) {
  //   Err(why) => panic!("couldn't read {}: {}", display, why),
  //   Ok(_) => {}
  // }
  //trim newlines, we don't need them

  let mut gondola_params_string = test_input;

  gondola_params_string.retain(|c| !c.is_whitespace());
  //pad string with a line of periods on front and back for validating first and last line
  let periods = ".".repeat(LINELENGTH.try_into().unwrap());
  let gondola_params_string_with_padding = format!(
    "{}{}{}",
    periods,
    gondola_params_string,
    periods
  );

  print!("{}", gondola_params_string_with_padding);

  //loop over chars in string
  loop {
    match gondola_params_string_with_padding.chars().nth(index.try_into().unwrap()) {
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
              gondola_params_string_with_padding
                .chars()
                .nth((index + move_arr[i]) as usize)
                .unwrap()
                .to_digit(10) != None
            {
              print!(
                "{}",
                gondola_params_string_with_padding
                  .chars()
                  .nth((index + move_arr[i]) as usize)
                  .unwrap()
              );
              //digit found
              //digit check loop
              loop {
                let mut j = 1;
                //digit found
                //check left of digit to see if its left most digit in number
                //if left most
                //this if block returns start index and end index of the digit
                if
                  gondola_params_string_with_padding
                    .chars()
                    .nth((index + move_arr[i] - j) as usize)
                    .unwrap()
                    .to_digit(10) == None
                {
                  let mut k = 1;
                  //find right most digit to figure out number length
                  //start index of number
                  let start_index = index + move_arr[i] - (j + 1); // if move_arr[i] - j is where the next non digit car is. if move_arr[i] - (j + 1) is the digit to the right of it
                  let mut end_index = start_index; //initialize to same value in case of one digit number
                  while
                    gondola_params_string_with_padding
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
                  match end_index - start_index {
                    1 => {
                      //2 digit
                      if move_arr[i] == backward {
                        move_arr[3] = 0;
                      }

                      if move_arr[i] == forward {
                        move_arr[6] = 0;
                      }
                    }

                    2 => {
                      // digit
                      if move_arr[i] == backward {
                        move_arr[3] = 0;
                        move_arr[4] = 0;
                      }

                      if move_arr[i] == forward {
                        move_arr[6] = 0;
                        move_arr[7] = 0;
                      }
                    }

                    default => {}
                  }

                  let digit_str: String = gondola_params_string_with_padding
                    .chars()
                    .skip(start_index as usize)
                    .take((start_index - end_index + 1) as usize)
                    .collect();

                  numbers_vec.push(digit_str.parse().unwrap()); //push string slice containing digit to vec as u32
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
          if number_counter == 2 {
            total += numbers_vec[1] * numbers_vec[1];
            number_counter = 0;
            print!("Total is: {}", total);
          }
        }
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
