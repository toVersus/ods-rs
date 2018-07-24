#![feature(test)]

extern crate test;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Read};

static DATAPATH: &'static str = "./data/ex1-1-6-large-input-with-random-length-of-lines.txt";

fn sort_by_row_count<R: Read>(r: BufReader<R>) -> String {
    let mut books: BTreeMap<String, usize> = BTreeMap::new();
    let mut out = String::new();

    for line in r.lines() {
        let line = line.unwrap();
        if let Some(x) = books.get_mut(&line) {
            *x += 1;
            continue;
        }
        books.insert(line, 1);
    }

    // sort by keys in dictionary order.
    let mut count_by_val: Vec<_> = books.iter().collect();
    // sort by values in descending order.
    count_by_val.sort_by(|a, b| b.1.cmp(a.1));

    for (key, val) in count_by_val.into_iter() {
        out.push_str(format!("{}: {}\n", key, val).as_str());
    }

    return out;
}

fn main() -> io::Result<()> {
    let f = File::open(DATAPATH)?;
    let f = BufReader::new(f);
    print!("{}", sort_by_row_count(f));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use test::Bencher;

    #[test]
    fn test_sort_by_row_count() {
        let c = Cursor::new("a\nb\nc\na\nb\nc\nd\ne\n");
        let f = BufReader::new(c);
        let result = sort_by_row_count(f);
        assert_eq!(result, "a: 2\nb: 2\nc: 2\nd: 1\ne: 1\n");
    }

    #[bench]
    fn bench_sort_by_row_count(b: &mut Bencher) {
        b.iter(|| {
            let f = File::open(DATAPATH).unwrap();
            let r = BufReader::new(f);
            sort_by_row_count(r);
        });
    }
}
