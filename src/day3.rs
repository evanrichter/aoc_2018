use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::iter::Iterator;
use std::ops::Add;

use regex::Regex;

pub fn solve(filename: String) {
    let mut input = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    /*
     * sample data:
     *
     *  #1 @ 829,837: 11x22
     *  #2 @ 14,171: 10x16
     *  #3 @ 456,661: 13x19
     *
     */

    let mut fabric = HashMap::new();
    let mut claims = Vec::new();

    let re = Regex::new(r"(?m)^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

    for cap in re.captures_iter(&input) {
        let claim = Claim {
            id: cap[1].parse().unwrap(),
            position: Vector {
                x: cap[2].parse().unwrap(),
                y: cap[3].parse().unwrap(),
            },
            size: Vector {
                x: cap[4].parse().unwrap(),
                y: cap[5].parse().unwrap(),
            },
        };

        claims.push(claim);

        for square in claim {
            let counter = fabric.entry(square).or_insert(0);
            *counter += 1;
        }
    }

    // part1
    println!("Count: {}", fabric.values().filter(|&&x| x > 1).count());

    // part2
    for claim in claims {
        if claim.into_iter().all(|x| fabric[&x] == 1) {
            println!("Uncontested: {}", claim.id);
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Claim {
    id: u32,
    position: Vector,
    size: Vector,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Vector {
    x: u32,
    y: u32,
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, {}>", self.x, self.y)
    }
}

impl fmt::Display for Claim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {}, position: {}, size: {}",
            self.id, self.position, self.size
        )
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl IntoIterator for Claim {
    type Item = Vector;
    type IntoIter = ClaimIterator;

    fn into_iter(self) -> Self::IntoIter {
        ClaimIterator {
            start: Vector {
                x: self.position.x,
                y: self.position.y,
            },
            end: Vector {
                x: self.position.x + self.size.x - 1,
                y: self.position.y + self.size.y - 1,
            },
            cur: Vector {
                x: self.position.x,
                y: self.position.y,
            },
        }
    }
}

struct ClaimIterator {
    start: Vector,
    end: Vector,
    cur: Vector,
}

impl Iterator for ClaimIterator {
    type Item = Vector;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.y > self.end.y {
            self.cur.y = self.start.y;
            self.cur.x += 1;
        }
        if self.cur.x > self.end.x {
            return None;
        }
        let temp = self.cur;
        self.cur.y += 1;
        Some(temp)
    }
}
