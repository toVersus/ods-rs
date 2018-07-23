#![feature(test)]

extern crate test;

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Read};

static DATAPATH: &'static str = "./data/ex1-1-3-large-input-with-empty-line.txt";

fn pass_uniq_line<R: Read>(r: BufReader<R>) -> String {
    let mut books = HashSet::new();
    let mut out = String::new();

    for line in r.lines() {
        let line = line.unwrap();
        if books.contains(&line) {
            continue;
        }
        books.insert(line.to_owned());
        out.push_str(&line);
        out.push_str("\n");
    }

    return out;
}

fn main() -> io::Result<()> {
    let f = File::open(DATAPATH)?;
    let f = BufReader::new(f);
    print!("{}", pass_uniq_line(f));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use test::Bencher;

    #[test]
    fn test_pass_uniq_line() {
        let c = Cursor::new("a\nb\nc\na\nb\nc\nd\ne\n");
        let f = BufReader::new(c);
        let result = pass_uniq_line(f);
        assert_eq!(result, "a\nb\nc\nd\ne\n");
    }

    #[bench]
    fn bench_pass_uniq_line(b: &mut Bencher) {
        b.iter(|| {
            let f = File::open(DATAPATH).unwrap();
            let r = BufReader::new(f);
            pass_uniq_line(r);
        });
    }
}
