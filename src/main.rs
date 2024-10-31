pub mod server; // Socket listener
pub mod sys; // OS-specific calls
mod client; // client of socket
mod init; // init parser
mod omicron; // Omicron Shell
mod unit; // unit description
mod warden; // host units
mod message; // for data exchange between processes and threads

fn main() {
    use std::process;

    if process::id() == 1 {
        // Run init process
        let _ = init::main();
    } else {
        // Connect to socket and send command to Lazie Init process
        let _ = client::lazycmd();
    }
}

