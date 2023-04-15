mod mikrotik;
mod scheduler;
mod scripting;

use duckscript::runner::run_script_file;
use scripting::create_context;

fn main() {
    // let mut api = RouterOSApi::new("10.1.0.1:8728").unwrap();
    // api.login("api_user", "example_pass").unwrap();

    // let api_rc = Rc::new(RefCell::new(api));

    let mut context = create_context();

    let scheduler = scheduler::Scheduler::new();
    scheduler.load_commands(&mut context.commands).unwrap();

    run_script_file("./test.ds", context).unwrap();
}
