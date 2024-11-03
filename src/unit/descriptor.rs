use crate::omicron::command::CommandBuilder;
use crate::unit::Unit;

pub struct UnitDescriptor {
    original_name: String,
    image: CommandBuilder
}

impl Clone for UnitDescriptor {
    fn clone(&self) -> Self {
        UnitDescriptor::new(self.original_name.clone(), self.image.clone())
    }
}

impl UnitDescriptor {
    pub fn new(name: String, image: CommandBuilder) -> UnitDescriptor {
        UnitDescriptor {original_name: name, image}
    }

    pub fn get_name(&self) -> String {
        self.original_name.clone()
    }

    pub fn spawn(&self, number: Option<u32>) -> Unit {
        use crate::omicron::ShellCommand;
        let x = self.image.spawn();
        let mut unit = Unit::new(self.clone(), number);
        if let Ok(x) = x {
            unit.assign(x);
        } else {
            eprintln!("UnitDescriptor spawn failed: {}", x.err().unwrap());
        }
        unit
    }
}
