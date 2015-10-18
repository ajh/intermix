extern crate pty;
extern crate tsm_sys;

mod program;

use program::*;
use std::io;
use std::io::Write;
use std::io::Read;
use std::os::unix::io::AsRawFd;

fn main() {
    let program = Program::new("vim".to_string()).unwrap();

    loop {
            // blocking! Need to put the file handle in raw mode.
            let mut input_buf  = [0; 10];
            io::stdin().read(&mut input_buf);
            println!("read from stdin {:?}", input_buf);

            program.write(&input_buf);
            println!("write to program");

            let mut output_buf = [0; 10];
            program.read(&mut output_buf);
            println!("read from program {:?}", output_buf);

            //let s = program.read_to_string();
            //println!("read from program {}", s);

            io::stdout().write(&output_buf);
            println!("wrote to stdout");
    }
}
