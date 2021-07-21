use crate::tagset::TagSet;
use chrono::Duration;
use std::collections::HashSet;

pub struct WorkGroup {
    tagset: TagSet,
    time_spent: Duration,
    time_allocated: Duration,
}
