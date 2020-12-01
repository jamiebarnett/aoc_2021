use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    
    let mut file = match File::open("input.txt") {
        Ok(f) => f,
        Err(err) => panic!("{}", err)
    };
    
    let mut input = Vec::new();
    let mut lines = io::BufReader::new(file).lines();
    for line in lines {
            if let Ok(line) = line {
                let i : isize = line.parse().expect("could not parse input");
                println!("{}", i);
                input.push(i);
            }
    }

    for i in input.clone() {
        for j in &input {
            if i + j == 2020 {
                println!("{} + {} = 2020", i, j);
                println!("{} * {} = {}", i, j, i * j)
            }
        }
    }

}
