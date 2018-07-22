#![feature(test)]

extern crate test;

use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Read};

static DATAPATH: &'static str = "./data/ex1-1-1-large-input.txt";

fn reverse_nline<R: Read>(r: BufReader<R>, nline: usize) -> String {
    let mut queue: VecDeque<String> = VecDeque::new();
    let mut out = String::new();

    for (i, line) in r.lines().enumerate() {
        queue.push_back(line.unwrap());
        if (i + 1) % nline != 0 {
            continue;
        }
        for _ in 0..queue.len() {
            out.push_str(&queue.pop_back().unwrap());
            out.push_str("\n");
        }
    }

    if queue.len() <= 0 {
        return out;
    }

    for _ in 0..queue.len() {
        out.push_str(&queue.pop_back().unwrap());
        out.push_str("\n");
    }
    return out;
}

fn main() -> io::Result<()> {
    let f = File::open(DATAPATH)?;
    let f = BufReader::new(f);
    print!("{}", reverse_nline(f, 50));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use test::Bencher;

    #[test]
    fn test_reverse_nline() {
        let c = Cursor::new("a\nb\nc\nd\ne\n");
        let f = BufReader::new(c);
        let result = reverse_nline(f, 2);
        assert_eq!(result, "b\na\nd\nc\ne\n");
    }

    #[bench]
    fn bench_reverse_nline(b: &mut Bencher) {
        b.iter(|| {
            let f = File::open(DATAPATH).unwrap();
            let r = BufReader::new(f);
            reverse_nline(r, 50);
        });
    }
}
