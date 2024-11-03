// Here OS calls / OS-specific calls

pub fn reboot(response: String) {
    unsafe {
        use libc::reboot;
        let cmd = match response.as_str() {
            "poweroff" => libc::RB_POWER_OFF,
            "reboot" => libc::RB_AUTOBOOT,
            "halt" => libc::RB_HALT_SYSTEM,
            &_ => todo!()
        };
        reboot(cmd);
    }
}

pub fn provide_hostname() {
    let data = std::fs::read("/etc/hostname").ok();
    if data.is_some() {
        let _ = std::fs::write("/proc/sys/kernel/hostname", data.unwrap());
    }
}

// use std::ffi::OsStr;
// fn mount_vfs<S: AsRef<OsStr> + std::ops::Add<&str, Output = &str>>(typefs: S, device: S, dir: S, options: S) {
//     use crate::omicron::command::CommandBuilder;
//     use std::path::Path;
//     let _ = std::fs::create_dir(Path::new(&dir));
//     if let Err(e) = CommandBuilder::new().program("mount\0").arg("-t\0").arg(typefs+"\0").arg(device+"\0").arg(dir+"\0").arg("-o\0").arg(options+"\0").spawn() {
//         eprintln!("Lazy mount failed: {}", e);
//     }
// }

pub fn init_mount() {
    // virtual file-systems
    // println!("Mount virtual file-systems");
    // mount_vfs("proc", "proc", "/proc", "nosuid,noexec,nodev");
    // mount_vfs("sysfs", "sys", "/sys", "nosuid,noexec,nodev");
    // mount_vfs("tmpfs", "run", "/run", "mode=0755,nosuid,nodev");
    // mount_vfs("devtmpfs", "dev", "/dev", "mode=0755,nosuid");
    // mount_vfs("devpts", "devpts", "/dev/pts", "mode=0620,gid=5,nosuid,noexec");
    // mount_vfs("tmpfs", "shm", "/dev/shm", "mode=1777,nosuid,nodev");
}

pub fn mount_fstab() {
    use crate::omicron::ShellCommand;
    use crate::omicron::command::CommandBuilder;
    // remount all
    println!("Mount all");
    if let Err(e) = CommandBuilder::new().program("mount\0").set_args(["-o\0","remount\0","-a\0"].to_vec()).spawn() {
        eprintln!("Lazy: mount failed: {}", e);
    }
}

pub fn enable_swap() {
    use crate::omicron::ShellCommand;
    use crate::omicron::command::CommandBuilder;
    if let Err(e) = CommandBuilder::new().program("swapon\0").arg("-a\0").spawn() {
        eprintln!("Lazy: enable swap failed: {}", e);
    }
}

pub fn mute_kernel() {
    let result = std::fs::File::create("/proc/sys/kernel/printk");
    if let Ok(mut x) = result {
        use std::io::Write;
        let _ = x.write_all(b"3 3 3 3");
    }
}

pub fn new_process_session() {
    unsafe {
        libc::setsid();
    }
}

pub fn change_dir_to_root() {
    use crate::omicron::utils::Cstr;
    unsafe {
        libc::chdir(Cstr::new_magic("/\0"));
    }
}
