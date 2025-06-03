use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

// https://adventofcode.com/2024/day/1
// correct day 1 part 1 answer is 1660292 for my input

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let f = BufReader::new(f);
    let mut v = Vec::new();
    let mut w = Vec::new();
    for line in f.lines() {
        let line = line?;
        let mut xs = line.split_whitespace();
        let left: i32 = xs.next().expect("").trim().parse().expect("");
        let right: i32 = xs.next().expect("").trim().parse().expect("");
        v.push(left);
        w.push(right);
    }

    v.sort();
    w.sort();

    let total: i32 = v.iter().zip(w).map(|(a, b)| (a - b).abs()).sum();

    assert_eq!(total, 1660292); // only true for my input
    println!("Day 1, Part 1: {total}");
    Ok(())
}
