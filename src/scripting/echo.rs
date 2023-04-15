use duckscript::types::command::{Command, CommandResult};

#[derive(Clone)]
pub struct Echo {}

impl Command for Echo {
    fn name(&self) -> String {
        "echo".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        for argument in &arguments {
            print!("{} ", argument);
        }

        println!();

        CommandResult::Continue(Some(arguments.len().to_string()))
    }
}
