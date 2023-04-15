use duckscript::types::command::{Command, CommandResult, GoToValue};

#[derive(Clone)]
pub struct Goto {}

impl Command for Goto {
    fn name(&self) -> String {
        "goto".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let label = arguments[0].clone();
        CommandResult::GoTo(None, GoToValue::Label(label))
    }
}
