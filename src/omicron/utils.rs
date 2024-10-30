// Use just CString???
// Probably best way is to construct zero terminated &str
// And validate that

use libc::c_char;

pub trait Cstr {
    fn check(self) -> Result<Self, &'static str> where Self: Sized {
        unimplemented!();
    }

    fn magic(self) -> *const c_char where Self: Sized {
        unimplemented!();
    }

    // return pointer to zero terminated string or panic
    // Really magic!
    fn new_magic(self) -> *const c_char where Self: Sized {
        self.check().ok().unwrap().magic()
    }
}

impl Cstr for &str {
    fn check(self) -> Result<Self, &'static str> {
        if self.chars().next_back().unwrap() == '\0' {
            Ok(self)
        } else {
            Err("Last Character of String Is Not Zero")
        }
    }

    fn magic(self) -> *const c_char where Self: Sized {
        return self.as_ptr() as *const c_char;
    }
}

pub unsafe fn errno_to_string() -> Option<String> {
    c_error_to_string(get_c_errno())
}

pub unsafe fn get_c_errno() -> libc::c_int {
    *libc::__errno_location()
}

pub unsafe fn c_error_to_string(current_error: libc::c_int) -> Option<String> {
    let buffer: [u8; 256] = [0; 256];
    let pointer = buffer.as_ptr() as *mut c_char;

    if libc::strerror_r(current_error, pointer, 256) == 0 {
        let s = std::str::from_utf8(buffer.as_slice()).unwrap();
        Some(String::from(s))
    } else {
        None
    }
}


// How to build array of strings?
// const char *argv[]
// pointer1, pointer2, pointer3, ...
