#[macro_use] extern crate log;
extern crate regex;
extern crate libintermix;
extern crate log4rs;
extern crate vterm_sys;
extern crate time;
extern crate ego_tree;

mod client;
mod server;
mod support;

static mut is_logging_setup: bool = false;

fn setup_logging() {
    // protect itself from running multiple times
    unsafe {
        if is_logging_setup { return }
        is_logging_setup = true;
    }

    log4rs::init_file(&std::env::current_dir().unwrap().join("tests/log4rs.toml"),
                      log4rs::toml::Creator::default())
        .unwrap();
}

// Returns true if the given function eventually returns true within several seconds
fn is_ultimately_true<F>(mut f: F) -> bool where F: FnMut() -> bool {
    let start_time = time::now();
    let timeout = time::Duration::seconds(15);

    loop {
        if f() { return true }

        if time::now() - start_time > timeout { break }

        // half a second
        let sleep_duration = ::std::time::Duration::from_millis(500);
        ::std::thread::sleep(sleep_duration);
    }

    false
}

/// Try the given function until is returns Ok, or there is a timeout.
///
/// Returns Ok on success, and the last returned value of the function inside Err on timeout.
fn try_until_ok<F, T>(mut f: F) -> Result<(), T> where F: FnMut() -> Result<(), T> {
    let start_time = time::now();
    let timeout = time::Duration::seconds(5);

    loop {
        match f() {
            Ok(()) => { return Ok(()) },
            Err(t) => {
                if time::now() - start_time > timeout {
                    return Err(t)
                }

                // half a second
                let sleep_duration = ::std::time::Duration::from_millis(500);
                ::std::thread::sleep(sleep_duration);
            }
        }
    }
}
