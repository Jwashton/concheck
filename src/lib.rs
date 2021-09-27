use std::collections::{HashMap, HashSet};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Services {
    ssh: Option<bool>,
    http: Option<bool>,
    https: Option<bool>,
    mariadb: Option<bool>,
    postgresql: Option<bool>,
    other: Option<HashMap<u16, bool>>,
}

impl Services {
    pub fn to_port_checks(&self) -> HashMap<u16, bool> {
        let mut ports = match &self.other {
            Some(values) => values.clone(),
            _ => HashMap::new(),
        };

        if let Some(enabled) = &self.ssh {
            ports.insert(22, *enabled);
        }

        if let Some(enabled) = &self.http {
            ports.insert(80, *enabled);
        }

        if let Some(enabled) = &self.https {
            ports.insert(443, *enabled);
        }

        if let Some(enabled) = &self.mariadb {
            ports.insert(3306, *enabled);
        }

        if let Some(enabled) = &self.postgresql {
            ports.insert(5432, *enabled);
        }

        ports
    }
}

#[derive(Deserialize, Debug)]
pub struct Role {
    services: Services,
    servers: Vec<String>,
}

impl Role {
    pub fn services(&self) -> &Services {
        &self.services
    }

    pub fn ports(&self) -> HashSet<u16> {
        self.services.to_port_checks().keys().cloned().collect()
    }

    pub fn servers(&self) -> &Vec<String> {
        &self.servers
    }
}