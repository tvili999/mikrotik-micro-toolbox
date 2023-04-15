use duckscript::types::command::{Command, CommandResult};

#[derive(Clone)]
pub struct Inc {}

impl Command for Inc {
    fn name(&self) -> String {
        "inc".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        CommandResult::Continue(Some(format!(
            "{}",
            arguments[0].parse::<i64>().unwrap() + 1
        )))
    }
}
