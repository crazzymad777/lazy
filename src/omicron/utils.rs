// Use just CString???
// Probably best way is to constuct zero terminated &str
// And validate that

pub trait Cstr {
    fn check(self) -> Result<Self, &'static str> where Self: Sized {
        unimplemented!();
    }

    fn magic(self) -> *const i8 where Self: Sized {
        unimplemented!();
    }
}

impl Cstr for &str {
    fn check(self) -> Result<Self, &'static str> {
        if self.chars().next_back().unwrap() == '\0' {
            Ok(self)
        } else {
            Err("Not Zero-Terminated String")
        }
    }

    fn magic(self) -> *const i8 where Self: Sized {
        return self.as_ptr() as *const i8;
    }
}

// How to build array of strings?
// const char *argv[]
// pointer1, pointer2, pointer3, ...
