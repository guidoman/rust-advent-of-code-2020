use std::fs::File;
use std::io::{prelude::*, BufReader};

fn day3_impl(right: usize, down: usize) -> usize {
    let file = File::open("input/day3.txt").unwrap();
    let reader = BufReader::new(file);
    let mut line_cnt = 0;
    let mut trees_cnt = 0;
    for line in reader.lines() {
        let s = line.unwrap();
        let s = s.trim();
        if !s.is_empty() {
            if line_cnt > 0 && (down == 1 || (line_cnt % 2) == 0) {
                let line_n_chars = s.chars().count();
                let pos = (right * (line_cnt / down)) % line_n_chars;
                let ch = s.chars().nth(pos).unwrap();
                if ch == '#' {
                    trees_cnt += 1;
                }
            }
            line_cnt += 1;
        } else {
            println!("WARNING empty line!")
        }
    }
    trees_cnt
}

fn part_2() {
    let mut part2_res = 1;
    let res = day3_impl(1, 1);
    part2_res *= res;
    println!("Part 2-1 result: {}", res);
    let res = day3_impl(3, 1);
    part2_res *= res;
    println!("Part 2-2 result: {}", res);
    let res = day3_impl(5, 1);
    part2_res *= res;
    println!("Part 2-3 result: {}", res);
    let res = day3_impl(7, 1);
    part2_res *= res;
    println!("Part 2-4 result: {}", res);
    let res = day3_impl(1, 2);
    part2_res *= res;
    println!("Part 2-5 result: {}", res);

    println!("Part 2 result: {}", part2_res);
}

fn part_1() {
    let res = day3_impl(3, 1);
    println!("Part 1 result: {}", res);
}

fn main() {
    part_1();
    part_2();
}
