use std::{cell::RefCell, rc::Rc};

use duckscript::types::{command::Commands, error::ScriptError};

use self::connections::store::MikroTikStore;

mod api;
mod commands;
mod connections;

pub struct MikroTik {
    store: Rc<RefCell<MikroTikStore>>,
}

impl MikroTik {
    pub fn new() -> Self {
        let store = Rc::new(RefCell::new(MikroTikStore::new()));
        Self { store }
    }

    pub fn load_commands(&self, commands: &mut Commands) -> Result<(), ScriptError> {
        commands.set(Box::new(connections::add_router::AddRouterCommand {
            store: self.store.clone(),
        }))?;
        commands.set(Box::new(connections::use_router::UseRouterCommand {
            store: self.store.clone(),
        }))?;
        commands.set(Box::new(connections::close_router::CloseRouterCommand {
            store: self.store.clone(),
        }))?;
        commands.set(Box::new(commands::ping::PingCommand {
            store: self.store.clone(),
        }))?;
        commands.set(Box::new(commands::hostname::HostnameCommand {
            store: self.store.clone(),
        }))?;

        Ok(())
    }
}
