use json::JsonValue;
use std::collections::HashSet;

#[derive(Debug)]
pub struct TagSet {
    pub tags: HashSet<String>,
}

impl TagSet {
    pub fn parse_from_json_value(jv: &JsonValue) -> Self {
        let mut tags = HashSet::new();
        for tag in jv["tags"].members() {
            tags.insert(tag.as_str().expect("Unable to parse string").to_string());
        }
        TagSet { tags }
    }

    pub fn has_all_tags_of(&self, other: &TagSet) -> bool {
        let intersection: HashSet<_> = self.tags.intersection(&(other.tags)).collect();
        intersection.len() == self.tags.len()
    }
}

#[cfg(test)]
mod test {
    use super::TagSet;

    #[test]
    fn parse_from_json_value() {
        let s = "{ \"tags\": [ \"office\", \"project\" ] }";
        let jv = json::parse(&s).unwrap();
        let tagset = TagSet::parse_from_json_value(&jv);

        assert_eq!(tagset.tags.len(), 2);
        assert!(tagset.tags.contains("office"));
        assert!(tagset.tags.contains("project"));
    }

    #[test]
    fn has_all_tags_of() {
        let s = "{ \"tags\": [ \"office\", \"project\" ] }";
        let jv = json::parse(&s).unwrap();
        let tagset1 = TagSet::parse_from_json_value(&jv);

        let s = "{ \"tags\": [ \"office\", \"project\", \"foo\" ] }";
        let jv = json::parse(&s).unwrap();
        let tagset2 = TagSet::parse_from_json_value(&jv);

        let s = "{ \"tags\": [ \"office\", \"maintenance\" ] }";
        let jv = json::parse(&s).unwrap();
        let tagset3 = TagSet::parse_from_json_value(&jv);

        assert!(tagset1.has_all_tags_of(&tagset2));
        assert!(!tagset1.has_all_tags_of(&tagset3));
    }
}
