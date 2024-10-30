use crate::unit::UnitDescriptor;
use std::collections::HashMap;
use crate::omicron::Process;

pub struct Warden {
    services: HashMap<String, Process>,
    count: HashMap<String, u32>
}

impl Warden {
    pub fn new(services: HashMap<String, Process>, count: HashMap<String, u32>) -> Warden {
        Warden {services, count}
    }

    pub fn generate_name(&mut self, servicename: String) -> String {
        let mut string: String = servicename.clone();
        let option = self.count.get_mut(&string);
        if option.is_some() {
             let value = option.unwrap();
             string = string + &value.to_string();
             *value = *value+1;
        } else {
            self.count.insert(string.clone(), 1);
        }
        string
    }

    pub fn save(&mut self, servicename: String, child: Process) {
        self.services.insert(servicename, child);
    }

    pub fn spawn_supervised(&mut self, descriptor: &UnitDescriptor) {
        let name = self.generate_name(descriptor.get_name());
        if let Ok(child) = descriptor.spawn() {
            println!("Lazie Warden: spawn {} {}", name, child.id());
            self.save(name, child);
        } else {
            println!("Lazie Warden: {} failed", name);
        }
    }
}
