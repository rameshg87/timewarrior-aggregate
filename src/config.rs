// Function to read configuration settings and return relevant workgroups. The files are located
// within ~/.timewarrior/aggregate.

// use crate::twentry::TimeWarriorEntry;
use crate::workgroup::WorkGroup;
use chrono::{Datelike, Local, NaiveDateTime, TimeZone};
use std::env;
use std::fs;

#[cfg(not(test))]
use log::debug;

#[cfg(test)]
use std::println as debug;

pub fn get_workgroups(start: &str, end: &str) -> Result<Vec<WorkGroup>, String> {
    // let config_dir = env::var("HOME").unwrap() + "/.timewarrior/aggregate";
    // //let workgroups = Vec::new();
    debug!("start {} end {}", start, end);
    let start = NaiveDateTime::parse_from_str(start, "%Y%m%dT%H%M%SZ").unwrap();
    let end = NaiveDateTime::parse_from_str(end, "%Y%m%dT%H%M%SZ").unwrap();
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

#[cfg(test)]
mod test {
    use super::get_workgroups;

    // #[test]
    // fn get_workgroups_for_day() {
    //     get_workgroups("20210723T183000Z", "20210724T183000Z").unwrap();
    // }
    //
    // #[test]
    // fn get_workgroups_for_week() {
    //     get_workgroups("20210723T183000Z", "20210730T183000Z").unwrap();
    // }

    #[test]
    fn get_workgroups_unsupported() {
        assert!(get_workgroups("20210723T183000Z", "20210729T183000Z").is_err());
    }
}
