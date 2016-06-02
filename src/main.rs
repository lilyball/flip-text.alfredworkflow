extern crate alfred;

#[macro_use]
extern crate lazy_static;

use std::env;
use std::io;
use std::io::prelude::*;
use std::process::exit;

mod flip;

fn main() {
    let arg = env::args().skip(1).next();
    match process_argument(arg.as_ref().map_or("", |s| &s[..])) {
        Err(err) => {
            let _ = writeln!(&mut io::stderr(), "I/O error: {}", err);
            exit(1);
        }
        _ => {}
    }
}

fn process_argument(arg: &str) -> io::Result<()> {
    let arg = strip(arg);
    if arg.is_empty() {
        return default_output();
    }
    let mut xmlw = try!(alfred::XMLWriter::new(io::stdout()));
    for op in flip::process_text(arg) {
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

fn default_output() -> io::Result<()> {
    let mut xmlw = try!(alfred::XMLWriter::new(io::stdout()));
    try!(xmlw.write_item(&alfred::ItemBuilder::new("Flip \u{2026}").valid(false).arg("")
                                              .subtitle("Flip typed text").into_item()));
    for &s in ["(╯°□°）╯︵ ┻━┻", "┬─┬ノ( º _ ºノ)", "┻━┻ ︵ヽ(`Д´)ﾉ︵ ┻━┻ "].iter() {
        try!(xmlw.write_item(&alfred::ItemBuilder::new(s).valid(true).arg(s)
                                                  .uid(format!("flip {}", s)).into_item()));
    }
    try!(xmlw.close()).flush()
}

fn strip(mut s: &str) -> &str {
    let prefixes: &[&str] = &["(╯°□°）╯︵", "(╯ಠ_ಠ）╯︵", "(ﾉ `Д´)ﾉ ﾐ"];
    let suffixes: &[&str] = &["ノ( º _ ºノ)"];
    s = s.trim();
    for prefix in prefixes {
        s = s.trim_left_matches(prefix).trim_left();
    }
    for suffix in suffixes {
        s = s.trim_right_matches(suffix).trim_right();
    }
    s
}
