// ---------------------------------------------------------------- Day 2-1, Bag Game ----------------------------------------------------------------
use std::fs::File;
use std::io::prelude::*;
use std::option;
use std::path::Path;

fn main()
{

    //crawl input by letter and computer with buffer?
    //open file
    
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
}

