mod add;
mod echo;
mod env;
mod goto;
mod jump;
mod set;
mod sleep;

use duckscript::types::runtime::Context;

pub fn create_context() -> Context {
    let mut context = Context::new();
    context.commands.set(Box::new(goto::Goto {})).unwrap();
    context.commands.set(Box::new(echo::Echo {})).unwrap();
    context.commands.set(Box::new(sleep::Sleep {})).unwrap();
    context.commands.set(Box::new(set::Set {})).unwrap();
    context.commands.set(Box::new(add::Add {})).unwrap();
    context.commands.set(Box::new(env::Env {})).unwrap();
    context.commands.set(Box::new(jump::Jump {})).unwrap();

    return context;
}
