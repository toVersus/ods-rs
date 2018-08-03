#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::{self, BufReader, Read};

static DATAPATH: &'static str = "./data/ex1-3-is-closed-grouping-statement.txt";

const LBRACE: char = '{';
const RBRACE: char = '}';
const LPARENT: char = '(';
const RPARENT: char = ')';
const LBRACKET: char = '[';
const RBRACKET: char = ']';

fn matched_string<R: Read>(mut r: BufReader<R>) -> bool {
    let mut buf = String::new();
    r.read_to_string(&mut buf).expect("read_to_string() failed");

    let mut stack = Vec::<(char, char)>::new();
    for c in buf.chars() {
        match c {
            LBRACE => stack.push((LBRACE, RBRACE)),
            LBRACKET => stack.push((LBRACKET, RBRACKET)),
            LPARENT => stack.push((LPARENT, RPARENT)),
            RBRACE | RPARENT | RBRACKET => {
                let pair = stack.pop().unwrap();
                if c != pair.1 {
                    return false;
                }
            }
            _ => (),
        }
    }
    if stack.len() > 0 {
        return false;
    }
    return true;
}

fn main() -> io::Result<()> {
    let f = File::open(DATAPATH)?;
    let f = BufReader::new(f);
    println!("{}", matched_string(f));

    Ok(())
}
