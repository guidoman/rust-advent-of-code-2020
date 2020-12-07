use std::fs::File;
use std::io::{prelude::*, BufReader};

fn part1_check(line_fields: &Vec<&str>) -> bool {
    let range_split: Vec<&str> = line_fields[0].split("-").collect();
    let min_occurrencies = range_split[0].parse::<i16>().unwrap();
    let max_occurrencies = range_split[1].parse::<i16>().unwrap();
    let policy_char = &line_fields[1][..1];
    let chars_to_check: Vec<char> = line_fields[2].chars().collect();
    let mut occurrencies_cnt = 0;
    for ch in chars_to_check.iter() {
        if ch.to_string() == policy_char {
            occurrencies_cnt += 1;
        }
    }
    occurrencies_cnt >= min_occurrencies && occurrencies_cnt <= max_occurrencies
}

fn part2_check(line_fields: &Vec<&str>) -> bool {
    let range_split: Vec<&str> = line_fields[0].split("-").collect();
    let pos1 = range_split[0].parse::<usize>().unwrap();
    let pos2 = range_split[1].parse::<usize>().unwrap();
    let policy_char = &line_fields[1][..1];
    let chars_to_check: Vec<char> = line_fields[2].chars().collect();
    let mut match_cnt = 0;
    if chars_to_check[pos1 - 1].to_string() == policy_char {
        match_cnt += 1;
    }
    if chars_to_check[pos2 - 1].to_string() == policy_char {
        match_cnt += 1;
    }
    match_cnt == 1
}

fn main() {
    let file = File::open("input/day2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut part1_valid_cnt = 0;
    let mut part2_valid_cnt = 0;
    for line in reader.lines() {
        let  s = line.unwrap();
        let s = s.trim();
        if !s.is_empty() {
            let line_fields : Vec<&str> = s.split(" ").collect();
            if line_fields.len() != 3 {
                println!("WARNING malformed line: {}", s);
                continue;
            }
            if part1_check(&line_fields) {
                part1_valid_cnt += 1;
            }
            if part2_check(&line_fields) {
                part2_valid_cnt += 1;
            }
        } else {
            println!("WARNING empty line!")
        }      
    }

    println!("PART 1 - Final result = {}", part1_valid_cnt);
    println!("PART 2 - Final result = {}", part2_valid_cnt);


}
