use crate::tagset::TagSet;
use chrono::Duration;
use chrono::{Datelike, Local, NaiveDateTime, TimeZone};
use json::JsonValue;
use std::env;
use std::fmt;
use std::fs;
use std::ops::Add;

use log::debug;

use crate::twentry::TimeWarriorEntry;
use crate::twinput::TimeWarriorInput;

pub struct WorkGroup {
    pub tagset: TagSet,
    pub time_allocated: Duration,
    pub time_spent: Duration,
}

impl WorkGroup {
    pub fn parse_from_json_value(jv: &JsonValue) -> Self {
        // Get the tags from the entry.
        let tagset = TagSet::parse_from_json_value(jv);

        let time_allocated: f64 = jv["allocation"].as_number().unwrap().into();
        let time_allocated = (time_allocated * 3600.00) as i64;
        let time_allocated = chrono::Duration::seconds(time_allocated);

        let time_spent = chrono::Duration::seconds(0);

        WorkGroup {
            tagset,
            time_spent,
            time_allocated,
        }
    }

    pub fn matches(&self, twe: &TimeWarriorEntry) -> bool {
        self.tagset.has_all_tags_of(&twe.tagset)
    }

    pub fn process(&mut self, twe: &TimeWarriorEntry) {
        self.time_spent = self.time_spent.add(twe.duration());
    }
}

fn format_duration(duration: Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    format!("{} hrs {} mins", hours, minutes)
}

impl fmt::Display for WorkGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tags_sorted = Vec::new();
        for tag in self.tagset.tags.iter() {
            tags_sorted.push(tag.clone());
        }
        tags_sorted.sort();
        let spent = format_duration(self.time_spent);
        let allocated = format_duration(self.time_allocated);
        let remaining = match self.time_allocated.checked_sub(&self.time_spent) {
            Some(val) => format_duration(val),
            None => format_duration(chrono::Duration::seconds(0)),
        };
        write!(
            f,
            "| {0: <20} | {1: <15} | {2: <15} | {3: <15}",
            tags_sorted.join(" "),
            spent,
            allocated,
            remaining
        )
    }
}

pub fn get_workgroups(twinput: &TimeWarriorInput) -> Result<Vec<WorkGroup>, String> {
    // let config_dir = env::var("HOME").unwrap() + "/.timewarrior/aggregate";
    // //let workgroups = Vec::new();
    let start = NaiveDateTime::parse_from_str(&twinput.start, "%Y%m%dT%H%M%SZ").unwrap();
    let end = NaiveDateTime::parse_from_str(&twinput.end, "%Y%m%dT%H%M%SZ").unwrap();
    let duration = end.signed_duration_since(start);

    let config_dir = env::var("HOME").unwrap() + "/.timewarrior/aggregate";
    let start = Local.from_utc_datetime(&start).date();
    let end = Local.from_utc_datetime(&end).date();

    let allocation_file_path;
    if duration.num_days() == 1 {
        let year = start.year();
        let month = start.month();
        let day = start.day();
        allocation_file_path = format!("{}/allocation/{}/{}/{}.json", config_dir, year, month, day);
    } else if duration.num_days() == 7 {
        let year = start.year();
        let month = start.month();
        let day = start.day();
        allocation_file_path = format!(
            "{}/allocation/{}/{}/week-of-{}.json",
            config_dir, year, month, day
        );
    } else {
        return Err(format!(
            "Unsupported duration of {} days. Start = {}, End = {}",
            duration.num_days(),
            start,
            end
        )
        .to_string());
    }

    debug!("allocation_file_path {}", allocation_file_path);

    let error_msg = format!("Unable to read file {}", allocation_file_path);
    let allocation_file_contents = fs::read_to_string(allocation_file_path).expect(&error_msg);
    let parsed_json = json::parse(&allocation_file_contents).expect("Unable to parse json file");
    let mut workgroups = Vec::new();
    for jv in parsed_json.members() {
        workgroups.push(WorkGroup::parse_from_json_value(jv));
    }
    Ok(workgroups)
}

pub fn process(twinput: &TimeWarriorInput, workgroups: &mut Vec<WorkGroup>) {
    for twentry in twinput.twentries.iter() {
        for workgroup in workgroups.iter_mut() {
            if workgroup.matches(&twentry) {
                workgroup.process(&twentry);
                break;
            }
        }
    }
}

pub fn print_result(workgroups: &Vec<WorkGroup>) {
    println!(
        "| {0: <20} | {1: <15} | {2: <15} | {3: <15}",
        "group", "spent", "allocated", "remaining"
    );
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
    println!(
        "| {0: <20} | {1: <15} | {2: <15} | {3: <15}",
        "total",
        format_duration(total_spent),
        format_duration(total_allocated),
        format_duration(total_remaining),
    );
}

#[cfg(test)]
mod test {
    use super::WorkGroup;

    #[test]
    fn parse_from_json_value() {
        let s = "{\"tags\": [ \"office\", \"project\" ], \"allocation\": 0.5}";
        let jv = json::parse(&s).unwrap();
        let wg = WorkGroup::parse_from_json_value(&jv);

        assert_eq!(wg.tagset.tags.len(), 2);
        assert!(wg.tagset.tags.contains("office"));
        assert!(wg.tagset.tags.contains("project"));

        assert_eq!(wg.time_allocated.num_minutes(), 30);
        assert_eq!(wg.time_spent.num_minutes(), 0);
    }

    #[test]
    fn matches() {
        let s = "{\"tags\": [ \"personal\", \"learning\" ], \"allocation\": 0.5}";
        let jv = json::parse(&s).unwrap();
        let wg = WorkGroup::parse_from_json_value(&jv);

        let s = "{\"id\":3,\"start\":\"20210722T152328Z\",\"tags\":[\"Rust talks\",\"learning\",\"personal\"]}";
        let jv = json::parse(&s).unwrap();
        let twe = super::TimeWarriorEntry::parse_from_json_value(&jv);
        assert!(wg.matches(&twe));

        let s = "{\"id\":3,\"start\":\"20210722T152328Z\",\"tags\":[\"Rust talks\",\"office\",\"project\"]}";
        let jv = json::parse(&s).unwrap();
        let twe = super::TimeWarriorEntry::parse_from_json_value(&jv);
        assert!(!wg.matches(&twe));
    }

    #[test]
    fn duration() {
        let s = "{\"tags\": [ \"personal\", \"learning\" ], \"allocation\": 0.5}";
        let jv = json::parse(&s).unwrap();
        let mut wg = WorkGroup::parse_from_json_value(&jv);

        let s = "{\"id\":3,\"start\":\"20210722T152328Z\",\"end\":\"20210722T152330Z\",\"tags\":[\"Rust talks\",\"learning\",\"personal\"]}";
        let jv = json::parse(&s).unwrap();
        let twe = super::TimeWarriorEntry::parse_from_json_value(&jv);

        assert_eq!(wg.time_spent.num_seconds(), 0);
        wg.process(&twe);
        assert_eq!(wg.time_spent.num_seconds(), 2);
        wg.process(&twe);
        assert_eq!(wg.time_spent.num_seconds(), 4);
    }
}
