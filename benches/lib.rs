#![feature(test)]

#[macro_use]
extern crate log;
extern crate libintermix;
extern crate log4rs;
extern crate vterm_sys;
extern crate test;

use vterm_sys::*;
use std::io::prelude::*;
use test::Bencher;

// This seems pretty fast! 17,000ns per write.
#[bench]
fn bench_get_screen_damage_event(b: &mut Bencher) {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 24,
        width: 80,
    });
    vterm.screen_receive_events(&ScreenCallbacksConfig::all());
    vterm.screen_set_damage_merge(DamageSize::Row);
    let rx = vterm.screen_event_rx.take().unwrap();

    b.iter(|| {
        println!("\n");
        vterm.write(b"\x1b[Hhi there").unwrap();
        vterm.screen_flush_damage();
        while let Some(msg) = rx.try_recv().ok() {
            println!("{:?}", msg);
        }
    });
}
