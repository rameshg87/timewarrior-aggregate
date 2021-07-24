// This is a time warrior extension for aggregating the amount of time spent and remaining on
// various groups based of predefined criteria. The criteria is defined in various files in
// ~/.timewarrior/aggregate directory. The tool is supposed to be helpful in identifying the
// various things required to understand how to use it.

use chrono::Duration;
use std::env;
use std::io::{self, Read};
use std::ops::Add;

pub mod config;
pub mod tagset;
pub mod twentry;
pub mod twinput;
pub mod workgroup;

use crate::twinput::TimeWarriorInput;

fn format_duration(duration: Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    format!("{} hrs {} mins", hours, minutes)
}

fn main() {
    // Check if ~/.timewarrior/aggregate directory exists.
    env_logger::init();

    // Accept the standard input and retrieve the individual items
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Unable to read standard input");
    let twinput = TimeWarriorInput::parse_from_str(&buffer);
    let mut workgroups = config::get_workgroups(&twinput.start, &twinput.end).unwrap();

    for twentry in twinput.twentries.iter() {
        for workgroup in workgroups.iter_mut() {
            if workgroup.matches(&twentry) {
                workgroup.process(&twentry);
                break;
            }
        }
    }

    let skip_allocated = match env::var("SKIP_ALLOCATED") {
        Ok(_) => true,
        Err(_) => false,
    };

    if skip_allocated {
        println!("| {0: <20} | {1: <15}", "group", "spent");
    } else {
        println!(
            "| {0: <20} | {1: <15} | {2: <15} | {3: <15}",
            "group", "spent", "allocated", "remaining"
        );
    }
    let mut total_spent = Duration::seconds(0);
    let mut total_allocated = Duration::seconds(0);
    for workgroup in workgroups {
        if workgroup.time_spent.num_seconds() > 0 {
            println!("{}", workgroup);
            total_spent = total_spent.add(workgroup.time_spent);
            total_allocated = total_allocated.add(workgroup.time_allocated);
        }
    }

    let total_remaining = match total_allocated.checked_sub(&total_spent) {
        Some(val) => val,
        None => chrono::Duration::seconds(0),
    };
    if skip_allocated {
        println!(
            "| {0: <20} | {1: <15}",
            "total",
            format_duration(total_spent),
        );
    } else {
        println!(
            "| {0: <20} | {1: <15} | {2: <15} | {3: <15}",
            "total",
            format_duration(total_spent),
            format_duration(total_allocated),
            format_duration(total_remaining),
        );
    }
}
