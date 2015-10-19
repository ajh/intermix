     ___         _                          _
    |_ _| _ __  | |_  ___  _ __  _ __ ___  (_)__  __
     | | | '_ \ | __|/ _ \| '__|| '_ ` _ \ | |\ \/ /
     | | | | | || |_|  __/| |   | | | | | || | >  <
    |___||_| |_| \__|\___||_|   |_| |_| |_||_|/_/\_\

# Intermix

Intermix aspires to be a terminal emulator / multiplexer / cli program.

## How to use

1. Install a nightly rust distro
2. clone this repo
3. run `cargo run`

## Status

### terminal emulation

Using libtsm.

* Need to upgrade to lastest version and take unmerged patches.
* Need to make the io reads non-blocking. May have to use libc read like:
  https://github.com/jsgf/eventfd-rust/blob/master/src/lib.rs#L62
* Program needs to split command name and args for execvp.

### running multiple processes at once

Not started

### client server

Not started

### modal ui

Not started

### shell

Not started

### keyboard interactions

Not started

### screen transforms

Not started

### detecting open files

Not started

### use threads in a better way

std has the select! macro which can help use threads better for
channels. But for IO objects, not so much.

## Copyright and License

Copyright 2015 Andy Hartford.

The program is available as open source under the terms of the GPLv3
(See COPYING).
