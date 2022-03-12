pub mod server;
pub mod sys;
mod client;
mod init;

fn main() {
    use std::process;

    if process::id() == 1 {
        init::main();
    } else {
        client::lazycmd();
    }
}

