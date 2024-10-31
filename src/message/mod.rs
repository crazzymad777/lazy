use crate::unit::UnitDescriptor;

/*
 * Message:
 * Message must be serializble
 * Message should be possibly implemented in other programming languages
 * Message Types:
 *  - Shutdown
 *  - NewService
 *  - DeleteService
 *  - StartService
 *  - StopService
 *  - RestartService
 *  - ReloadService
 *  - StatusService
 *  - ListOfServices
 *  - CloseConnection
 * Replies:
 *  - Fail (reason)
 *  - Ok (list, status, none)
 */

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
