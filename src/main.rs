#![feature(exit_status)]

extern crate alfred;

#[macro_use]
extern crate lazy_static;

use std::env;
use std::io;
use std::io::prelude::*;

mod flip;

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
    let mut xmlw = try!(alfred::XMLWriter::new(io::stdout()));
    for op in flip::process_text(strip(&arg)) {
        let title = format!("{}", op);
        let mut builder = alfred::ItemBuilder::new(&title[..]).arg(&title[..]);
        match op {
            flip::Operation::Flip(_) => builder.set_subtitle("Flipped"),
            flip::Operation::Unflip(_) => builder.set_subtitle("Unflipped")
        }
        builder.set_valid(true);
        try!(xmlw.write_item(&builder.into_item()));
    }
    try!(xmlw.close()).flush()
}

fn strip(mut s: &str) -> &str {
    let prefixes: &[&str] = &["(╯°□°）╯︵", "(╯ಠ_ಠ）╯︵", "(ﾉ `Д´)ﾉ ﾐ"];
    let suffixes: &[&str] = &["ノ( º _ ºノ)"];
    for prefix in prefixes {
        s = s.trim_left_matches(prefix);
    }
    for suffix in suffixes {
        s = s.trim_right_matches(suffix);
    }
    s
}
