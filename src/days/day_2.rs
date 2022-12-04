use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

/*
        A [1]: Rock         X [0]: lose
        B [2]: Paper        Y [3]: draw
        C [3]: Scissors     Z [6]: win
*/

//Part 1
pub fn rock_paper_scissors_1() -> io::Result<()> {
    let file = File::open("rps_strategy.txt")?;
    let reader = BufReader::new(file);

    let mut score = 0;

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        match line.as_str() {
            "A X" => {score += 4},
            "A Y" => {score += 8},
            "A Z" => {score += 3},
            "B X" => {score += 1},
            "B Y" => {score += 5},
            "B Z" => {score += 9},
            "C X" => {score += 7},
            "C Y" => {score += 2},
            "C Z" => {score += 6},
            _ => unreachable!(),
        };
        
    }
    Ok(())
}

//Part 2
pub fn rock_paper_scissors_2() -> io::Result<()> {
    let file = File::open("rps_strategy.txt")?;
    let reader = BufReader::new(file);

    let mut score = 0;

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        match line.as_str() {
            "A X" => {score += 3},
            "A Y" => {score += 4},
            "A Z" => {score += 8},
            "B X" => {score += 1},
            "B Y" => {score += 5},
            "B Z" => {score += 9},
            "C X" => {score += 2},
            "C Y" => {score += 6},
            "C Z" => {score += 7},
            _ => unreachable!(),
        };

    }
    Ok(())
}