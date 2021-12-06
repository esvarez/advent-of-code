use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut last_number = 0;
    let mut measurements = -1;
    if let Ok(lines) = read("./input.txt") {
        for line in lines {
            if let Ok(deep_str) = line {
                let deep = deep_str.parse::<i32>().unwrap();
                if  deep > last_number {
                    measurements += 1;
                }   
                last_number = deep;
            } 
        } 
    } 
    println!("measurements {}", measurements);
}

fn read<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}