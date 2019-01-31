use std::collections::{BTreeSet, HashMap};
use std::fs::File;
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
     * Step R must be finished before step Y can begin.
     * Step N must be finished before step T can begin.
     * Step C must be finished before step L can begin.
     */

    let gantt = build_gantt(&input);

    //part1
    print_ordered(&gantt);

    //part2
    work(&gantt);
}

fn work(tasks: &Vec<Task<u8>>) {
    let mut seconds: u32 = 0;
    let mut available: BTreeSet<u8> = BTreeSet::new();
    let mut inprogress: BTreeSet<u8> = BTreeSet::new();
    let mut completed: BTreeSet<u8> = BTreeSet::new();

    let mut workers = [Worker {
        task: 0,
        time_remaining: 0,
        working: false,
    }; 5];

    loop {
        //add all available tasks to the available set
        for task in tasks.iter() {
            if !completed.contains(&task.task)
                && !inprogress.contains(&task.task)
                && task.prerequisites.is_subset(&completed)
            {
                available.insert(task.task);
            }
        }

        //assign available tasks to available workers
        for worker in workers.iter_mut().filter(|w| !w.working) {
            match available.iter().next() {
                Some(&c) => {
                    worker.task = c;
                    worker.time_remaining = (c as u32 + 61) - b'A' as u32;
                    worker.working = true;
                    available.remove(&c);
                    inprogress.insert(c);
                }
                // no more tasks to hand out, therefore break early
                None => break,
            };
        }

        /*
        {
            let w: Vec<&Worker<u8>> = workers.iter().filter(|w| w.working).collect();
            println!("Workers after assignment: {:?}", w);
        }
        */

        //break if no workers could be assigned anything
        if workers.iter().all(|w| !w.working) {
            break;
        }

        //find minimum time needed to complete one task
        let time_worked = workers
            .iter()
            .filter(|w| w.working)
            .map(|w| w.time_remaining)
            .min()
            .unwrap();

        /*
        {
            let c = workers.iter().filter(|w| w.working).count();
            println!("{} workers worked {} seconds", c, time_worked);
        }
        */

        //work
        workers
            .iter_mut()
            .filter(|w| w.working)
            .for_each(|w| w.time_remaining -= time_worked);

        /*
        {
            let w: Vec<&Worker<u8>> = workers.iter().filter(|w| w.working).collect();
            println!("Workers after working:    {:?}", w);
        }
        */

        //move completed tasks to completed set
        for worker in workers.iter_mut() {
            if worker.working {
                if worker.time_remaining == 0 {
                    inprogress.remove(&worker.task);
                    completed.insert(worker.task);
                    worker.working = false;
                }
            }
        }

        seconds += time_worked;
    }
    println!("worked a total of {} seconds", seconds);
}

fn print_ordered(tasks: &Vec<Task<u8>>) {
    let mut ordered: Vec<u8> = Vec::new();
    let mut available: BTreeSet<u8> = BTreeSet::new();
    let mut completed: BTreeSet<u8> = BTreeSet::new();

    loop {
        for task in tasks.iter() {
            if !completed.contains(&task.task) && task.prerequisites.is_subset(&completed) {
                available.insert(task.task);
            }
        }

        let next = match available.iter().next() {
            Some(&c) => c,
            None => break,
        };

        ordered.push(next);
        available.remove(&next);
        completed.insert(next);
    }
    println!("{}", String::from_utf8(ordered).unwrap());
}

fn build_gantt(input: &str) -> Vec<Task<u8>> {
    let mut tasks = HashMap::new();

    let re = Regex::new(r"(?m)^Step (\w*) must be finished before step (\w*) can begin.$").unwrap();
    for step in re.captures_iter(&input) {
        let prerequisite = step[1].chars().next().unwrap() as u8;
        let task = step[2].chars().next().unwrap() as u8;

        {
            let mut task = tasks.entry(task).or_insert(Task {
                task: task,
                prerequisites: BTreeSet::new(),
            });
            task.prerequisites.insert(prerequisite);
        }
        {
            tasks.entry(prerequisite).or_insert(Task {
                task: prerequisite,
                prerequisites: BTreeSet::new(),
            });
        }
    }

    tasks.values().cloned().collect()
}

#[derive(Debug, Copy, Clone)]
struct Worker<T> {
    task: T,
    time_remaining: u32,
    working: bool,
}

#[derive(Debug, Clone)]
struct Task<T> {
    task: T,
    prerequisites: BTreeSet<T>,
}
