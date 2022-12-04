use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn top_three() -> io::Result<()> {
    //open file and use reader
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut results = Vec::new();
    let mut x = 0;

    //go through this file line by line
    for line_result in reader.lines() {
        //line_result becomes a String
        let line = match line_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        if !line.is_empty() {
            x += line.parse::<i32>().unwrap();
            continue
        }

        results.push(x);
        x = 0;
    }

    results.sort_by(|x, y| y.cmp(&x));

    let end_result = results[0] + results[1] + results[2];

    println!("END RESULT: {}", end_result);

    Ok(())
}

pub fn max_calories() -> io::Result<()> {
    //open file and use reader
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut results = Vec::new();
    let mut x = 0;

    //go through this file line by line
    for line_result in reader.lines() {
        //line_result becomes a String
        let line = match line_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        if !line.is_empty() {
            x += line.parse::<i32>().unwrap();
            continue
        }

        results.push(x);
        x = 0;
    }

    results.sort_by(|x, y| y.cmp(&x));

    println!("END RESULT: {}", results[0]);

    Ok(())
}