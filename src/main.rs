// This is a time warrior extension for aggregating the amount of time spent and remaining on
// various groups based of predefined criteria. The criteria is defined in various files in
// ~/.timewarrior/aggregate directory. The tool is supposed to be helpful in identifying the
// various things required to understand how to use it.

use chrono::{self, NaiveDateTime};
use log::{debug, error};
use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs;
use std::io::{self, Read};
use std::ops::Add;
use std::path::Path;
use std::process;

struct InterestingTagSet {
    tag_set: HashSet<String>,
    time_spent: chrono::Duration,
}

fn format_duration(duration: chrono::Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    format!("{} hrs {} mins", hours, minutes)
}

impl fmt::Display for InterestingTagSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tags_sorted = Vec::new();
        for tag in self.tag_set.iter() {
            tags_sorted.push(tag.clone());
        }
        tags_sorted.sort();
        let duration = format_duration(self.time_spent);
        write!(
            f,
            "| {0: <20} | {1: <15} |",
            tags_sorted.join(" "),
            duration
        )
    }
}

impl InterestingTagSet {
    fn new(tag_set: HashSet<String>) -> InterestingTagSet {
        return InterestingTagSet {
            tag_set,
            time_spent: chrono::Duration::seconds(0),
        };
    }
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
    let tags_file_path = config_dir + "/tags.json";
    let path = Path::new(&tags_file_path);
    if !path.exists() {
        error!("{} doesn't exist", tags_file_path);
        process::exit(1);
    }
    debug!("{} exists", tags_file_path);

    // Parse the json file and retrieve the tags
    let tags_file_contents = fs::read_to_string(tags_file_path).expect("Unable to read tags file");
    let parsed_json = json::parse(&tags_file_contents).expect("Unable to parse json file");
    let mut interesting_tag_sets = Vec::new();
    for tag_group in parsed_json.members() {
        let mut tag_set = HashSet::new();
        for tag in tag_group.members() {
            tag_set.insert(tag.as_str().expect("Unable to parse string").to_string());
        }
        debug!("found tag_set {:?}", tag_set);
        interesting_tag_sets.push(InterestingTagSet::new(tag_set))
    }
    debug!("found {} tags from tags file", interesting_tag_sets.len());

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
            let parsed_json = json::parse(line).expect("Unable to parse json");
            let mut tag_set = HashSet::new();
            for tag in parsed_json["tags"].members() {
                tag_set.insert(tag.as_str().expect("Unable to parse string").to_string());
            }
            let mut max_matching_tags_count = 0;
            let mut max_matching_tag_set: Option<&mut InterestingTagSet> = None;
            for interesting_tag_set in interesting_tag_sets.iter_mut() {
                let intersection: HashSet<_> =
                    interesting_tag_set.tag_set.intersection(&tag_set).collect();
                if intersection.len() > max_matching_tags_count {
                    max_matching_tags_count = intersection.len();
                    max_matching_tag_set = Some(interesting_tag_set);
                }
            }
            if let Some(matching_tag_set) = max_matching_tag_set {
                let start = parsed_json["start"].as_str().unwrap();
                let start = NaiveDateTime::parse_from_str(start, "%Y%m%dT%H%M%SZ").unwrap();
                let end;
                if let Some(value) = parsed_json["end"].as_str() {
                    end = NaiveDateTime::parse_from_str(value, "%Y%m%dT%H%M%SZ").unwrap();
                } else {
                    end = chrono::Local::now().naive_utc();
                }
                matching_tag_set.time_spent = matching_tag_set
                    .time_spent
                    .add(end.signed_duration_since(start));
                debug!("time_spent {:?}", matching_tag_set.time_spent);
            }
        }
    }

    println!("| {0: <20} | {1: <15} |", "group", "duration");
    let mut total = chrono::Duration::seconds(0);
    for interesting_tag_set in interesting_tag_sets {
        if interesting_tag_set.time_spent.num_seconds() > 0 {
            println!("{}", interesting_tag_set);
            total = total.add(interesting_tag_set.time_spent);
        }
    }
    println!("| {0: <20} | {1: <15} |", "total", format_duration(total));
}
