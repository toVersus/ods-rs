#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::{self, BufReader, Read};

static DATAPATH: &'static str = "./data/ex1-2-dyck-words.txt";

const PLUS: char = '+';
const MINUS: char = '-';

fn dyck_word<R: Read>(mut r: BufReader<R>) -> bool {
    let mut buf = String::new();
    r.read_to_string(&mut buf).expect("read_to_string() failed");

    let mut stack = Vec::new();
    for (i, c) in buf.chars().enumerate() {
        if i % 2 != 0 {
            continue;
        }
        if c == PLUS {
            stack.push(1);
        } else if c == MINUS && stack.len() > 0 {
            stack.pop();
        } else {
            return false;
        }
    }
    true
}

fn main() -> io::Result<()> {
    let f = File::open(DATAPATH)?;
    let f = BufReader::new(f);
    println!("{}", dyck_word(f));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use test::Bencher;

    #[test]
    fn it_dyck_word() {
        let c = Cursor::new("+1-1+1-1+1-1+1");
        let f = BufReader::new(c);
        let result = dyck_word(f);
        assert_eq!(result, true);
    }

    #[bench]
    fn bench_dyck_word(b: &mut Bencher) {
        b.iter(|| {
            let f = File::open(DATAPATH).unwrap();
            let r = BufReader::new(f);
            dyck_word(r);
        });
    }
}
