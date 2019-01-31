use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve(filename: String) {
    let input = File::open(filename).unwrap();

    //part1(input);
    part2(input);
}

fn part2(input: File) {
    let reader = BufReader::new(input);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    let mut common_chars = Vec::new();
    for (i, this_line) in lines.iter().enumerate() {
        let mut rest = lines.iter().skip(i);
        if rest.next().is_none() {
            panic!("Ran out of strings to compare!");
        }
        for other_line in rest {
            let mut diff_occurred = false;
            for (a, b) in this_line.chars().zip(other_line.chars()) {
                if a != b {
                    if diff_occurred {
                        common_chars.clear();
                        break;
                    } else {
                        diff_occurred = true;
                    }
                } else {
                    common_chars.push(a);
                }
            }
            if !common_chars.is_empty() {
                let s: String = common_chars.iter().collect();
                println!("Match! {}", s);
            }
        }
    }
}

fn part1(input: File) {
    let reader = BufReader::new(input);
    // Read through the file of frequencies looking for repeat
    /*
        ayitmcjvlhedbsyoqfzukjpxwt
        agirmcjvlheybsyogfzuknpxxt
        wgirmcjvlvedbsyoqfzujnpxwt
        agizmcjvlhedbsyoqfzuenlxwt
        aryrmcjvlheebsyoqfzuknpxwt
        agirmcjelhedbsyoqfzukosxwt
        azirmcjvlhedbsooqfzuknpxvt
        agirmcjvffedbsyoqfzudnpxwt
        agilmcjvlhedbsyrqfzuknpxrt
        agirmcjvlhndbsyoofzukcpxwt
    */

    let mut num2 = 0;
    let mut num3 = 0;

    let mut counter = HashMap::new();
    for line in reader.lines() {
        // count occurences of chars in line
        for byte in line.unwrap().into_bytes() {
            let v = counter.entry(byte).or_insert(0);
            *v += 1;
        }

        for &v in counter.values() {
            if v == 2 {
                num2 += 1;
                break;
            }
        }

        for &v in counter.values() {
            if v == 3 {
                num3 += 1;
                break;
            }
        }

        counter.drain();
    }

    println!("num2 * num3 = {}", num2 * num3);
}
