#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Read};

static DATAPATH: &'static str = "./data/ex1-4-sequence-of-numbers.txt";

fn push_bottom(stack: &mut Vec<i32>, x: i32) {
    if stack.is_empty() {
        stack.push(x);
        return;
    }
    let top = stack.pop().unwrap();
    push_bottom(stack, x);
    stack.push(top);
}

fn reverse(stack: &mut Vec<i32>) -> &mut Vec<i32> {
    if stack.is_empty() {
        return stack;
    }
    let top = stack.pop().unwrap();
    reverse(stack);
    push_bottom(stack, top);
    return stack;
}

fn read_seq_numbers<R: Read>(r: BufReader<R>) -> Vec<i32> {
    let mut stack = Vec::new();
    for line in r.lines() {
        let line = line.unwrap();
        stack.push(line.parse::<i32>().unwrap());
    }
    return stack;
}

fn main() -> io::Result<()> {
    let f = File::open(DATAPATH)?;
    let f = BufReader::new(f);
    let mut s = read_seq_numbers(f);

    reverse(&mut s);
    println!("{:?}", s);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use test::Bencher;

    #[test]
    fn it_reverses_sequence_of_numbers() {
        let c = Cursor::new("3\n2\n1\n");
        let f = BufReader::new(c);
        let mut result = read_seq_numbers(f);
        reverse(&mut result);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[bench]
    fn bench_reverse_sequence_of_numbers(b: &mut Bencher) {
        let f = File::open(DATAPATH).unwrap();
        let f = BufReader::new(f);
        let mut s = read_seq_numbers(f);
        b.iter(|| {
            reverse(&mut s);
        })
    }
}
