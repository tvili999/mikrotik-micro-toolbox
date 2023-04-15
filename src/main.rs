mod mikrotik;
mod scripting;

use duckscript::runner::run_script_file;
use mikrotik::api::RouterOSApi;
use mikrotik::utilities::PingCommand;
use scripting::create_context;
use std::{cell::RefCell, rc::Rc};

fn main() {
    let mut api = RouterOSApi::new("10.1.0.1:8728").unwrap();
    api.login("api_user", "example_pass").unwrap();

    let api_rc = Rc::new(RefCell::new(api));

    let mut context = create_context();
    context
        .commands
        .set(Box::new(PingCommand { api: api_rc }))
        .unwrap();

    run_script_file("./test.ds", context).unwrap();
}
