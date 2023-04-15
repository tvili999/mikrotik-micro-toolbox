use duckscript::types::command::{Command, CommandResult};

#[derive(Clone)]
pub struct Env {}

impl Command for Env {
    fn name(&self) -> String {
        "env".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if let Ok(value) = std::env::var(&arguments[0]) {
            return CommandResult::Continue(Some(value));
        }

        if let Some(value) = arguments.get(1) {
            return CommandResult::Continue(Some(value.to_string()));
        }

        CommandResult::Continue(None)
    }
}
