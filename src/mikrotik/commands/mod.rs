use std::{cell::RefCell, rc::Rc};

use duckscript::types::command::Command;

use super::connections::store::MikroTikStore;

mod hostname;
mod ping;

pub fn load_commands(store: &Rc<RefCell<MikroTikStore>>) -> Vec<Box<dyn Command>> {
    vec![
        Box::new(ping::Ping::new(store.clone())),
        Box::new(hostname::Hostname::new(store.clone())),
    ]
}
