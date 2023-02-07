#![no_std]

pub struct Kos {}

#[repr(C)]
struct CFile
{
    _ptr: *mut char,
    _cnt: i32,
    _base: *mut char,
    _flag: i32,
    _file: i32,
    _charbuf: i32,
    _bufsiz: i32,
    _tmpfname: *mut char,
}

impl Kos {
    pub fn test() -> () {
        unsafe {
            c_raw::println("Hello, Rust!\n");
        }
    }
}

pub mod c_raw {
    extern "C" {
        pub fn gdb_init();
        pub fn pvr_init_defaults();
        pub fn println(s: &str, ...);
    }
}