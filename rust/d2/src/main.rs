use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
// https://adventofcode.com/2024/day/2

fn main() -> Result<(), io::Error> {
    let f = File::open("input")?;
    let f = BufReader::new(f);

    let mut safe_count = 0;

    for line in f.lines() {
        let line = line?;
        if validate(&line) {
            safe_count += 1;
        }
    }

    assert_eq!(safe_count, 402); // only true for my input
    println!("Day 2, part1 {}", safe_count);

    Ok(())
}

fn validate(line: &str) -> bool {
    let f = |v: Vec<i32>| -> bool {
        return v.iter().all(|x| *x >= 0) || v.iter().all(|x| *x <= 0);
    };

    let g = |v: Vec<i32>| -> bool {
        return v.iter().all(|x| x.abs() > 0 && x.abs() < 4);
    };

    let xs: Vec<&str> = line.split_whitespace().collect();
    let xs: Vec<i32> = xs.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut ys = xs.to_vec();
    ys.insert(0, 0);
    let xs = ys.iter().zip(xs.iter());
    let mut xs: Vec<i32> = xs.map(|(a, b)| b - a).collect();
    xs.remove(0);
    return f(xs.clone()) && g(xs);
}
