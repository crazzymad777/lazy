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
// fn mount_vfs<S: AsRef<OsStr>>(typefs: S, device: S, dir: S, options: S) {
//     use std::process::Command;
//     use std::path::Path;
//     let _ = std::fs::create_dir(Path::new(&dir));
//     if let Err(e) = Command::new("mount").arg("-t").arg(typefs).arg(device).arg(dir).arg("-o").arg(options).spawn() {
//         eprintln!("Lazy mount failed: {}", e);
//     }
// }
//
// pub fn init_mount() {
//     // virtual file-systems
//     println!("Mount virtual file-systems");
//     mount_vfs("proc", "proc", "/proc", "nosuid,noexec,nodev");
//     mount_vfs("sysfs", "sys", "/sys", "nosuid,noexec,nodev");
//     mount_vfs("tmpfs", "run", "/run", "mode=0755,nosuid,nodev");
//     mount_vfs("devtmpfs", "dev", "/dev", "mode=0755,nosuid");
//     mount_vfs("devpts", "devpts", "/dev/pts", "mode=0620,gid=5,nosuid,noexec");
//     mount_vfs("tmpfs", "shm", "/dev/shm", "mode=1777,nosuid,nodev");
// }

pub fn mount_fstab() {
    use crate::omicron::command::CommandBuilder;
    // remount all
    println!("Mount all");
    if let Err(e) = CommandBuilder::new().program("mount\0").set_args(["-o\0","remount\0","-a\0"].to_vec()).spawn() {
        eprintln!("Lazy: mount failed: {}", e);
    }
}

pub fn enable_swap() {
    if let Err(e) = CommandBuilder::new().program("swapon\0").arg("-a\0")).spawn() {
        eprintln!("Lazy: enable swap failed: {}", e);
    }
}
