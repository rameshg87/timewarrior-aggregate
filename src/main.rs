// This is a time warrior extension for aggregating the amount of time spent and remaining on
// various groups based of predefined criteria. The criteria is defined in various files in
// ~/.timewarrior/aggregate directory. The tool is supposed to be helpful in identifying the
// various things required to understand how to use it.

use std::io::{self, Read};

pub mod tagset;
pub mod twentry;
pub mod twinput;
pub mod workgroup;

use crate::twinput::TimeWarriorInput;

fn main() {
    // Check if ~/.timewarrior/aggregate directory exists.
    env_logger::init();

    // Accept the standard input and retrieve the individual items
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Unable to read standard input");
    let twinput = match TimeWarriorInput::parse_from_str(&buffer) {
        Ok(val) => val,
        Err(error) => {
            print!("{}", error);
            std::process::exit(1);
        }
    };

    let mut workgroups = match workgroup::get_workgroups(&twinput) {
        Ok(val) => val,
        Err(error) => {
            print!("{}", error);
            std::process::exit(1);
        }
    };
    workgroup::process(&twinput, &mut workgroups);
    workgroup::print_result(&workgroups);
}
