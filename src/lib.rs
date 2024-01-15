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
