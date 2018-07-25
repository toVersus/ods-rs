#![feature(test)]

extern crate rand;
extern crate test;

use rand::{thread_rng, Rng};

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Read};

static DATAPATH: &'static str = "./data/ex1-1-6-large-input-with-random-length-of-lines.txt";

fn random_walk<R: Read>(r: BufReader<R>) -> String {
    let mut stmts = Vec::new();

    for line in r.lines() {
        stmts.push(line.unwrap());
    }
    let stmts: &mut [String] = &mut stmts;
    thread_rng().shuffle(stmts);

    return stmts.join("\n");
}

fn main() -> io::Result<()> {
    let f = File::open(DATAPATH)?;
    let f = BufReader::new(f);
    print!("{}", random_walk(f));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use test::Bencher;

    #[test]
    // always failed...
    fn test_random_walk() {
        let c = Cursor::new("a\nb\nc\na\nb\nc\nd\ne");
        let f = BufReader::new(c);
        let result = random_walk(f);
        assert_eq!(result, "a\nc\nb\nd\nb\na\nc\ne\n");
    }

    #[bench]
    fn bench_random_walk(b: &mut Bencher) {
        b.iter(|| {
            let f = File::open(DATAPATH).unwrap();
            let r = BufReader::new(f);
            random_walk(r);
        });
    }
}
