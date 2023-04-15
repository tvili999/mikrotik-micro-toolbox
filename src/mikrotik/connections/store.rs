use std::collections::HashMap;

use crate::mikrotik::api::RouterOSApi;

pub struct MikroTikConnection {
    pub username: String,
    pub password: String,
    pub api: Option<RouterOSApi>,
}

pub struct MikroTikStore {
    pub connections: HashMap<String, MikroTikConnection>,
    pub current: Option<String>,
}

impl MikroTikStore {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            current: None,
        }
    }

    pub fn connect(&mut self) -> &mut RouterOSApi {
        let current = if let Some(key) = &self.current {
            key
        } else {
            panic!("No connection used");
        };

        let mut connection = if let Some(connection) = self.connections.get_mut(current) {
            connection
        } else {
            panic!("No connection used");
        };

        if let None = connection.api {
            let mut api = RouterOSApi::new(current.as_str()).unwrap();
            api.login(&connection.username, &connection.password)
                .unwrap();

            connection.api = Some(api);
        }

        if let Some(api) = &mut connection.api {
            return api;
        } else {
            panic!("Shouldn't happen");
        }
    }
}
