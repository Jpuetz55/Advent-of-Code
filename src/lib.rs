mod data;
pub mod solutions;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error<'a> {
  #[error("Day not yet implemented: {}.", .0)] DayNotImplemented(&'a str),
}

pub fn run_day<'a>(data_dir: &'a str, day: &'a str) -> Result<(), Error<'a>> {
  match day {
    "011" => {
      solutions::day011::main(data_dir);
      Ok(())
    }
    "012" => {
      solutions::day012::main(data_dir);
      Ok(())
    }
    "021" => {
      solutions::day021::main(data_dir);
      Ok(())
    }
    "022" => {
      solutions::day022::main(data_dir);
      Ok(())
    }
    "031" => {
      solutions::day031::main(data_dir);
      Ok(())
    }
    "032" => {
      solutions::day032::main(data_dir);
      Ok(())
    }
    "041" => {
      solutions::day041::main(data_dir);
      Ok(())
    }
    "042" => {
      solutions::day042::main(data_dir);
      Ok(())
    }
    "051" => {
      solutions::day051::main(data_dir);
      Ok(())
    }
    "052" => {
      solutions::day052::main(data_dir);
      Ok(())
    }
    "061" => {
      solutions::day061::main(data_dir);
      Ok(())
    }
    "062" => {
      solutions::day062::main(data_dir);
      Ok(())
    }
    "071" => {
      solutions::day071::main(data_dir);
      Ok(())
    }
    "072" => {
      solutions::day072::main(data_dir);
      Ok(())
    }
    "081" => {
      solutions::day081::main(data_dir);
      Ok(())
    }
    "082" => {
      solutions::day082::main(data_dir); // Fix the function call
      Ok(())
    }
    "091" => {
      solutions::day091::main(data_dir);
      Ok(())
    }
    // "092" => {
    //     solutions::day092::main(data_dir);
    //     Ok(())
    // }
    // "101" => {
    //     solutions::day101::main(data_dir);
    //     Ok(())
    // }
    // "102" => {
    //     solutions::day102::main(data_dir);
    //     Ok(())
    // }
    // "111" => {
    //     solutions::day111::main(data_dir);
    //     Ok(())
    // }
    // "112" => {
    //     solutions::day112::main(data_dir);
    //     Ok(())
    // }
    // "121" => {
    //     solutions::day121::main(data_dir);
    //     Ok(())
    // }
    // "122" => {
    //     solutions::day122::main(data_dir);
    //     Ok(())
    // }
    // "131" => {
    //     solutions::day131::main(data_dir);
    //     Ok(())
    // }
    // "132" => {
    //     solutions::day132::main(data_dir);
    //     Ok(())
    // }
    // "141" => {
    //     solutions::day141::main(data_dir);
    //     Ok(())
    // }
    // "142" => {
    //     solutions::day142::main(data_dir);
    //     Ok(())
    // }
    // "151" => {
    //     solutions::day151::main(data_dir);
    //     Ok(())
    // }
    // "152" => {
    //     solutions::day152::main(data_dir);
    //     Ok(())
    // }
    // "161" => {
    //     solutions::day161::main(data_dir);
    //     Ok(())
    // }
    // "162" => {
    //     solutions::day162::main(data_dir);
    //     Ok(())
    // }
    // "171" => {
    //     solutions::day171::main(data_dir);
    //     Ok(())
    // }
    // "172" => {
    //     solutions::day172::main(data_dir);
    //     Ok(())
    // }
    // "181" => {
    //     solutions::day181::main(data_dir);
    //     Ok(())
    // }
    // "182" => {
    //     solutions::day182::main(data_dir);
    //     Ok(())
    // }
    // "191" => {
    //     solutions::day191::main(data_dir);
    //     Ok(())
    // }
    // "192" => {
    //     solutions::day192::main(data_dir);
    //     Ok(())
    // }
    // "201" => {
    //     solutions::day201::main(data_dir);
    //     Ok(())
    // }
    // "202" => {
    //     solutions::day202::main(data_dir);
    //     Ok(())
    // }
    // "211" => {
    //     solutions::day211::main(data_dir);
    //     Ok(())
    // }
    // "212" => {
    //     solutions::day212::main(data_dir);
    //     Ok(())
    // }
    // "221" => {
    //     solutions::day221::main(data_dir);
    //     Ok(())
    // }
    // "222" => {
    //     solutions::day222::main(data_dir);
    //     Ok(())
    // }
    // "231" => {
    //     solutions::day231::main(data_dir);
    //     Ok(())
    // }
    // "232" => {
    //     solutions::day232::main(data_dir);
    //     Ok(())
    // }
    // "241" => {
    //     solutions::day241::main(data_dir);
    //     Ok(())
    // }
    // "242" => {
    //     solutions::day242::main(data_dir);
    //     Ok(())
    // }
    // "251" => {
    //     solutions::day251::main(data_dir);
    //     Ok(())
    // }
    // "252" => {
    //     solutions::day252::main(data_dir);
    //     Ok(())
    // }

    // <-- INSERT NEW DAY HERE -->
    _ => Err(Error::DayNotImplemented(day)),
  }
}

// pub fn run_all(data_dir: &str) {
//     for day in 1..26 {
//         match run_day(data_dir, day) {
//             Ok(()) => {
//                 continue;
//             }
//             Err(Error::DayNotImplemented(_)) => {
//                 break;
//             }
//         }
//     }
// }
