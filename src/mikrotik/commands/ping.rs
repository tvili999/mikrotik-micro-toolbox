use std::{cell::RefCell, rc::Rc};

use crate::mikrotik::connections::store::MikroTikStore;

use duckscript::types::command::{Command, CommandResult};

#[derive(Clone)]
pub struct Ping {
    store: Rc<RefCell<MikroTikStore>>,
}

impl Ping {
    pub fn new(store: Rc<RefCell<MikroTikStore>>) -> Self {
        Self { store }
    }
}

impl Command for Ping {
    fn name(&self) -> String {
        "ping".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let mut store = self.store.borrow_mut();
        let api = store.connect();

        let resp = api
            .talk(&[
                "/ping",
                format!("=address={}", arguments[0]).as_str(),
                "=count=1",
            ])
            .unwrap();
        println!("{:?}", resp);

        CommandResult::Continue(Some("".to_string()))
    }
}
