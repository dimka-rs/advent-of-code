use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("./d1p1.in")?;
    let f = BufReader::new(f);

    let mut sum: i32 = 0;

    for line in f.lines() {
        let mut add_mass = line.unwrap().parse().unwrap();
        while let Some(new_mass) = calc(add_mass)  {
            sum += new_mass;
            add_mass = new_mass;
        }
    }

    println!("Result is {}", sum);
    Ok(())
}

fn calc(mass: i32) -> Option<i32> {
    let new_mass = mass / 3 - 2;
    if new_mass > 0
    {
        Some(new_mass)
    } else {
        None
    }
}