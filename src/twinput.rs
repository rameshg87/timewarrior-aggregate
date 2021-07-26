use crate::twentry::TimeWarriorEntry;
use log::debug;
use std::env;
use std::path::PathBuf;

pub struct TimeWarriorInput {
    pub start: String,
    pub end: String,
    pub twentries: Vec<TimeWarriorEntry>,
}

impl TimeWarriorInput {
    pub fn parse_from_str(s: &String) -> Result<Self, String> {
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
        if start.len() == 0 || end.len() == 0 {
            let current_exe = std::env::current_exe().unwrap();
            let mut expected_exe = PathBuf::new();
            expected_exe.push(env::var("HOME").unwrap());
            expected_exe.push(".timewarrior");
            expected_exe.push("extensions");
            expected_exe.push("aggregate");
            if current_exe != expected_exe {
                let current_exe = current_exe.as_path().to_str().unwrap();
                let expected_exe = expected_exe.as_path().to_str().unwrap();
                return Err(
                    format!("This binary is supposed to be installed in {} inorder for it to be an extension of timewarrior.\nCurrently it was executed from {}\n", expected_exe, current_exe)
                );
            }
            return Err(
                "Unable to find timewarrior passed statistics in standard input.\nWas this program run directly? This program is supposed to be invoked by timewarrior.\n".to_string()
            );
        }
        Ok(TimeWarriorInput {
            start,
            end,
            twentries,
        })
    }
}
