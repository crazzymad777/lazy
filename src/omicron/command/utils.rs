use crate::omicron::Process;

// Wrapper for execvp

// return pid?
// 0 - for child
// <0 - error
// >0 - parent, Process::new()

pub unsafe fn fopen_write_to_null() -> *mut libc::FILE {
    use crate::omicron::utils::Cstr;
    libc::fopen(
        Cstr::new_magic("/dev/null\0"),
        Cstr::new_magic("w\0")
    )
}

pub unsafe fn fopen_read_from_null() -> *mut libc::FILE {
    use crate::omicron::utils::Cstr;
    libc::fopen(
        Cstr::new_magic("/dev/null\0"),
        Cstr::new_magic("r\0")
    )
}

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
    unsafe {
        // redirect stdio
        use libc::FILE;
        let file_out = fopen_write_to_null();
        let new_fd_out = libc::fileno(file_out);

        let file_in = fopen_write_to_null();
        let new_fd_in = libc::fileno(file_in);

        // Not Thread Safe:
        // libc::fclose(libc::STDOUT_FILENO as *mut FILE);
        // libc::fclose(libc::STDERR_FILENO as *mut FILE);
        // libc::fcntl(newFd, libc::F_DUPFD, libc::STDOUT_FILENO);
        // libc::fcntl(newFd, libc::F_DUPFD, libc::STDERR_FILENO);

        libc::dup2(new_fd_out, libc::STDOUT_FILENO);
        libc::dup2(new_fd_out, libc::STDERR_FILENO);
        libc::dup2(new_fd_in, libc::STDIN_FILENO);
    }
    let _error = libc::execvp(Cstr::magic(program), ptr_args.as_ptr());
    panic!("execvp failed: {}", errno_to_string().unwrap_or("execv failed".to_string())) // child panic
}
