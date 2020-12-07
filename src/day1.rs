use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input/day1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut vec: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let  s = line.unwrap();
        let s = s.trim();
        let my_int = s.parse::<i32>().unwrap();
        vec.push(my_int);
    }
    for i in 0..vec.len() {
        for j in i..vec.len() {
            if vec[i] + vec[j] == 2020 {
                let res = vec[i] * vec[j];
                println!("{} X {} = {}", vec[i], vec[j], res);
            }
        }
    }

}
