use chrono::{NaiveDate, NaiveDateTime};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate chrono;
extern crate regex;

struct Event {
    dt: NaiveDateTime,
    id: i32,
    sleep: bool
}

impl Event {
    #[allow(dead_code)]
    pub fn print(&self) {
        if self.id >= 0 {
            println!("{} - #{}", self.dt.format("[%Y-%m-%d %H:%M]").to_string(), self.id);
        } else {
            println!("{} - {}", self.dt.format("[%Y-%m-%d %H:%M]").to_string(), self.sleep.to_string());
        }
    }

    pub fn get_id(&self) -> Option<i32> {
        if self.id >= 0 {
            Some(self.id)
        } else {
            None
        }
    }

    pub fn sleep(&self) -> bool {
        return self.sleep;
    }

    pub fn mins(&self) -> u32 {
        use chrono::Timelike;
        return self.dt.minute();
    }
}

fn get_date_time(cap:regex::Captures) -> NaiveDateTime
{
    let yr:i32 = cap[1].parse::<i32>().unwrap();
    let mo:u32 = cap[2].parse::<u32>().unwrap();
    let dy:u32 = cap[3].parse::<u32>().unwrap();
    let hr:u32 = cap[4].parse::<u32>().unwrap();
    let mn:u32 = cap[5].parse::<u32>().unwrap();
    return NaiveDate::from_ymd(yr, mo, dy).and_hms(hr, mn, 0);
}

#[allow(unused_parens)]
fn solution(part1: bool) {
    let path = "C:\\Users\\lgascoigne\\IdeaProjects\\advent\\input.txt";
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut lines = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                lines.push(line);
            }
            Err(e) => println!("err: {}", e)
        }
    }

    let re_begin = Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] Guard #(\d*) begins shift").unwrap();
    let re_wake = Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] wakes up").unwrap();
    let re_sleep = Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] falls asleep").unwrap();

    let mut event_vec = Vec::new();
    for line in &lines {
        let begin_caps = re_begin.captures(line);
        if begin_caps.is_some() {
            let cap = begin_caps.unwrap();
            let id:i32 = cap[6].parse::<i32>().unwrap();
            let dt: NaiveDateTime = get_date_time(cap);
            event_vec.push(Event { dt: dt, id: id, sleep: false });
            continue;
        }
        let wake_caps = re_wake.captures(line);
        if wake_caps.is_some() {
            let cap = wake_caps.unwrap();
            let dt: NaiveDateTime = get_date_time(cap);
            event_vec.push(Event { dt: dt, id: -1, sleep: false });
            continue;
        }
        let sleep_caps = re_sleep.captures(line);
        if sleep_caps.is_some() {
            let cap = sleep_caps.unwrap();
            let dt: NaiveDateTime = get_date_time(cap);
            event_vec.push(Event { dt: dt, id: -1, sleep: true });
        }
    }

    event_vec.sort_by_key(|x| x.dt);

    let mut map = HashMap::new();
    let num_events = event_vec.len();
    let mut x = 0;
    while x < num_events {
        let ev = &event_vec[x];
        x += 1;
        let id = ev.get_id();
        if id.is_some() {
            let record = map.entry(id.unwrap()).or_insert([0; 61]);
            let mut start: usize = 0;
            let mut end;
            let mut iters = 0;
            for y in x..num_events {
                let ev2 = &event_vec[y];
                if ev2.get_id().is_some() {
                    break;
                }

                iters += 1;
                if ev2.sleep() {
                    start = ev2.mins() as usize;
                } else {
                    end = ev2.mins() as usize;
                    assert!(start < end);
                    for i in start..end {
                        record[i] += 1;
                        record[60] += 1;
                    }
                }
            }
            x += iters;
        }
    }

    let mut best_id = -1;
    let mut best_minute = 0;
    if (part1) {
        // part1
        let mut best_total = 0;
        for (k, v) in map {
            let total = v[60];
            if total > best_total {
                best_id = k;
                best_total = total;
                let mut min = -1;
                for i in 0..60 {
                    let tmp = v[i];
                    if tmp > min {
                        min = tmp;
                        best_minute = i;
                    }
                }
                //println!("{:?}{:?}", &v[0..31], &v[32..60]);
            }
        }
    } else {
        // part2
        let mut best_count = 0;
        for (k, v) in map {
            let mut count = -1;
            let mut min = 0;
            for i in 0..60 {
                let tmp= v[i];
                if tmp > count {
                    count = tmp;
                    min = i;
                }
            }
            if count > best_count {
                best_count = count;
                best_id = k;
                best_minute = min;
                //println!("{:?}{:?}", &v[0..31], &v[32..60]);
            }
        }
    }

    println!("result:{}", best_id * best_minute as i32);
}

fn main() {
    // part1 and part2 in same sln
    solution(false);
}
