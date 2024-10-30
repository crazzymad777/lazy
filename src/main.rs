pub mod server;
pub mod sys;
mod client;
mod init;
mod omicron;
mod unit;
mod warden;

fn main() {
    use std::process;

    if process::id() == 1 {
        let _ = init::main();
    } else {
        let _ = client::lazycmd();
    }
}

