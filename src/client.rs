use std::os::unix::net::UnixStream;

pub fn lazycmd() -> std::io::Result<()> {
   use std::env;
   use std::process;
   use std::io::Write;

   println!("My pid is {}", process::id());

   let args: Vec<String> = env::args().collect();
   let arg = &args[1];
   let mut stream = UnixStream::connect("/run/lazy")?;
   stream.write_all(arg.as_bytes())?;
   Ok(())
}

