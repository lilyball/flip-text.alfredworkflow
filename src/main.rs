#![feature(exit_status)]

extern crate alfred;

use std::env;
use std::io;
use std::io::prelude::*;

fn main() {
    let arg = env::args().skip(1).next().expect("expected argument");
    match process_argument(arg) {
        Err(err) => {
            let _ = writeln!(&mut io::stderr(), "I/O error: {}", err);
            env::set_exit_status(1);
            return
        }
        _ => {}
    }
}

fn process_argument(arg: String) -> io::Result<()> {
    // process argument here
    return Ok(())
}
