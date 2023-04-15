use std::{cell::RefCell, rc::Rc};

use duckscript::types::command::{Command, CommandResult};

use super::store::{MikroTikConnection, MikroTikStore};

#[derive(Clone)]
pub struct AddRouter {
    store: Rc<RefCell<MikroTikStore>>,
}

impl AddRouter {
    pub fn new(store: Rc<RefCell<MikroTikStore>>) -> Self {
        Self { store }
    }
}

impl Command for AddRouter {
    fn name(&self) -> String {
        "add_router".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let mut store = self.store.borrow_mut();

        let router = arguments[0].to_string();
        let username = arguments[1].to_string();
        let password = arguments[2].to_string();

        store.connections.insert(
            router.clone(),
            MikroTikConnection {
                username,
                password,
                api: None,
            },
        );
        store.current = Some(router.clone());

        CommandResult::Continue(Some(router))
    }
}
