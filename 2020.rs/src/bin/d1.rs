use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let v: Vec<i32>;

    if args.len() < 2
    {
        println!("Usage: {} input_file", &args[0]);
        process::exit(1);
    }

    v = vec_from_file(&args[1]);

    println!("Answer 1: {}", solve1(&v));

    println!("Answer 2: {}", solve2(&v));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn vec_from_file(file: &str) -> Vec<i32>
{
    let mut v: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(num) = line {
                v.push(num.parse::<i32>().unwrap());
            }
        }
    }

    return v
}

fn solve1(v: &Vec<i32>) -> i32
{
    let mut sum :i32;

    for a in 0..v.len()
    {
        for b in a..v.len()
        {
            if a == b
            {
                continue;
            }

            sum = v[a] + v[b];

            if sum == 2020
            {
                return v[a] * v[b]
            }
        }
    }

    return 0
}

fn solve2(v: &Vec<i32>) -> i32
{
    let mut sum :i32;

    for a in 0..v.len()
    {
        for b in a..v.len()
        {
            for c in b..v.len()
            {
                if a == b || b == c || a == c
                {
                    continue;
                }

                sum = v[a] + v[b] + v[c];

                if sum == 2020
                {
                    return v[a] * v[b] * v[c]
                }
            }
        }
    }

    return 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve1(&vec_from_file("d1.in.test")), 514579);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve2(&vec_from_file("d1.in.test")), 241861950);
    }
}