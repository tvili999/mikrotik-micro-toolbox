use std::{cell::RefCell, rc::Rc};

use duckscript::types::command::{Command, CommandResult};

use super::store::MikroTikStore;

#[derive(Clone)]
pub struct CloseRouter {
    store: Rc<RefCell<MikroTikStore>>,
}

impl CloseRouter {
    pub fn new(store: Rc<RefCell<MikroTikStore>>) -> Self {
        Self { store }
    }
}

impl Command for CloseRouter {
    fn name(&self) -> String {
        "close_router".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        let mut store = self.store.borrow_mut();

        let current = if let Some(current) = &store.current {
            current.to_string()
        } else {
            return CommandResult::Continue(None);
        };

        if let Some(connection) = store.connections.get_mut(&current) {
            connection.api = None;
        }

        store.current = None;

        CommandResult::Continue(None)
    }
}
