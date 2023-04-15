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
        for command in commands::load_commands(&self.store) {
            commands.set(command)?;
        }
        for command in connections::load_commands(&self.store) {
            commands.set(command)?;
        }

        Ok(())
    }
}
