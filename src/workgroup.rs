use crate::tagset::TagSet;
use chrono::Duration;
use json::JsonValue;
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
