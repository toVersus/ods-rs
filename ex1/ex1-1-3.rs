#![feature(test)]

extern crate test;

use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Read};

static DATAPATH: &'static str = "./data/ex1-1-3-large-input-with-empty-line.txt";

fn get_prev_nline_from_empty_line<R: Read>(r: BufReader<R>, nline: usize) -> String {
    let mut queue: VecDeque<String> = VecDeque::new();
    let mut out = String::new();

    for (i, line) in r.lines().enumerate() {
        let line = line.unwrap();
        if i <= nline {
            queue.push_front(line);
            continue;
        }

        // keep length of queue in 'nlines' size.
        queue.pop_back().unwrap();

        if !line.is_empty() {
            queue.push_front(line);
            continue;
        }
        // line is puhsed here to avoid an error throwed by borrow checker.
        queue.push_front(line);

        out.push_str(format!("l.{}: ", (i + 1) - (queue.len() - 1)).as_str());
        out.push_str(&queue.pop_back().unwrap());
        out.push_str("\n");
    }

    return out;
}

fn main() -> io::Result<()> {
    let f = File::open(DATAPATH)?;
    let f = BufReader::new(f);
    print!("{}", get_prev_nline_from_empty_line(f, 42));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use test::Bencher;

    #[test]
    fn test_get_prev_nline_from_empty_line() {
        let c = Cursor::new("a\nb\nc\nd\n\ne\nf\ng\nh\n");
        let f = BufReader::new(c);
        let result = get_prev_nline_from_empty_line(f, 2);
        assert_eq!(result, "l.3: c\n");
    }

    #[bench]
    fn bench_get_prev_nline_from_empty_line(b: &mut Bencher) {
        b.iter(|| {
            let f = File::open(DATAPATH).unwrap();
            let r = BufReader::new(f);
            get_prev_nline_from_empty_line(r, 50);
        });
    }
}
