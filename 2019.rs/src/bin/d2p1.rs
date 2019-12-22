use std::io::prelude::*;
use std::io::{self};
use std::io::BufReader;
use std::fs::File;
//use std::vec;

fn main() -> io::Result<()> {
    let mut prog: Vec<i32> = Vec::new();
    get_vec("./d2p1.in", &mut prog);

    // fix input as required
    prog[1] = 12;
    prog[2] = 2;

    // run program
    let mut pc = 0;
    while pc < prog.len()
    {
        match prog[pc]
        {
            1 => pc += op_add(&mut prog, pc),
            2 => pc += op_mul(&mut prog, pc),
            _ => break,
        }
    }
    println!("Result: {}", prog[0]);
    Ok(())
}

fn op_add(prog: &mut Vec<i32>, pc: usize) -> usize
{
    let idx1: i32 = prog[pc+1 as usize];
    let idx2: i32 = prog[pc+2 as usize];
    let idx3: i32 = prog[pc+3 as usize];

    let arg1: i32 = prog[idx1 as usize];
    let arg2: i32 = prog[idx2 as usize];

    prog[idx3 as usize] = arg1 + arg2;
    4
}

fn op_mul(prog: &mut Vec<i32>, pc: usize) -> usize
{
    let idx1: i32 = prog[pc+1 as usize];
    let idx2: i32 = prog[pc+2 as usize];
    let idx3: i32 = prog[pc+3 as usize];

    let arg1: i32 = prog[idx1 as usize];
    let arg2: i32 = prog[idx2 as usize];

    prog[idx3 as usize] = arg1 * arg2;
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