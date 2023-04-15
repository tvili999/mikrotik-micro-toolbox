use std::{
    collections::{BTreeMap, LinkedList},
    time::SystemTime,
};

pub struct ScheduledEntry {
    pub run_time: SystemTime,
    pub labels: LinkedList<String>,
}
pub struct SchedulerStore {
    pub entries: BTreeMap<SystemTime, ScheduledEntry>,
    pub current_entry: Option<ScheduledEntry>,
}

impl SchedulerStore {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            current_entry: None,
        }
    }
}
