use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

pub fn solve(filename: String) {
    let mut input = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    //part2
    println!("safe_count: {}", safe_count(800, &input));

    //part1
    let mut gridsmall = neighbor_count(800, &input);
    let gridlarge = neighbor_count(820, &input);

    gridsmall.retain(|&key, val| val == gridlarge.get(&key).unwrap_or(&0));
    let mut ranked: Vec<&usize> = gridsmall.values().collect();
    ranked.sort();
    println!("{:?}", gridsmall);
    println!("{:?}", ranked);
}

fn safe_count(gridsize: usize, input: &str) -> usize {
    let mut grid = HashMap::new();
    let lower = -(gridsize as i64) / 2;
    let upper = gridsize as i64 / 2;

    let re = Regex::new(r"(?m)^(\d*), (\d*)$").unwrap();
    for cap in re.captures_iter(&input) {
        let coord = Coord {
            x: cap[1].parse::<i64>().unwrap(),
            y: cap[2].parse::<i64>().unwrap(),
        };

        for x in lower..upper {
            for y in lower..upper {
                let distance = ((coord.x - x).abs() + (coord.y - y).abs()) as usize;
                let mut entry = grid.entry(Coord { x: x, y: y }).or_insert(0);
                *entry += distance;
            }
        }
    }

    grid.retain(|_, &mut val| val < 10000);
    grid.len()
}

fn neighbor_count(gridsize: usize, input: &str) -> HashMap<Coord, usize> {
    let mut grid = HashMap::new();
    let lower = -(gridsize as i64) / 2;
    let upper = gridsize as i64 / 2;

    let re = Regex::new(r"(?m)^(\d*), (\d*)$").unwrap();
    for cap in re.captures_iter(&input) {
        let coord = Coord {
            x: cap[1].parse::<i64>().unwrap(),
            y: cap[2].parse::<i64>().unwrap(),
        };

        for x in lower..upper {
            for y in lower..upper {
                let distance = ((coord.x - x).abs() + (coord.y - y).abs()) as usize;
                let mut entry = grid
                    .entry(Coord { x: x, y: y })
                    .or_insert((None, gridsize * 2));
                if entry.1 > distance {
                    entry.0 = Some(coord);
                    entry.1 = distance;
                } else if entry.1 == distance {
                    entry.0 = None;
                    entry.1 = distance;
                }
            }
        }
    }

    let mut neighbor_map = HashMap::new();
    for x in lower..upper {
        for y in lower..upper {
            let mut entry = grid.entry(Coord { x: x, y: y }).or_insert((None, 0));
            let closest_neighbor = match entry.0 {
                Some(c) => c,
                None => continue,
            };
            let count = neighbor_map.entry(closest_neighbor).or_insert(0);
            *count += 1;
        }
    }

    neighbor_map
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}
