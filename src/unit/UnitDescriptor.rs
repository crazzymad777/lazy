use crate::omicron::command::CommandBuilder;

pub struct UnitDescriptor {
    original_name: String,
    image: CommandBuilder
}

impl UnitDescriptor {
    pub fn new(name: &str, image: CommandBuilder) -> UnitDescriptor {
        UnitDescriptor {original_name: String::from(name), image}
    }

    pub fn spawn() {

    }
}
