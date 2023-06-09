use std::{cell::RefCell, rc::Rc};

use crate::mikrotik::connections::store::MikroTikStore;

use duckscript::types::command::{Command, CommandResult};

#[derive(Clone)]
pub struct Hostname {
    store: Rc<RefCell<MikroTikStore>>,
}

impl Hostname {
    pub fn new(store: Rc<RefCell<MikroTikStore>>) -> Self {
        Self { store }
    }
}

impl Command for Hostname {
    fn name(&self) -> String {
        "hostname".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        let mut store = self.store.borrow_mut();
        let api = store.connect();

        let resp = api.talk(&["/system/identity/print"]).unwrap();
        println!("{:?}", resp);

        CommandResult::Continue(Some("".to_string()))
    }
}
