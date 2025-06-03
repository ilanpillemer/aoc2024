use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
// https://adventofcode.com/2024/day/1

fn main() -> Result<(), io::Error> {
    let f = File::open("input")?;
    let f = BufReader::new(f);
    let mut v = Vec::new();
    let mut w = Vec::new();
    let mut counts = HashMap::<i32, i32>::new();
    for line in f.lines() {
        let line = line?;
        let mut xs = line.split_whitespace();
        let left: i32 = xs.next().unwrap().trim().parse().unwrap();
        let right: i32 = xs.next().unwrap().trim().parse().unwrap();
        v.push(left);
        w.push(right);

        // build the distribution for part 2
        counts.entry(left).or_insert(0);
        let count = counts.entry(right).or_insert(0);
        *count += 1
    }

    v.sort();
    w.sort();

    let total: i32 = v.iter().zip(w).map(|(a, b)| (a - b).abs()).sum();
    let score: i32 = v.iter().map(|x| x * counts.get(x).unwrap()).sum();

    assert_eq!(total, 1660292); // only true for my input
    println!("Day 1, Part 1: {total}");

    assert_eq!(score, 22776016); // only true for my input
    println!("Day 1, Part 2: {score}");

    Ok(())
}
