use std::{cell::RefCell, rc::Rc};

use super::api::RouterOSApi;
use duckscript::types::command::{Command, CommandResult};

#[derive(Clone)]
pub struct PingCommand {
    pub api: Rc<RefCell<RouterOSApi>>,
}

impl Command for PingCommand {
    fn name(&self) -> String {
        "ping".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let mut api = self.api.borrow_mut();
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
