use std::fs::File;
use std::io::prelude::*;
use std::mem;

use indicatif::{ProgressBar, ProgressStyle};

pub fn solve(filename: String) {
    let mut input = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    /*
     * sample data:
     *
     * ppPjJfFLlNtTyYKjJTtkSMmYysAgGTexwWXE...
     *
     */

    let input = input.trim();

    // part1
    let dissolved = dissolve(&input);
    println!("{}", dissolved.len());

    // part2
    let best = best_polymer(&input);
    println!("{}", best);
}

fn best_polymer(polymer: &str) -> usize {
    let style = ("█▉▊▋▌▍▎▏  ", "yellow");
    let pb = ProgressBar::new(26);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(&format!(
                "{{prefix:.bold}}▕{{bar:.{}}}▏{{msg}}",
                style.1
            )).progress_chars(style.0),
    );

    let mut best_len = polymer.len();
    for c in b'a'..=b'z' {
        pb.inc(1);
        let lower = c as char;
        let upper = (c ^ 0x20) as char;
        let test = polymer.replace(lower, "").replace(upper, "");
        let test_len = dissolve(&test).len();
        if test_len < best_len {
            best_len = test_len;
        }
    }
    best_len
}

fn dissolve(polymer: &str) -> String {
    let mut polymer = polymer.as_bytes().to_vec();
    let mut next_polymer = vec![];
    loop {
        let mut matched = false;
        let mut i = 1;
        while i < polymer.len() {
            if matches(polymer[i - 1], polymer[i]) {
                matched = true;
                i += 2;
            } else {
                next_polymer.push(polymer[i - 1]);
                i += 1;
            }
        }
        if i == polymer.len() {
            next_polymer.push(polymer[i - 1]);
        }

        mem::swap(&mut polymer, &mut next_polymer);
        next_polymer.clear();

        if !matched {
            break;
        }
    }
    String::from_utf8(polymer).unwrap()
}

fn matches(a: u8, b: u8) -> bool {
    a == b ^ 0x20
}
