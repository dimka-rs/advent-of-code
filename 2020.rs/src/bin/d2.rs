use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let v: Vec<String>;

    if args.len() < 2 {
        println!("Usage: {} input_file", &args[0]);
        process::exit(1);
    }

    v = vec_from_file(&args[1]);

    println!("Answer 1: {}", solve1(&v));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn vec_from_file(file: &str) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(l) = line {
                v.push(l);
            }
        }
    }

    return v;
}

fn solve1(v: &Vec<String>) -> i32 {
    let mut cnt: i32;
    cnt = 0;

    for a in 0..v.len() {
        if is_valid_pw(&v[a]) {
            cnt += 1;
        }
    }
    return cnt;
}

fn is_valid_pw(line: &String) -> bool {
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve1(&vec_from_file("d2.in.test")), 514579);
    }
}
