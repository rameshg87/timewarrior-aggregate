// This is a time warrior extension for aggregating the amount of time spent and remaining on
// various groups based of predefined criteria. The criteria is defined in various files in
// ~/.timewarrior/aggregate directory. The tool is supposed to be helpful in identifying the
// various things required to understand how to use it.

use chrono::{Datelike, Duration, Local};
use log::{debug, error};
use std::env;
use std::fs;
use std::io::{self, Read};
use std::ops::Add;
use std::path::Path;
use std::process;

pub mod tagset;
pub mod twentry;
pub mod workgroup;

use crate::twentry::TimeWarriorEntry;
use crate::workgroup::WorkGroup;

fn format_duration(duration: Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    format!("{} hrs {} mins", hours, minutes)
}

fn main() {
    // Check if ~/.timewarrior/aggregate directory exists.
    env_logger::init();
    let config_dir = env::var("HOME").unwrap() + "/.timewarrior/aggregate";
    let path = Path::new(&config_dir);
    if !path.exists() {
        error!("{} doesn't exist", config_dir);
        process::exit(1);
    }
    debug!("{} exists", config_dir);

    // Check if ~/.timewarrior/aggregate/tags.json file exists.
    let tags_file_path = config_dir.clone() + "/tags.json";
    let path = Path::new(&tags_file_path);
    if !path.exists() {
        error!("{} doesn't exist", tags_file_path);
        process::exit(1);
    }
    debug!("{} exists", tags_file_path);

    // If DAILY environment variable is set, check if
    // ~/.timewarrior/aggregate/<year>/<month>/<day>.json file exists.
    let now = Local::now().naive_utc();
    let today = now.date();
    let year = today.year();
    let month = today.month();
    let day = today.day();
    let allocation_file_path = format!("{}/allocation/{}/{}/{}.json", config_dir, year, month, day);
    debug!("allocation_file_path {}", allocation_file_path);
    let path = Path::new(&allocation_file_path);
    if !path.exists() {
        error!("{} doesn't exist", allocation_file_path);
        process::exit(1);
    }

    let allocation_file_contents =
        fs::read_to_string(allocation_file_path).expect("Unable to read tags file");
    let parsed_json = json::parse(&allocation_file_contents).expect("Unable to parse json file");
    let mut workgroups = Vec::new();
    for jv in parsed_json.members() {
        workgroups.push(WorkGroup::parse_from_json_value(jv));
    }

    // Accept the standard input and retrieve the individual items
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Unable to read standard input");
    for line in buffer.lines() {
        if line.starts_with("{") {
            debug!("line {}", line);
            let line = match line.strip_suffix(",") {
                Some(val) => val,
                None => line,
            };
            let jv = json::parse(line).expect("Unable to parse json");
            let twentry = TimeWarriorEntry::parse_from_json_value(&jv);
            for workgroup in workgroups.iter_mut() {
                if workgroup.matches(&twentry) {
                    workgroup.process(&twentry);
                    break;
                }
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
