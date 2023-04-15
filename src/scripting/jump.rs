use duckscript::types::command::{Command, CommandResult, GoToValue};

#[derive(Clone)]
pub struct Jump {}

impl Command for Jump {
    fn name(&self) -> String {
        "j".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let mut condition = false;
        let mut label = None;

        if arguments[0] == "eq" {
            condition = arguments[1] == arguments[2];
            label = Some(arguments[3].to_string());
        }
        if arguments[0] == "ne" {
            condition = arguments[1] != arguments[2];
            label = Some(arguments[3].to_string());
        }
        if arguments[0] == "gt" {
            condition = arguments[1].parse::<i64>().unwrap() > arguments[2].parse::<i64>().unwrap();
            label = Some(arguments[3].to_string());
        }
        if arguments[0] == "lt" {
            condition = arguments[1].parse::<i64>().unwrap() < arguments[2].parse::<i64>().unwrap();
            label = Some(arguments[3].to_string());
        }
        if arguments[0] == "ge" {
            condition =
                arguments[1].parse::<i64>().unwrap() >= arguments[2].parse::<i64>().unwrap();
            label = Some(arguments[3].to_string());
        }
        if arguments[0] == "le" {
            condition = arguments[1].parse::<i64>().unwrap() < arguments[2].parse::<i64>().unwrap();
            label = Some(arguments[3].to_string());
        }
        if arguments[0] == "t" {
            condition = !(arguments[1].is_empty()
                || arguments[1] == "0"
                || arguments[1] == "false"
                || arguments[1] == "no");
            label = Some(arguments[2].to_string());
        }
        if arguments[0] == "f" {
            condition = arguments[1].is_empty()
                || arguments[1] == "0"
                || arguments[1] == "false"
                || arguments[1] == "no";
            label = Some(arguments[2].to_string());
        }

        if let Some(label) = label {
            if condition {
                CommandResult::GoTo(None, GoToValue::Label(label))
            } else {
                CommandResult::Continue(None)
            }
        } else {
            CommandResult::Error(format!("{} is not an operation", arguments[0]))
        }
    }
}
