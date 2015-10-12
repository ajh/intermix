#![feature(libc)]
extern crate libc;
use libc::c_void;
use libc::c_int;
use std::ptr;
//mod libtsm;

#[repr(C)]
struct TsmScreen;
#[repr(C)]
struct TsmLogT;

// This code is editable and runnable!
fn main() {
    #[link(name="tsm", kind="static")]
    extern {
        fn tsm_screen_new(out: *mut *mut TsmScreen, log: Option<TsmLogT>, log_data: *mut c_void) -> c_int;
    }

    unsafe {
      let mut screen = ptr::null_mut();
      tsm_screen_new(&mut screen, None, ptr::null_mut());
    };
}
