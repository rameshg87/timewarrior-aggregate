use crate::tagset::TagSet;
use chrono::Duration;
use json::JsonValue;
use std::env;
use std::fmt;
use std::ops::Add;

use crate::twentry::TimeWarriorEntry;

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
        let skip_allocated = match env::var("SKIP_ALLOCATED") {
            Ok(_) => true,
            Err(_) => false,
        };
        if skip_allocated {
            write!(f, "| {0: <20} | {1: <15}", tags_sorted.join(" "), spent)
        } else {
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
