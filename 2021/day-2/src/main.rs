use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello!");

    let fordward_str = "forward";
    let down_str = "down";
    let up_str = "up";

    let mut fordward = 0;
    let mut depth = 0;
    let mut aim = 0;
    if let Ok(lines) = read_file("./input.txt") {
        for line in lines {
            if let Ok(move_str) = line {
                let mut split = move_str.split(" ");
                let vec = split.collect::<Vec<&str>>();
                let move_str = vec[0];
                let move_int = vec[1].trim().parse::<u32>().unwrap();
                if move_str.eq(fordward_str) {
                    fordward += move_int;
                    depth += aim * move_int
                } else if move_str.eq(down_str) {
                    aim += move_int;
                } else if move_str.eq(up_str) {
                    aim -= move_int;   
                }                
            } 
        }
        println!("forward: {} x depth: {} = {}", fordward, depth, fordward * depth);         
    } 
}

fn read_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}