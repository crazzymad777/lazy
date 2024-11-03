use crate::omicron::Process;

// Wrapper for execvp

// return pid?
// 0 - for child
// <0 - error
// >0 - parent, Process::new()
pub unsafe fn execute(program: &str, ptr_args: &Vec<*const i8>, new_group: bool, pipe_out: bool) -> Result<(Process, Option<libc::c_int>), String> {
    use crate::omicron::utils::errno_to_string;
    use crate::omicron::utils::Cstr;

    let mut fds: [libc::c_int; 2] = [-1, -1];
    let mut fd: Option<libc::c_int> = None;
    if pipe_out {
        libc::pipe(fds.as_mut_ptr());
        fd = Some(fds[0]);
    }

    let result = libc::fork();

    if result != 0 {
        return if result > 0 {
            Ok((Process::new(result), fd))
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
        let new_fd_err = libc::open(Cstr::new_magic("/dev/null\0"), libc::O_WRONLY);
        let new_fd_in = libc::open(Cstr::new_magic("/dev/null\0"), libc::O_RDONLY);
        let mut new_fd_out = new_fd_err;
        if pipe_out {
            if fds[1] != -1 {
                new_fd_out = fds[1];
            }
        }

        libc::close(libc::STDOUT_FILENO);
        libc::close(libc::STDERR_FILENO);
        libc::close(libc::STDIN_FILENO);

        libc::fcntl(new_fd_out, libc::F_DUPFD, libc::STDOUT_FILENO);
        libc::fcntl(new_fd_err, libc::F_DUPFD, libc::STDERR_FILENO);
        libc::fcntl(new_fd_in, libc::F_DUPFD, libc::STDIN_FILENO);
    }
    let _error = libc::execvp(Cstr::magic(program), ptr_args.as_ptr());
    panic!("execvp failed: {}", errno_to_string().unwrap_or("execv failed".to_string())) // child panic
}
