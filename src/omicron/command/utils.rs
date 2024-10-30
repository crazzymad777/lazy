use crate::omicron::Process;

// Wrapper for execvp

// return pid?
// 0 - for child
// <0 - error
// >0 - parent, Process::new()
pub unsafe fn execute(program: &str, ptr_args: &Vec<*const i8>, new_group: bool) -> Result<Process, String> {
    use crate::omicron::utils::errno_to_string;
    use crate::omicron::utils::Cstr;

    let result = libc::fork();

    if result != 0 {
        return if result > 0 {
            Ok(Process::new(result))
        } else {
            // result < 0
            Err(errno_to_string().unwrap_or("fork failed".to_string()))
        }
    }

    if new_group {
        libc::setsid();
        // what if setsid failed?
    }

    // result = 0
    let _error = libc::execvp(Cstr::magic(program), ptr_args.as_ptr());
    panic!("execvp failed: {}", errno_to_string().unwrap_or("execv failed".to_string())) // child panic
}
