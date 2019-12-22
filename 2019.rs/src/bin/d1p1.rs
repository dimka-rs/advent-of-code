use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("./d1p1.in")?;
    let f = BufReader::new(f);

    let mut sum: i32 = 0;

    for line in f.lines() {
        sum += calc(line.unwrap().parse().unwrap());
    }

    println!("Result is {}", sum);
    Ok(())
}

fn calc(mass: i32) -> i32 {
    mass / 3 - 2
}