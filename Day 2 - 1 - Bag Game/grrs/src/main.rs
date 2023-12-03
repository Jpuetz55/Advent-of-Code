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

    //vector to hold digits
    let mut digits = Vec::new();

    //crawl input by letter and computer with buffer?
    //open file

    // let path = Path::new("./params.txt");
    // let display = path.display();
    // let mut game_params_file = match File::open(&path) {
    //     Err(why) => panic!("couldn't open {}: {}", display, why),
    //     Ok(file) => file,
    // };

    // // write opened filed to string

    // let mut game_params_string = String::new();
    // match game_params_file.read_to_string(&mut game_params_string) {
    //     Err(why) => panic!("couldn't read {}: {}", display, why),
    //     Ok(_) => {}
    // }

    let test_params: &str = "Game 1: 4 blue, 4 red, 16 green; 14 green, 5 red; 1 blue, 3 red, 5 green
Game 2: 3 green, 8 red, 1 blue; 5 green, 6 blue; 4 green, 4 blue, 10 red; 2 green, 6 red, 4 blue; 8 red, 11 blue, 4 green; 10 red, 10 blue
Game 3: 7 blue, 2 green; 9 blue, 2 green, 4 red; 5 blue, 2 red; 1 red, 1 green, 10 blue; 1 green, 5 blue, 1 red
Game 4: 5 green, 4 blue, 15 red; 1 green, 5 blue, 2 red; 14 red, 3 blue, 2 green; 6 red, 12 green, 1 blue; 1 blue, 6 green, 16 red
Game 5: 1 red, 1 blue, 4 green; 3 blue, 2 green, 4 red; 4 blue, 1 red, 2 green; 1 green, 3 red, 4 blue; 1 green, 2 blue
Game 6: 17 red, 2 blue, 18 green; 4 green, 10 blue, 14 red; 10 blue, 15 green, 14 red; 6 blue, 9 red; 5 blue, 7 red, 10 green
Game 7: 2 green, 3 red, 14 blue; 3 red, 2 green, 6 blue; 3 blue, 1 red; 10 blue, 1 green; 3 green, 17 blue
Game 8: 9 blue, 13 green, 2 red; 3 red, 10 green, 18 blue; 8 blue, 8 green
Game 9: 11 red, 2 blue; 1 blue, 9 green, 13 red; 2 blue, 17 red, 6 green
Game 10: 13 green, 8 red, 8 blue; 10 red, 5 blue, 9 green; 3 blue, 2 green, 1 red; 5 blue, 1 red, 10 green; 10 red, 8 blue; 8 blue, 1 green
Game 11: 14 red, 19 green; 2 blue, 6 red, 17 green; 12 green, 9 red, 6 blue
Game 12: 19 green, 3 blue, 10 red; 8 red, 2 blue, 19 green; 3 blue, 6 red, 2 green; 8 red, 5 blue; 1 blue, 15 green; 8 green, 7 red
Game 13: 2 red, 8 green, 1 blue; 4 green, 3 blue, 2 red; 4 red, 1 green; 1 red, 1 green; 2 green, 1 blue
Game 14: 4 blue, 2 green; 2 blue, 6 red, 2 green; 6 red, 16 blue; 5 blue, 1 green, 5 red
Game 15: 2 red, 4 green, 4 blue; 5 red; 5 green, 2 red, 2 blue; 5 green, 1 blue, 7 red";

    //let game_params_split = game_params_string.split("\n");

    let game_params_split = test_params.split("\n");

    let collect_params = game_params_split.collect::<Vec<&str>>();

    

    print!("{:?}\n", collect_params);

    

    //game counter
    let mut game_count: u32 = 1;

    //game total
    let mut game_total: u32 = 0;

    let mut i: u32 = 0;
    for item in collect_params {
        let mut temp_color_count: u32 = 0;  //hold total value for digit computation
        //red counter
        let mut red_count: u32 = 0;
        //green counter
        let mut green_count: u32 = 0;
        //blue counter
        let mut blue_count: u32 = 0;
        
        for letter in item.chars() {
            if (i >= 7) 
            {
                if letter.is_numeric() 
                {   
                     temp_color_count = 0;               
                    digits.push(letter.to_digit(10).unwrap()); //add to vec
                    if !item.chars().nth((i + 1).try_into().unwrap()).unwrap().is_digit(10) 
                    { //if next is not a digit i.e. ->  ' '. digit end. compute
                        let mut mult_casc = 1; 
                        for &element in digits.iter().rev() {
                            let temp = element * mult_casc;
                            //multiply by mult_cascade
                            temp_color_count += temp;
                            //multiply mult_casc by 10
                            mult_casc *= 10;                         
                        }
                        //have quantity, now find color
                        //character value of current pos + 2
                        //find corresponding color
                        //skip forward 2, get letter (r, b, g)
                        let color_char: char = item.chars().nth((i + 2).try_into().unwrap()).unwrap();

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
                        digits.clear();                     
                    } 
                    break;

                }                           
            }
            //check failure condition   
            if blue_count > BLUEMAX {
                game_total += game_count;
                print!("Blue Failed {}\n", game_count);
            }
            if green_count > GREENMAX {
                game_total += game_count;
                print!("Green Failed {}\n", game_count);
            }
            if red_count > REDMAX {
                game_total += game_count;
                print!("Red Failed {}\n", game_count);
            }
            i += 1;            
        }                                                           
        print!("end loop {}\n", i);
        //set i back to zero for next item
        i = 0;
        game_count += 1;   
    }
}

