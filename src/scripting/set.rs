use duckscript::types::command::{Command, CommandResult};

#[derive(Clone)]
pub struct Set {}

impl Command for Set {
    fn name(&self) -> String {
        "set".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        CommandResult::Continue(Some(arguments.join(" ").to_string()))
    }
}
