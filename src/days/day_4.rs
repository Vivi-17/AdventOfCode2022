/*
.234.....  2-4
.....678.  6-8

.23......  2-3
...45....  4-5

....567..  5-7
......789  7-9

.2345678.  2-8
..34567..  3-7

.....6...  6-6
...456...  4-6

.23456...  2-6
...45678.  4-8
 */

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn get_assignment() -> io::Result<()> {
    let file = File::open("assignment.txt")?;
    let reader = BufReader::new(file);

    let mut pairs = Vec::new();

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(file) => file,
            Err(e) => panic!("Line panic - e: {}", e),
        };

        pairs.push(line);
    }
    //let result = common_assignment_1(pairs);
    let result = common_assignment_2(pairs);
    println!("Result: {}", result);
    Ok(())
}

fn common_assignment_1(pairs: Vec<String>) -> i32 {
    let mut common_assignment = 0;

    for index in 0..pairs.len() {
        let pair = &pairs[index];
        let (num1, num2) = pair.split_once(",").unwrap();
        let part1: Vec<&str> = num1.split("-").collect();
        let part2: Vec<&str> = num2.split("-").collect();
        let a: i32 = part1[0].parse().unwrap();
        let b: i32 = part2[0].parse().unwrap();
        let c: i32 = part1[1].parse().unwrap();
        let d: i32 = part2[1].parse().unwrap();

        let (x, y) = (b - a, d - c);
        if x * y <= 0 {
            common_assignment += 1;
        }
    }

    common_assignment
}

fn common_assignment_2(pairs: Vec<String>) -> i32 {
    let mut common_assignment = 0;

    for index in 0..pairs.len() {
        let pair = &pairs[index];
        let (num1, num2) = pair.split_once(",").unwrap();
        let part1: Vec<&str> = num1.split("-").collect();
        let part2: Vec<&str> = num2.split("-").collect();
        let a: i32 = part1[0].parse().unwrap();
        let b: i32 = part2[0].parse().unwrap();
        let c: i32 = part1[1].parse().unwrap();
        let d: i32 = part2[1].parse().unwrap();

        if b <= c && a <= d {
            common_assignment += 1;
        }
    }

    common_assignment
}