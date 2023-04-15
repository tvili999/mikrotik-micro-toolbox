use std::{cell::RefCell, rc::Rc};

use duckscript::types::command::{Command, CommandResult};

use super::store::MikroTikStore;

#[derive(Clone)]
pub struct CloseRouterCommand {
    pub store: Rc<RefCell<MikroTikStore>>,
}

impl Command for CloseRouterCommand {
    fn name(&self) -> String {
        "close_router".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let mut store = self.store.borrow_mut();

        let router = arguments[0].to_string();

        if let Some(connection) = store.connections.get_mut(&router) {
            connection.api = None;
        }

        if let Some(current) = &store.current {
            if current == &router {
                store.current = None;
            }
        }

        CommandResult::Continue(None)
    }
}
