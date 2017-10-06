//! Routines for flipping/unflipping text

use std::collections::HashMap;
use std::fmt;
use std::iter::IntoIterator;

pub const FLIP_PREFIX: &'static str = "(╯°□°）╯︵";
pub const UNFLIP_SUFFIX: &'static str = "ノ( º _ ºノ)";

/// The results of a flip or unflip operation.
pub enum Operation {
    Flip(String),
    Unflip(String)
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Operation::Flip(ref s) => write!(f, "{} {}", FLIP_PREFIX, s),
            &Operation::Unflip(ref s) => write!(f, "{} {}", s, UNFLIP_SUFFIX)
        }
    }
}

pub fn process_text(s: &str) -> Vec<Operation> {
    let (flipped, flipcount) = flip(s);
    if let Some((unflipped, unflipcount)) = unflip(s) {
        if unflipcount > flipcount {
            vec![Operation::Unflip(unflipped), Operation::Flip(flipped)]
        } else {
            vec![Operation::Flip(flipped), Operation::Unflip(unflipped)]
        }
    } else {
        vec![Operation::Flip(flipped)]
    }
}

fn flip(s: &str) -> (String, usize) {
    let mut flipped = Vec::new();
    let mut count = 0;
    for c in s.chars() {
        if let Some(&fc) = FLIP_MAP.get(&c) {
            flipped.push(fc);
            count += 1;
        } else {
            flipped.push(c);
        }
    }
    (flipped.into_iter().rev().collect(), count)
}

fn unflip(s: &str) -> Option<(String, usize)> {
    let mut unflipped = Vec::new();
    let mut count = 0;
    let mut flipcount = 0;
    for c in s.chars() {
        if let Some(&fc) = UNFLIP_MAP.get(&c) {
            unflipped.push(fc);
            count += 1
        } else {
            unflipped.push(c);
            if FLIP_MAP.get(&c).is_some() {
                flipcount += 1;
            }
        }
    }
    if count > flipcount {
        Some((unflipped.into_iter().rev().collect(), count))
    } else {
        None
    }
}

lazy_static! {
    static ref FLIP_MAP: HashMap<char, char> = {
        [
            ('a', '\u{250}'), ('b', 'q'), ('c', '\u{254}'), ('d', 'p'), ('e', '\u{1DD}'),
            ('f', '\u{25F}'), ('g', '\u{253}'), ('h', '\u{265}'), ('i', '\u{131}'),
            ('j', '\u{27E}'), ('k', '\u{29E}'), ('l', '\u{A781}'), ('m', '\u{26F}'), ('n', 'u'),
            ('o', 'o'), ('p', 'd'), ('q', 'b'), ('r', '\u{279}'), ('s', 's'), ('t', '\u{287}'),
            ('u', 'n'), ('v', '\u{28C}'), ('w', '\u{28D}'), ('x', 'x'), ('y', '\u{28E}'),
            ('z', 'z'),
            ('A', '\u{2200}'), ('B', '\u{10412}'), ('C', '\u{186}'), ('D', '\u{15E1}'),
            ('E', '\u{18E}'), ('F', '\u{2132}'), ('G', '\u{2141}'), ('H', 'H'), ('I', 'I'),
            ('J', '\u{17F}'), ('K', '\u{22CA}'), ('L', '\u{2E5}'), ('M', 'W'), ('N', 'N'),
            ('O', 'O'), ('P', '\u{500}'), ('Q', '\u{38C}'), ('R', '\u{1D1A}'), ('S', 'S'),
            ('T', '\u{22A5}'), ('U', '\u{2229}'), ('V', '\u{39B}'), ('W', 'M'), ('X', 'X'),
            ('Y', '\u{2144}'), ('Z', 'Z'),
            ('0', '0'), ('1', '\u{21C2}'), ('2', '\u{1105}'), ('3', '\u{190}'), ('4', '\u{3123}'),
            ('5', '\u{78E}'), ('6', '9'), ('7', '\u{3125}'), ('8', '8'), ('9', '6'),
            ('&', '\u{214B}'), ('_', '\u{203E}'), ('?', '\u{BF}'), ('!', '\u{A1}'),
            ('"', '\u{201E}'), ('\'', ','), ('.', '\u{2D9}'), (',', '\''), (';', '\u{61B}')
        ].into_iter().cloned().collect()
    };
    static ref UNFLIP_MAP: HashMap<char, char> = {
        let mut map = [('ƃ', 'g'), ('ʃ', 'l'), ('\u{5DF}', 'l')].into_iter().cloned().collect::<HashMap<char,char>>();
        map.extend(FLIP_MAP.iter().map(|(&k,&v)| (v,k)));
        map
    };
}
