//use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::prelude::*;

pub fn solve(filename: String) {
    let mut input = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    /* Sample-data:
     *
     * 2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2....
     *
     */

    let example = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
    let nums: Vec<usize> = input
        .split_whitespace()
        .filter_map(|s| match s.parse::<usize>() {
            Ok(n) => Some(n),
            Err(_) => None,
        })
        .collect();

    let (example, _) = build_tree(&example);
    let (tree, _) = build_tree(&nums);

    //part1
    println!("metadata sum: {}", tree.sum_metadata());

    //part2
    println!("value of root: {}", example.value());
    println!("value of root: {}", tree.value());
}

fn build_tree(mut cursor: &[usize]) -> (Node, &[usize]) {
    let num_children = cursor[0];
    let num_metadata = cursor[1];
    let mut this_node = Node {
        children: Vec::new(),
        metadata: Vec::new(),
    };

    cursor = &cursor[2..];
    for _ in 0..num_children {
        let (child, new_cursor) = build_tree(cursor);
        cursor = new_cursor;
        this_node.children.push(child);
    }

    for m in cursor[0..num_metadata].iter() {
        this_node.metadata.push(m.clone());
    }

    (this_node, &cursor[num_metadata..])
}

#[derive(Debug, Clone)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn sum_metadata(self: &Self) -> usize {
        self.metadata.iter().sum::<usize>()
            + self
                .children
                .iter()
                .fold(0, |acc, c| acc + c.sum_metadata())
    }

    fn value(self: &Self) -> usize {
        if self.children.len() == 0 {
            self.metadata.iter().sum::<usize>()
        } else {
            self.metadata
                .iter()
                .filter(|&&x| x > 0 && x <= self.children.len())
                .map(|&i| self.children[i - 1].value())
                .sum()
        }
    }
}
