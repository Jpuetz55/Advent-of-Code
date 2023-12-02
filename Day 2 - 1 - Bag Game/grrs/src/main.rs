// ---------------------------------------------------------------- Day 2-1, Bag Game ----------------------------------------------------------------
use std::fs::File;
use std::io::prelude::*;
use std::option;
use std::path::Path;

fn main()
{
    const REDMAX: i32 = 12;
    const GREENMAX: i32 = 13;
    const BLUEMAX: i32 = 14;

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

    let game_params_split = calib_params_string.split("\n");

    let collect_params = game_params_split.collect::<Vec<&str>>();

    //game counter

    let mut game_count: u32 = 1;

    for item in collect_params {

        for letter in item.chars() {

            //red counter
            let mut red_count: u32 = 0;
            //green counter
            let mut green_count: u32 = 0;
            //blue counter
            let mut blue_count: u32 = 0;




        }
        
        
        



    }


}

