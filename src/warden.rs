use crate::unit::UnitDescriptor;
use std::collections::HashMap;
use crate::unit::Unit;

pub struct Warden {
    services: HashMap<String, Unit>,
    count: HashMap<String, u32>
}

impl Warden {
    pub fn new(services: HashMap<String, Unit>, count: HashMap<String, u32>) -> Warden {
        Warden {services, count}
    }

    pub fn generate_name(&mut self, servicename: String) -> (String, u32) {
        let mut string: String = servicename.clone();
        let option = self.count.get_mut(&string);
        let mut x = 0;
        if option.is_some() {
             let value = option.unwrap();
             string = string + &value.to_string();
             x = *value;
             *value = *value+1;
        } else {
            self.count.insert(string.clone(), 1);
            x = 1;
        }
        (string, x)
    }

    pub fn save(&mut self, servicename: String, unit: Unit) {
        self.services.insert(servicename, unit);
    }

    pub fn spawn_supervised(&mut self, descriptor: &UnitDescriptor) {
        let tuple = self.generate_name(descriptor.get_name());
        let unit = descriptor.spawn(Some(tuple.1));
        if let Some(x) = unit.get_process() {
            println!("Lazie Warden: spawn {} {}", tuple.0, x.id());
        } else {
            eprintln!("Lazie Warden: detected failed spawn of {}", tuple.0);
        }
        self.save(tuple.0, unit);
    }
}
