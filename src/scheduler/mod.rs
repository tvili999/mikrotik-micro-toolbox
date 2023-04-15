mod run_scheduler;
mod set_timeout;
mod store;

use std::cell::RefCell;
use std::rc::Rc;

use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

use self::store::SchedulerStore;

pub struct Scheduler {
    store: Rc<RefCell<SchedulerStore>>,
}

impl Scheduler {
    pub fn new() -> Self {
        let store = Rc::new(RefCell::new(SchedulerStore::new()));
        Self { store }
    }

    pub fn load_commands(&self, commands: &mut Commands) -> Result<(), ScriptError> {
        commands.set(Box::new(set_timeout::SetTimeoutCommand {
            store: self.store.clone(),
        }))?;
        commands.set(Box::new(run_scheduler::RunSchedulerCommand {
            store: self.store.clone(),
        }))?;

        Ok(())
    }
}
