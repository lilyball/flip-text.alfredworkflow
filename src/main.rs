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
    let items = flip::process_text(arg).into_iter().map(|op| {
        let title = format!("{}", op);
        let mut builder = alfred::ItemBuilder::new(title.clone()).arg(title);
        match op {
            flip::Operation::Flip(_) => builder.set_subtitle("Flipped"),
            flip::Operation::Unflip(_) => builder.set_subtitle("Unflipped")
        }
        builder.set_valid(true);
        builder.into_item()
    }).collect::<Vec<_>>();
    alfred::json::write_items(io::stdout(), &items)
}

fn default_output() -> io::Result<()> {
    let mut items = vec![alfred::ItemBuilder::new("Flip \u{2026}").valid(false).arg("")
                                             .subtitle("Flip typed text").into_item()];
    for &s in ["(╯°□°）╯︵ ┻━┻", "┬─┬ノ( º _ ºノ)", "┻━┻ ︵ヽ(`Д´)ﾉ︵ ┻━┻ "].iter() {
        items.push(alfred::ItemBuilder::new(s).valid(true).arg(s)
                                       .uid(format!("flip {}", s)).into_item());
    }
    alfred::json::write_items(io::stdout(), &items)
}

fn strip(mut s: &str) -> &str {
    let prefixes: &[&str] = &["(╯°□°）╯︵", "(╯ಠ_ಠ）╯︵", "(ﾉ `Д´)ﾉ ﾐ"];
    let suffixes: &[&str] = &["ノ( º _ ºノ)"];
    s = s.trim();
    for prefix in prefixes {
        s = s.trim_start_matches(prefix).trim_start();
    }
    for suffix in suffixes {
        s = s.trim_end_matches(suffix).trim_end();
    }
    s
}
