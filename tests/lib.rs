extern crate regex;
extern crate libintermix;
extern crate log4rs;
extern crate vterm_sys;
extern crate time;

mod client;
mod server;

fn setup_logging() {
    log4rs::init_file(&std::env::current_dir().unwrap().join("tests/log4rs.toml"),
                      log4rs::toml::Creator::default())
        .unwrap();
}

/// The failure message on this is lame
fn try_until_true<F>(mut f: F) where F: FnMut() -> bool {
    let start_time = time::now();
    let timeout = time::Duration::seconds(3);

    loop {
        if f() { return }

        if time::now() - start_time > timeout { break }

        // half a second
        let sleep_duration = ::std::time::Duration::new(0, 500_000_000);
        ::std::thread::sleep(sleep_duration);
    }
    panic!("expected closure to return true but didn't after {:?}", timeout);
}

