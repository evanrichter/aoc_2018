use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve(filename: String) {
    let input = File::open(filename).unwrap();
    let reader = BufReader::new(input);

    let mut nums: Vec<i64> = vec![];
    let mut acc = 0;
    let mut seen = HashSet::new();
    seen.insert(0);
    let mut len = seen.len();

    // Read through the file of frequencies looking for repeat
    for line in reader.lines() {
        let val: i64 = line.unwrap().parse().unwrap();
        nums.push(val);
        acc += val;
        seen.insert(acc);
        if seen.len() == len {
            println!("found in file");
            found(acc);
        } else {
            len += 1;
        }
    }

    // Loop through frequencies looking for repeat
    for val in nums.into_iter().cycle() {
        acc += val;
        seen.insert(acc);
        if seen.len() == len {
            println!("found in loop");
            found(acc);
        } else {
            len += 1;
        }
    }
}

fn found(freq: i64) {
    println!("{}", freq);
    std::process::exit(0);
}
