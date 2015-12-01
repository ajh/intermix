extern crate regex;
extern crate libintermix;
extern crate log4rs;
extern crate vterm_sys;

mod client;
mod server;

fn setup_logging() {
    log4rs::init_file(&std::env::current_dir().unwrap().join("tests/log4rs.toml"),
                      log4rs::toml::Creator::default())
        .unwrap();
}
