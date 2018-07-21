#![feature(test)]

extern crate test;

use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Read};

fn reverse_lines<R: Read>(r: BufReader<R>) -> String {
    let mut queue: VecDeque<String> = VecDeque::new();

    for line in r.lines() {
        queue.push_back(line.unwrap());
    }

    let mut out = String::new();
    for _ in 0..queue.len() {
        out.push_str(&queue.pop_back().unwrap());
        out.push_str("\n");
    }
    return out;
}

fn main() -> io::Result<()> {
    let f = File::open("./data/large-input.txt")?;
    let f = BufReader::new(f);
    print!("{}", reverse_lines(f));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use test::Bencher;

    #[test]
    fn test_reverse_lines() {
        let c = Cursor::new("a\nb\nc\n");
        let f = BufReader::new(c);
        let result = reverse_lines(f);
        assert_eq!(result, "c\nb\na\n");
    }

    #[bench]
    fn bench_reverse_lines(b: &mut Bencher) {
        b.iter(|| {
            let f = File::open("./data/large-input.txt").unwrap();
            let r = BufReader::new(f);
            reverse_lines(r);
        });
    }
}
