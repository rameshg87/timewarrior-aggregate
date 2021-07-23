use chrono::{Local, NaiveDateTime};

use crate::tagset::TagSet;
use json::JsonValue;

#[derive(Debug)]
pub struct TimeWarriorEntry {
    pub tagset: TagSet,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

impl TimeWarriorEntry {
    pub fn parse_from_json_value(jv: &JsonValue) -> Self {
        // Get the tags from the entry.
        let tagset = TagSet::parse_from_json_value(&jv);

        // Get the starting time for the entry.
        let start = jv["start"].as_str().unwrap();
        let start = NaiveDateTime::parse_from_str(start, "%Y%m%dT%H%M%SZ").unwrap();

        // Get the ending time for the entry. If end doesn't exist in the entry, then current time
        // is the ending time for the entry (it is still going on).
        let end;
        if let Some(value) = jv["end"].as_str() {
            end = NaiveDateTime::parse_from_str(value, "%Y%m%dT%H%M%SZ").unwrap();
        } else {
            end = Local::now().naive_utc();
        }

        TimeWarriorEntry { tagset, start, end }
    }

    pub fn duration(&self) -> chrono::Duration {
        self.end.signed_duration_since(self.start)
    }
}

#[cfg(test)]
mod test {
    use super::TimeWarriorEntry;

    #[test]
    fn parse_from_json_value_with_end() {
        let s = "{\"id\":3,\"start\":\"20210722T152328Z\",\"end\":\"20210722T153753Z\",\"tags\":[\"Rust talks\",\"learning\",\"personal\"]}";
        let jv = json::parse(&s).unwrap();
        let twe = TimeWarriorEntry::parse_from_json_value(&jv);

        assert_eq!(twe.tagset.tags.len(), 3);
        assert!(twe.tagset.tags.contains("personal"));
        assert!(twe.tagset.tags.contains("learning"));
        assert!(twe.tagset.tags.contains("Rust talks"));

        assert_eq!(
            twe.start.format("%Y-%m-%d %H:%M:%S").to_string(),
            "2021-07-22 15:23:28"
        );
        assert_eq!(
            twe.end.format("%Y-%m-%d %H:%M:%S").to_string(),
            "2021-07-22 15:37:53"
        );
    }

    #[test]
    fn parse_from_json_value_without_end() {
        let s = "{\"id\":3,\"start\":\"20210722T152328Z\",\"tags\":[\"Rust talks\",\"learning\",\"personal\"]}";
        let jv = json::parse(&s).unwrap();
        let twe = TimeWarriorEntry::parse_from_json_value(&jv);

        assert_eq!(twe.tagset.tags.len(), 3);
        assert!(twe.tagset.tags.contains("personal"));
        assert!(twe.tagset.tags.contains("learning"));
        assert!(twe.tagset.tags.contains("Rust talks"));

        assert_eq!(
            twe.start.format("%Y-%m-%d %H:%M:%S").to_string(),
            "2021-07-22 15:23:28"
        );
    }

    #[test]
    fn duration() {
        let s = "{\"id\":3,\"start\":\"20210722T152328Z\",\"end\":\"20210722T153753Z\",\"tags\":[\"Rust talks\",\"learning\",\"personal\"]}";
        let jv = json::parse(&s).unwrap();
        let twe = TimeWarriorEntry::parse_from_json_value(&jv);
        assert_eq!(twe.duration().num_seconds(), 865);
    }
}
