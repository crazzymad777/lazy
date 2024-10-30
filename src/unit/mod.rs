pub mod descriptor;
pub use crate::unit::descriptor::UnitDescriptor;
use crate::omicron::Process;

pub struct Unit {
    descriptor: UnitDescriptor,
    process: Option<Process>,
    number: Option<u32>
}

impl Unit {
    pub fn new(descriptor: UnitDescriptor, number: Option<u32>) -> Unit {
        Unit {descriptor, process: None, number}
    }

    pub fn assign(&mut self, process: Process) {
        self.process = Some(process);
    }

    pub fn get_process(&self) -> Option<Process> {
        self.process
    }
}
