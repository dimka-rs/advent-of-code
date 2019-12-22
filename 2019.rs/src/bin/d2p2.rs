use std::io::prelude::*;
use std::io::{self};
use std::io::BufReader;
use std::fs::File;
//use std::vec;

fn main() -> io::Result<()> {

    for val1 in 0..99
    {
        for val2 in 0..99
        {
            if run_prog(val1, val2) == 19690720
            {
                println!("Result: {}", 100 * val1 + val2);
                break;
            }
        }
    }
    Ok(())
}

fn run_prog(val1: i32, val2: i32) -> i32
{
    let mut prog: Vec<i32> = Vec::new();
    get_vec("./d2p1.in", &mut prog);

    prog[1] = val1;
    prog[2] = val2;

    let mut pc = 0;
    while pc < prog.len()
    {
        match prog[pc]
        {
            1 | 2 => pc += oper(&mut prog, pc),
            _ => break,
        }
    }
    prog[0]
}

fn oper(prog: &mut Vec<i32>, pc: usize) -> usize
{
    let idx1: i32 = prog[pc+1 as usize];
    let idx2: i32 = prog[pc+2 as usize];
    let idx3: i32 = prog[pc+3 as usize];

    let arg1: i32 = prog[idx1 as usize];
    let arg2: i32 = prog[idx2 as usize];

    if prog[pc as usize] == 1
    {
        prog[idx3 as usize] = arg1 + arg2;
    } else {
        prog[idx3 as usize] = arg1 * arg2;
    }
    4
}

fn get_vec(file: &str, prog: &mut Vec<i32>)
{
    let f = File::open(file).unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    for cmd in line.trim().split(',') {
        //print!("{}, ", cmd);
        prog.push(cmd.parse::<i32>().unwrap());
    }
}