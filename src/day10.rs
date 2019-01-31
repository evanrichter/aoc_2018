//use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io;
use std::io::prelude::*;

use regex::Regex;

pub fn solve(filename: String) {
    let mut input = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    /* Sample-data:
     *
     * position=<-41933,  10711> velocity=< 4, -1>
     * position=< 10721, -31361> velocity=<-1,  3>
     * position=<-52401, -20843> velocity=< 5,  2>
     * position=< 10666, -52404> velocity=<-1,  5>
     * position=< 31716, -20842> velocity=<-3,  2>
     * position=<-41914, -31366> velocity=< 4,  3>
     * position=<-31393, -20845> velocity=< 3,  2>
     * position=< 10706,  21233> velocity=<-1, -2>
     * position=< 21181, -31359> velocity=<-2,  3>
     * position=<-41881,  42273> velocity=< 4, -4>
     *
     */

    let re = Regex::new(r"(?m)^position=<\s*(-?\d*),\s*(-?\d*)> velocity=<\s*(-?\d*),\s*(-?\d*)>$")
        .unwrap();

    let mut comets = Vec::new();
    for cap in re.captures_iter(&input) {
        comets.push(Comet {
            position: V {
                x: cap[1].parse::<i64>().unwrap(),
                y: cap[2].parse::<i64>().unwrap(),
            },
            velocity: V {
                x: cap[3].parse::<i64>().unwrap(),
                y: cap[4].parse::<i64>().unwrap(),
            },
        });
    }
    // for c in comets.iter() { println!("{:?}", c); }

    println!("{:?}", comets.iter().next().unwrap());
    loop {
        comets.iter_mut().for_each(|c| c.step());
        if comets.iter().next().unwrap().position.y < 200 {
            break;
        }
    }

    //let mut s = String::new();
    for _ in 0..10 {
        //io::stdin().read_line(&mut s);
        next_frame(&mut comets);
    }
}

fn next_frame(comets: &mut Vec<Comet>) {
    let canvas = PixelSpace {
        offset: V { x: 0, y: 0 },
        size: V { x: 60, y: 15 },
    };
    let mut frame = [[b' '; 61]; 16];

    //advance each comet
    comets.iter_mut().for_each(|c| c.step());
    println!("comet1: {:?}", comets.iter().next().unwrap());

    // find smallest bounding box of all comets
    let min_x = comets.iter().map(|c| c.position.x).min().unwrap();
    let max_x = comets.iter().map(|c| c.position.x).max().unwrap();
    let min_y = comets.iter().map(|c| c.position.y).min().unwrap();
    let max_y = comets.iter().map(|c| c.position.y).max().unwrap();

    let bounded = PixelSpace {
        offset: V { x: min_x, y: min_y },
        size: V {
            x: max_x - min_x,
            y: max_y - min_y,
        },
    };

    comets.iter().for_each(|c| {
        let pixel = c.scale_position(&bounded, &canvas);
        frame[pixel.y as usize][pixel.x as usize] = b'#';
    });

    for line in frame.iter() {
        println!(
            "{}",
            String::from_utf8(line.iter().map(|&x| x).collect()).unwrap()
        );
    }
    println!("--------------");
}

#[derive(Debug)]
struct Comet {
    position: V,
    velocity: V,
}

#[derive(Debug)]
struct V {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct PixelSpace {
    offset: V,
    size: V,
}

impl Comet {
    fn step(self: &mut Self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    fn scale_position(self: &Self, unscaled: &PixelSpace, target: &PixelSpace) -> V {
        V {
            x: {
                let abs = self.position.x - unscaled.offset.x;
                let scaled = abs as f64 * target.size.x as f64 / unscaled.size.x as f64;
                scaled as i64 + target.offset.x
            },
            y: {
                let abs = self.position.y - unscaled.offset.y;
                let scaled = abs as f64 * target.size.y as f64 / unscaled.size.y as f64;
                scaled as i64 + target.offset.y
            },
        }
    }
}
