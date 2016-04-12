#![feature(test)]

#[macro_use]
extern crate libintermix;
extern crate vterm_sys;
extern crate test;

use vterm_sys::*;
use std::io::prelude::*;
use test::Bencher;

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
        vterm.write(b"\x1b[Hhi there").unwrap();
        vterm.screen_flush_damage();
        while let Some(msg) = rx.try_recv().ok() {
            ::std::io::stderr().write_fmt(format_args!("{:?}\n", msg)).unwrap();
        }
    });
}
