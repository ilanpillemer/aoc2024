use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

// correct day 1 answer is 1660292

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let f = BufReader::new(f);
    let mut v = Vec::new();
    let mut w = Vec::new();
    for line in f.lines() {
        let line = line?;
        let mut xs = line.split_whitespace();
        let left = xs.next().expect("");
        let right = xs.next().expect("");
        let left: i32 = left.trim().parse().expect("");
        let right: i32 = right.trim().parse().expect("");
        v.push(left);
        w.push(right);
    }

    v.sort();
    w.sort();
    let z = v.iter().zip(w);

    let mut total = 0;
    for x in z {
        let (a, b) = x;
        total += (a - b).abs()
    }
    println!("{}", total);

    Ok(())
}
