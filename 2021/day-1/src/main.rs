use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

fn main() {
    println!("hey!");
    let mut sum_buf: u32 = 0;
    let mut buf: VecDeque<u32> = VecDeque::new();

    let mut last_number = 0;
    let mut measurements = -1;
    if let Ok(lines) = read("./input.txt") {
        for line in lines {
            if let Ok(deep_str) = line {
                let deep = deep_str.trim().parse::<u32>().unwrap();
                sum_buf += deep;
                buf.push_back(deep);
                println!("buf len{}", buf.len());
                if buf.len() == 3 {
                    println!("sum_buf {}", sum_buf);
                    if sum_buf > last_number {
                        measurements += 1;
                    }
                    last_number = sum_buf;
                    sum_buf -= buf.pop_front().unwrap();
                    println!("sum_buf rest {}", sum_buf);
                }
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