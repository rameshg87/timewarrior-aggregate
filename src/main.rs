// This is a time warrior extension for aggregating the amount of time spent and remaining on
// various groups based of predefined criteria. The criteria is defined in various files in
// ~/.timewarrior/aggregate directory. The tool is supposed to be helpful in identifying the
// various things required to understand how to use it.

use crate::twinput::TimeWarriorInput;
use isatty::stdin_isatty;
use std::env;
use std::io::{self, Read};
use std::path::PathBuf;

pub mod tagset;
pub mod twentry;
pub mod twinput;
pub mod workgroup;

fn check_exe() {
    let current_exe = std::env::current_exe().unwrap();
    let mut expected_exe = PathBuf::new();
    expected_exe.push(env::var("HOME").unwrap());
    expected_exe.push(".timewarrior");
    expected_exe.push("extensions");
    expected_exe.push("aggregate");
    if current_exe != expected_exe {
        let expected_exe = expected_exe.as_path().to_str().unwrap();
        println!(
            "Install this binary in {} inorder for it to be an extension of timewarrior.",
            expected_exe
        );
        std::process::exit(1);
    }
}

fn check_stdin_isatty() {
    if stdin_isatty() {
        println!("This binary is supposed to be invoked by timewarrior.");
        println!("Run 'timew aggregate :day' or 'timew aggregate :week' to get started.");
        std::process::exit(1);
    }
}

fn main() {
    // Check if ~/.timewarrior/aggregate directory exists.
    env_logger::init();

    check_exe();
    check_stdin_isatty();

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
