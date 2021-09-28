use std::collections::HashSet;
use std::error::Error;
use std::fs::File;

use serde::Deserialize;
use serde_yaml;

use crate::role::Role;

#[derive(Deserialize)]
pub struct Inventory {
    pub roles: Vec<Role>,
}

impl Inventory {
    pub fn from_file(filename: &str) -> Result<Inventory, Box<dyn Error>> {
        let file = File::open(filename)?;
        let inventory = serde_yaml::from_reader(file)?;

        Ok(inventory)
    }

    pub fn all_ports(&self) -> Vec<u16> {
        let mut all_ports = self
            .roles
            .iter()
            .fold(HashSet::new(), |ports, role| &ports | &role.ports())
            .into_iter()
            .collect::<Vec<u16>>();

        all_ports.sort();

        all_ports
    }

    fn all_server_names(&self) -> Vec<&String> {
        self.roles
            .iter()
            .map(|role| role.servers())
            .flatten()
            .collect::<Vec<&String>>()
    }

    pub fn length_of_longest_server_name(&self) -> usize {
        self.all_server_names()
            .iter()
            .map(|name| name.chars().count())
            .max()
            .unwrap()
    }
}
