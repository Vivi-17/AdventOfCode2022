/*
vJrwpWtwJgWr hcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
 */

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn rucksack_reorganization_1() -> io::Result<()> {
    let file = File::open("rucksack.txt")?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        lines.push(line);
    }

    println!("Result: {}", get_points_1(&mut lines));

    Ok(())
}

fn get_points_1(vec: &Vec<String>) -> usize {
    let letters = "abcdefghijklmnopqrstuvwxyz";
    let mut result = 0;

    for line in vec {
        let lines = line.split_at(line.len() / 2);

        let mut char = ' ';

        for char_0 in lines.0.chars() {
            for char_1 in lines.1.chars() {
                if char_0.eq(&char_1) {
                    char = char_0;
                }
            }
        }

        let mut priority = 1;
        if char.is_uppercase() {
            priority = 27;
        }

        let index = match letters.find(&char.to_lowercase().to_string()) {
            Some(i) => i + priority,
            None => panic!("Unknown index.")
        };
        result += index;
    }

    result
}

pub fn rucksack_reorganization_2() -> io::Result<()> {
    let file = File::open("rucksack.txt")?;
    let reader = BufReader::new(file);

    let mut groups: Vec<Vec<String>> = Vec::from(Vec::new());
    let mut group = Vec::new();

    let mut index = 0;
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        if index == 2 {
            group.push(line);
            groups.push(group.clone());
            group.clear();
            index = 0;
        } else {
            group.push(line);
            index += 1;
        }

    }

    let mut result = 0;

    for group in &groups {
        result += get_points_2( group);
    }

    println!("Result: {}", result);
    Ok(())
}

fn get_points_2(vec: &Vec<String>) -> usize {
    let letters = "abcdefghijklmnopqrstuvwxyz";
    let mut result = 0;

    let mut char = ' ';

    for char_0 in vec[0].chars() {
        for char_1 in vec[1].chars() {
            for char_2 in vec[2].chars() {
                if char_0.eq(&char_1) && char_1.eq(&char_2) && char_2.eq(&char_0) {
                    char = char_0;
                }
            }
        }
    }

    let mut priority = 1;
    if char.is_uppercase() {
        priority = 27;
    }

    let index = match letters.find(&char.to_lowercase().to_string()) {
        Some(i) => i + priority,
        None => panic!("Unknown index.")
    };
    result = index;

    result
}