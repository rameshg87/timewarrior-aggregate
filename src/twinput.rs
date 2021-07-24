use crate::twentry::TimeWarriorEntry;
use log::debug;

pub struct TimeWarriorInput {
    pub start: String,
    pub end: String,
    pub twentries: Vec<TimeWarriorEntry>,
}

impl TimeWarriorInput {
    pub fn parse_from_str(s: &String) -> Self {
        let mut twentries = Vec::new();
        let mut start = String::from("");
        let mut end = String::from("");
        for line in s.lines() {
            if line.starts_with("{") {
                debug!("line {}", line);
                let line = match line.strip_suffix(",") {
                    Some(val) => val,
                    None => line,
                };
                let jv = json::parse(line).expect("Unable to parse json");
                let twentry = TimeWarriorEntry::parse_from_json_value(&jv);
                twentries.push(twentry)
            } else if line.starts_with("temp.report.start") {
                let split = line.split(" ");
                let vec: Vec<&str> = split.collect();
                start = vec[1].to_string();
            } else if line.starts_with("temp.report.end") {
                let split = line.split(" ");
                let vec: Vec<&str> = split.collect();
                end = vec[1].to_string();
            }
        }
        TimeWarriorInput {
            start,
            end,
            twentries,
        }
    }
}
