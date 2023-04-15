mod mikrotik;
mod scheduler;
mod scripting;

use duckscript::runner::run_script_file;
use scripting::create_context;

fn main() {
    let mut context = create_context();

    let scheduler = scheduler::Scheduler::new();
    let mikrotik = mikrotik::MikroTik::new();

    scheduler.load_commands(&mut context.commands).unwrap();
    mikrotik.load_commands(&mut context.commands).unwrap();

    run_script_file("./test.ds", context).unwrap();
}
