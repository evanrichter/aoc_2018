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

    /*
     * sample data:
     *
     * [1518-09-01 00:58] wakes up
     * [1518-04-28 00:44] wakes up
     * [1518-09-08 00:34] falls asleep
     * [1518-11-10 00:43] falls asleep
     * [1518-10-21 00:17] falls asleep
     * [1518-11-09 23:58] Guard #853 begins shift
     * [1518-07-21 00:55] falls asleep
     * [1518-04-23 00:06] falls asleep
     * [1518-08-12 00:01] Guard #2111 begins shift
     * [1518-11-02 00:03] Guard #151 begins shift
     *
     */

    let mut entries = Vec::new();

    let re = Regex::new(r"(?m)^\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.*)$").unwrap();
    let wake_re = Regex::new(r"^(?P<wake>wakes up)").unwrap();
    let sleep_re = Regex::new(r"^(?P<sleep>falls asleep)").unwrap();
    let guard_re = Regex::new(r"^(?P<guard>Guard \#(?P<id>\d+) )").unwrap();

    for cap in re.captures_iter(&input) {
        let time = Time {
            year: cap[1].parse().unwrap(),
            month: cap[2].parse().unwrap(),
            day: cap[3].parse().unwrap(),
            hour: cap[4].parse().unwrap(),
            minute: cap[5].parse().unwrap(),
        };

        let entrytext = &cap[6];

        let entrytype = if wake_re.is_match(entrytext) {
            EntryType::Wake
        } else if sleep_re.is_match(entrytext) {
            EntryType::Sleep
        } else if guard_re.is_match(entrytext) {
            EntryType::Guard(
                guard_re
                    .captures(entrytext)
                    .unwrap()
                    .name("id")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
            )
        } else {
            panic!("Entrytype could not be parsed.")
        };

        entries.push(Entry {
            time: time,
            entrytype: entrytype,
        });
    }

    entries[..].sort_by(|a, b| a.time.cmp(&b.time));

    // part1
    let mut guards = HashMap::new();
    let mut guard = 0;
    let mut sleeptime = 0;
    let mut most_sleep = 0;
    for entry in entries {
        match entry.entrytype {
            EntryType::Guard(id) => {
                //                println!("guard: {}", id);
                guard = id;
            }
            EntryType::Sleep => sleeptime = entry.time.minute,
            EntryType::Wake => {
                let waketime = entry.time.minute;
                let (ref mut time, ref mut hm) = guards.entry(guard).or_insert_with(|| {
                    let hm: HashMap<u8, u32> = HashMap::new();
                    (0 as u32, hm)
                });
                *time += waketime as u32 - sleeptime as u32;
                if *time > most_sleep {
                    most_sleep = *time;
                    println!("most time: {}", most_sleep);
                }
                for minute in sleeptime..waketime {
                    *hm.entry(minute).or_insert(0) += 1;
                }
            }
        }
    }

    let mut times: Vec<(u32, u32)> = guards.iter().map(|(&id, &(time, _))| (time, id)).collect();
    times[..].sort();
    println!("{:?}", times);

    // part1
    let sleepy_guard = times.pop().unwrap().1;
    let (best_min, _) =
        guards
            .get(&sleepy_guard)
            .unwrap()
            .1
            .iter()
            .fold(
                (0, 0),
                |(m, a), (&min, &val)| if val > a { (min, val) } else { (m, a) },
            );
    println!("Sleepy guard: {}", sleepy_guard);
    println!("Best minute: {}", best_min);
    println!("{}", sleepy_guard * best_min as u32);

    // part2
    let mut sleepy_minute = (0_32, 0_u8, 0_u32);
    for (&id, (_, minfreq)) in guards.iter() {
        let (best, val) = minfreq.iter().fold(
            (0, 0),
            |(m, a), (&min, &val)| if val > a { (min, val) } else { (m, a) },
        );
        if val > sleepy_minute.2 {
            sleepy_minute = (id, best, val);
        }
    }
    println!("Sleepy guard: {}", sleepy_minute.0);
    println!("Best minute: {}", sleepy_minute.1);
    println!("{}", sleepy_minute.0 * sleepy_minute.1 as u32);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Time {
    year: u32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

#[derive(Debug)]
enum EntryType {
    Guard(u32),
    Sleep,
    Wake,
}

#[derive(Debug)]
struct Entry {
    time: Time,
    entrytype: EntryType,
}
