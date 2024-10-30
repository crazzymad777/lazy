use crate::omicron::command::CommandBuilder;
use crate::omicron::Process;

pub struct UnitDescriptor {
    original_name: String,
    image: CommandBuilder
}

impl UnitDescriptor {
    pub fn new(name: &str, image: CommandBuilder) -> UnitDescriptor {
        UnitDescriptor {original_name: String::from(name), image}
    }

    pub fn get_name(&self) -> String {
        self.original_name.clone()
    }

    pub fn spawn(&self) -> Result<Process, String> {
        self.image.spawn()
    }
}
