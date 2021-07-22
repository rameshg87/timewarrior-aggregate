use crate::tagset::TagSet;
use chrono::Duration;
use json::JsonValue;

use crate::twentry::TimeWarriorEntry;

pub struct WorkGroup {
    tagset: TagSet,
    time_allocated: Duration,
    time_spent: Duration,
}

impl WorkGroup {
    fn parse_from_json_value(jv: &JsonValue) -> Self {
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

    fn matches(&self, twe: &TimeWarriorEntry) -> bool {
        self.tagset.has_all_tags_of(&twe.tagset)
    }
}
