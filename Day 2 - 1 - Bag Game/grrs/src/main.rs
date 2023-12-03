// ---------------------------------------------------------------- Day 2-1, Bag Game ----------------------------------------------------------------
use std::fs::File;
use std::io::prelude::*;
use std::option;
use std::path::Path;

fn main()
{
    const REDMAX: u32 = 12;
    const GREENMAX: u32 = 13;
    const BLUEMAX: u32 = 14;

    //crawl input by letter and computer with buffer?
    //open file

    let path = Path::new("./params.txt");
    let display = path.display();
    let mut game_params_file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // write opened filed to string

    let mut game_params_string = String::new();
    match game_params_file.read_to_string(&mut game_params_string) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    }

    let game_params_split = game_params_string.split("\n");

    let collect_params = game_params_split.collect::<Vec<&str>>();

    //game counter
    let mut game_count: u32 = 1;

    //game total
    let mut game_total: u32 = 0;

    for item in collect_params {

        let mut iter: u32 = 0;
        let mut temp_color_count: u32 = 0;  //hold total value for digit computation
        //red counter
        let mut red_count: u32 = 0;
        //green counter
        let mut green_count: u32 = 0;
        //blue counter
        let mut blue_count: u32 = 0;
        for letter in item.chars() {
            while iter >= 7 {                     //skip to first number
                
                while letter.is_numeric() {
                    //vector to hold digits
                    let mut digits = Vec::new();

                    digits.push(letter.to_digit(10).unwrap()); //add to vec

                    if !item.chars().next().unwrap().is_digit(10) { //if next is not a digit i.e. ->  ' '. digit end. compute
                        let mut mult_casc = 1; 
                        for &element in digits.iter().rev() {
                            let temp = element * mult_casc;
                            //multiply by mult_cascade
                            temp_color_count += temp;
                            //multiply mult_casc by 10
                            mult_casc *= 10;                         
                        }
                        //have quantity, now find color
                        //character value of current pos + 1 - iter already incremented above
                        //find corresponding color
                        //skip forward 2, get letter (r, b, g)
                        let color_char: char = item.chars().nth((iter + 2).try_into().unwrap()).unwrap();

                        match color_char {
                            'r' => {
                                red_count += temp_color_count
                            }
                            'g' => {
                                green_count += temp_color_count
                            }
                            'b' => {
                                blue_count += temp_color_count
                            },
                            _ => {}
                        }
                        //check failure condition 
                
                        if blue_count > BLUEMAX {
                            game_total += game_count;
                            break;
                        }
                        if green_count > GREENMAX {
                            game_total += game_count;
                            break;
                        }
                        if red_count > REDMAX {
                            game_total += game_count;
                            break;
                        }                      
                    }           
                }                                                     
            }
            iter += 1;
            game_count += 1;
        }         
    }
}

