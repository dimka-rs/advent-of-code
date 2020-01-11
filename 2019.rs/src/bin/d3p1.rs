use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

enum Direction
{
    Up,
    Down,
    Left,
    Right,
}

struct Wire
{
    dir: Direction,
    len: u32,
}

// tile of grid
struct Tile
{
    val: i32,
    x: i32,
    y: i32,
}

fn main() -> io::Result<()>
{
    let infile: &str = "d3p1.in";

    // read file to vectors of strings
    let wire1 = get_wire(&infile, 1);
    let wire2 = get_wire(&infile, 2);

    // parse to vector of wires
    let wire1 = parse_wire(wire1);
    let wire2 = parse_wire(wire2);

    // place wires on grid
    let mut grid: HashMap<String, Tile> = HashMap::new();

    place_wire(&mut grid, &wire1, 1);
    place_wire(&mut grid, &wire2, 2);

    // find intersections

    let mut answer = Tile
    {
        val: 10_000_000, // big enough?
        x: 0,
        y: 0,
    };

    let mut count : u32 = 0;
    let mut x_min : i32 = 0;
    let mut x_max : i32 = 0;
    let mut y_min : i32 = 0;
    let mut y_max : i32 = 0;

    for (_k, v) in &grid
    {
        count += 1;
        if v.x < 0 && v.x < x_min {x_min = v.x;}
        if v.x > 0 && v.x > x_max {x_max = v.x;}
        if v.y < 0 && v.y < y_min {y_min = v.y;}
        if v.y > 0 && v.y > y_max {y_max = v.y;}

        if v.val == 3
        {
            if v.x.abs() + v.y.abs() < answer.val
            {
                answer.val = v.x.abs() + v.y.abs();
                answer.x = v.x;
                answer.y = v.y;
            }
        }
    }

    println!("Answer: x = {}, y = {}, distance = {}.",
            answer.x, answer.y, answer.val);
    println!("Total points: {}. X range: {} to {}. Y range: {} to {}",
            count, x_min, x_max, y_min, y_max);
    Ok(())
}

fn place_wire(grid: &mut HashMap<String, Tile>, wire: &std::vec::Vec<Wire>, weight: i32)
{
    let mut x = 0;
    let mut y = 0;

    for leg in wire
    {
        for _ in 1..=leg.len
        {
            match &leg.dir
            {
                Direction::Up => y = y + 1,
                Direction::Down => y = y - 1,
                Direction::Left => x = x - 1,
                Direction::Right => x = x + 1,
            }
            place_dot(grid, x, y, weight);
        }
    }
}

fn place_dot(grid: &mut HashMap<String, Tile>, x: i32, y: i32, weight: i32)
{
    let key = x.to_string() + "_" + &y.to_string();
    if grid.contains_key(&key)
    {
        let tmp = grid.get_mut(&key).unwrap();
        if (tmp.val & weight) == 0
        {
            tmp.val = tmp.val | weight;
        }

    } else {
        grid.insert(key, Tile {val: weight, x: x, y: y});
    }
    // suggestion
    //grid.entry(key).and_modify(|e| { *e += weight }).or_insert(weight);
}


fn parse_wire(wire: Vec<std::string::String>) -> std::vec::Vec<Wire>
{
    let mut w: Vec<Wire> = Vec::new();

    for word in wire
    {
        let  dir: Direction = match &word[0..1]
        {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            &_ => panic!("Unknown direction: {}", &word[0..1]),
        };

        let len: u32 = word[1..].parse::<u32>().unwrap();
        w.push(Wire {dir, len});
    }
    return w;
}

fn get_wire(file: &str, line_num: u32) -> Vec<std::string::String>
{

    let f = File::open(file).unwrap();
    let mut f = BufReader::new(f);
    let mut line = String::new();
    for l in 0..line_num
    {
        line.clear();
        f.read_line(&mut line).unwrap();
        if l == line_num-1
        {
            print!("Line {}:", l);
            break;
        }
    }


    let mut wire: Vec<std::string::String> = Vec::new();
    let mut cnt = 0;
    for cmd in line.trim().split(',') {
        cnt += 1;
        wire.push(cmd.to_string());
    }
    println!(" {} items", cnt);
    wire
}