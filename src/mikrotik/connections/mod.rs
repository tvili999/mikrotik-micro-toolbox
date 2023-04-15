use std::{cell::RefCell, rc::Rc};

use duckscript::types::command::Command;

use self::store::MikroTikStore;

pub mod add_router;
pub mod close_router;
pub mod store;
pub mod use_router;

pub fn load_commands(store: &Rc<RefCell<MikroTikStore>>) -> Vec<Box<dyn Command>> {
    vec![
        Box::new(add_router::AddRouter::new(store.clone())),
        Box::new(use_router::UseRouter::new(store.clone())),
        Box::new(close_router::CloseRouter::new(store.clone())),
    ]
}
