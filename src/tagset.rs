use log::debug;
use std::collections::HashSet;
use std::fs;

pub struct TagSet {
    tagset: HashSet<String>,
}

impl TagSet {
    fn num_matches(&self, other: &TagSet) -> usize {
        let intersection: HashSet<_> = self.tagset.intersection(&(other.tagset)).collect();
        intersection.len()
    }
}

pub struct TagSets {
    tagsets: Vec<TagSet>,
}

impl TagSets {
    fn from(tags_file_path: &String) -> Result<Self, &str> {
        let tags_file_contents =
            fs::read_to_string(tags_file_path).expect("Unable to read tags file");
        let parsed_json = json::parse(&tags_file_contents).expect("Unable to parse json file");
        let mut tagsets = TagSets {
            tagsets: Vec::new(),
        };
        for tag_group in parsed_json.members() {
            let mut tagset = HashSet::new();
            for tag in tag_group.members() {
                tagset.insert(tag.as_str().expect("Unable to parse string").to_string());
            }
            debug!("found tag_set {:?}", tagset);
            tagsets.tagsets.push(TagSet { tagset });
        }
        debug!("found {} tags from tags file", tagsets.tagsets.len());
        Ok(tagsets)
    }

    fn all_match(&self, other_tagset: &TagSet) -> Option<&TagSet> {
        let mut all_match_tagset = None;
        for tagset in self.tagsets.iter() {
            if tagset.num_matches(other_tagset) == tagset.tagset.len() {
                all_match_tagset = Some(tagset);
                break;
            }
        }
        all_match_tagset
    }
}
