use crate::unit::UnitDescriptor;
use std::collections::HashMap;
use crate::unit::Unit;

use std::sync::mpsc::Receiver;
use std::sync::mpsc;

use crate::message::Message;

pub fn spawn_warden(rx: Receiver<Message>) {
    let mut warden = Warden::new(HashMap::new(), HashMap::new());
    let mut command = rx.recv();
    while let Ok(x) = command {
        command_warden(&mut warden, x);
        command = rx.recv();
    }
    // println!("Warden exits...");
}

fn command_warden(employee: &mut Warden, value: Message) {
    use crate::sys;
    use crate::message::MessageCommand::ExecService;
    use crate::message::MessageCommand;
    use crate::message::MessagePayload::Shutdown;
    if value.cmd == ExecService {
        employee.spawn_supervised(value.get_descriptor().unwrap());
    } else if value.cmd == MessageCommand::Shutdown {
        sys::reboot(value.get_shutdown().unwrap());
    }
}

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

        if option.is_some() {
            let value = option.unwrap();
            string = string + &value.to_string();
            let x = *value;
            *value = *value+1;
            (string, x)
        } else {
            self.count.insert(string.clone(), 1);
            (string, 1)
        }
    }

    pub fn save(&mut self, servicename: String, unit: Unit) {
        self.services.insert(servicename, unit);
    }

    pub fn spawn_supervised(&mut self, descriptor: UnitDescriptor) {
        let tuple = self.generate_name(descriptor.get_name());
        let unit = descriptor.spawn(Some(tuple.1));
        if let Some(x) = unit.get_process() {
            println!("Lazie Warden: spawn {} {}", tuple.0, x.id());
        } else {
            eprintln!("Lazie Warden: detected failed spawn of {}", tuple.0);
        }
        self.save(tuple.0, unit);
    }

    pub fn stop_unit(&mut self, service_name: String) {
        let option = self.services.get(&service_name);
        if let Some(unit) = option {
            if let Some(x) = unit.get_process() {
                let _ = x.signal_to_group(libc::SIGTERM);
            }
        }
    }

    pub fn restart_unit(&mut self, service_name: String) {
        self.stop_unit(service_name);
        // mark unit for restart?
    }
}
