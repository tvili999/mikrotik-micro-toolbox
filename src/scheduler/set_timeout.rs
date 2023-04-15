use std::{
    cell::RefCell,
    collections::LinkedList,
    rc::Rc,
    time::{Duration, SystemTime},
};

use duckscript::types::command::{Command, CommandResult};

use super::store::{ScheduledEntry, SchedulerStore};

#[derive(Clone)]
pub struct SetTimeoutCommand {
    pub store: Rc<RefCell<SchedulerStore>>,
}

impl Command for SetTimeoutCommand {
    fn name(&self) -> String {
        "set_timeout".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let mut store = self.store.borrow_mut();

        let delay = arguments[0].parse::<u64>().unwrap();
        let label = arguments[1].clone();

        let mut time = SystemTime::now();
        time += Duration::from_millis(delay);

        if let Some(entry) = store.entries.get_mut(&time) {
            entry.labels.push_back(label);
        } else {
            store.entries.insert(
                time,
                ScheduledEntry {
                    run_time: time,
                    labels: LinkedList::from([label]),
                },
            );
        }

        CommandResult::Continue(None)
    }
}
