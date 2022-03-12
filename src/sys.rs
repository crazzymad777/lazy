pub fn reboot(response: String) {
    unsafe {
        use syscalls::syscall;
	let cmd = match response.as_str() {
            "poweroff" => 0x4321fedcusize,
            "restart" => 0x01234567usize,
            "halt" => 0xcdef0123usize,
	    &_ => todo!()
	};
        if let Err(e) = syscall(syscalls::SYS_reboot, &syscalls::SyscallArgs::from(&[0xfee1dead, 537993216, cmd])) {
            println!("{}", e);
        }
    }
}

