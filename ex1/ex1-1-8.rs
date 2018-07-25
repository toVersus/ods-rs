#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Read};

static DATAPATH: &'static str = "./data/ex1-1-6-large-input-with-random-length-of-lines.txt";

fn alter_odd_even<R: Read>(r: BufReader<R>) -> String {
    let mut even = String::new();
    let mut odd = String::new();

    for (i, line) in r.lines().enumerate() {
        let line = line.unwrap() + "\n";
        if i % 2 == 0 {
            even.push_str(line.as_str());
        } else {
            odd.push_str(line.as_str());
        }
    }

    return even.lines().chain(odd.lines()).collect::<String>();
}

fn main() -> io::Result<()> {
    let f = File::open(DATAPATH)?;
    let f = BufReader::new(f);
    print!("{}", alter_odd_even(f));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use test::Bencher;

    #[test]
    fn test_alter_odd_even() {
        let c = Cursor::new("a\nb\nc\na\nb\nc\nd\ne\n");
        let f = BufReader::new(c);
        let result = alter_odd_even(f);
        assert_eq!(result, "acbdbace");
    }

    #[bench]
    fn bench_alter_odd_even(b: &mut Bencher) {
        b.iter(|| {
            let f = File::open(DATAPATH).unwrap();
            let r = BufReader::new(f);
            alter_odd_even(r);
        });
    }
}
