use libloading::{Library, Symbol};
use std::ffi::{c_char, c_void, CStr};
use std::str;

struct SmartSocket {
    lib: Library,
}

enum Status {
    On,
    Off,
}

impl SmartSocket {
    fn new() -> Self {
        let lib_: Library;
        unsafe {
            lib_ = Library::new("smart_socket.dll").unwrap();
        }
        Self { lib: lib_ }
    }

    fn set_status(&self, status: Status) {
        let status_: u32 = match status {
            Status::On => 1,
            Status::Off => 0,
        };
        unsafe {
            let f: Symbol<unsafe extern "C" fn(u32) -> c_void> =
                self.lib.get(b"ex_set_status\0").unwrap();
            f(status_);
        }
    }

    fn get_status(&self) -> String {
        let mut buffer = [0i8; 32];
        unsafe {
            let f: Symbol<unsafe extern "C" fn(*const c_char, usize) -> c_void> =
                self.lib.get(b"get_status\0").unwrap();
            f(buffer.as_mut_ptr(), buffer.len());
        }
        let c_str: &CStr = unsafe { CStr::from_ptr(buffer.as_ptr()) };
        let str_slice: &str = c_str.to_str().unwrap();
        let str_buf: String = str_slice.to_owned();
        str_buf
    }
}

fn main() {
    let smart_socket = SmartSocket::new();
    {
        println!("SmartSocket: {}", smart_socket.get_status());
        smart_socket.set_status(Status::On);
        println!("SmartSocket: {}", smart_socket.get_status());
        smart_socket.set_status(Status::Off);
        println!("SmartSocket: {}", smart_socket.get_status());
    }
}
