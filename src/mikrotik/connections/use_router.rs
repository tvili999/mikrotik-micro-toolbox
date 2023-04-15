use std::{cell::RefCell, rc::Rc};

use duckscript::types::command::{Command, CommandResult};

use super::store::MikroTikStore;

#[derive(Clone)]
pub struct UseRouterCommand {
    pub store: Rc<RefCell<MikroTikStore>>,
}

impl Command for UseRouterCommand {
    fn name(&self) -> String {
        "use_router".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let mut store = self.store.borrow_mut();

        let router = arguments[0].to_string();

        if store.connections.contains_key(&router) {
            store.current = Some(router);
        }

        CommandResult::Continue(None)
    }
}
