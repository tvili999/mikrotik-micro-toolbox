use std::{thread::sleep, time::Duration};

use duckscript::types::command::{Command, CommandResult};

#[derive(Clone)]
pub struct Sleep {}

impl Command for Sleep {
    fn name(&self) -> String {
        "sleep".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let millis = arguments[0].parse::<u64>().unwrap();
        sleep(Duration::from_millis(millis));
        CommandResult::Continue(Some(arguments.len().to_string()))
    }
}
