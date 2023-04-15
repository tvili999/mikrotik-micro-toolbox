use std::{cell::RefCell, rc::Rc, thread::sleep, time::SystemTime};

use duckscript::types::command::{Command, CommandResult, GoToValue};

use super::store::SchedulerStore;

#[derive(Clone)]
pub struct RunSchedulerCommand {
    pub store: Rc<RefCell<SchedulerStore>>,
}

impl RunSchedulerCommand {
    fn load_current_entry(&self, store: &mut SchedulerStore) {
        if let Some(entry) = &store.current_entry {
            if entry.labels.len() > 0 {
                return;
            }
        }

        if let Some((_, value)) = store.entries.pop_first() {
            store.current_entry = Some(value);
        }
    }
}

impl Command for RunSchedulerCommand {
    fn name(&self) -> String {
        "run_scheduler".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        let mut store = self.store.borrow_mut();
        self.load_current_entry(&mut store);

        if let Some(entry) = &mut store.current_entry {
            let now = SystemTime::now();
            if now < entry.run_time {
                sleep(entry.run_time.duration_since(now).unwrap());
            }
            if let Some(label) = entry.labels.pop_front() {
                return CommandResult::GoTo(None, GoToValue::Label(label));
            }
        }

        CommandResult::Continue(None)
    }
}
