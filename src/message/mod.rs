use crate::unit::UnitDescriptor;

#[derive(PartialEq)]
pub enum MessageCommand {
    ExecService,
    SaveService,
    Shutdown
}

pub enum MessagePayload {
    Unit,
    Descriptor(UnitDescriptor),
    Shutdown(String)
}

pub struct Message {
    pub cmd: MessageCommand,
    payload: MessagePayload
}

impl Message {
    pub fn new(cmd: MessageCommand, payload: MessagePayload) -> Message {
        Message {cmd, payload}
    }

    pub fn get_descriptor(self) -> Option<UnitDescriptor> {
        if let MessagePayload::Descriptor(x) = self.payload {
            Some(x)
        } else {
            None
        }
    }

    pub fn get_shutdown(self) -> Option<String> {
        if let MessagePayload::Shutdown(x) = self.payload {
            Some(x)
        } else {
            None
        }
    }
}
