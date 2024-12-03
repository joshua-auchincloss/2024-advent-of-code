use std::env::args;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
}

impl Direction {
    fn new(last: &u32, item: &u32) -> Self {
        if last.lt(item) {
            Direction::Up
        } else {
            Direction::Down
        }
    }
}

fn is_report_safe_with_tolerance(items: Vec<u32>) -> bool {
    let permuts = {
        let mut permuts = vec![];
        for i in 0..items.len() {
            let mut permut = items.clone();
            permut.remove(i);
            permuts.push(permut);
        }
        permuts
    };
    permuts.into_iter().any(is_report_safe)
}

fn is_report_safe(items: Vec<u32>) -> bool {
    let mut last = 0_u32;
    let mut direction = Direction::Up;

    for (i, item) in items.iter().enumerate() {
        if i == 0 {
            last = *item;
            continue;
        } else if i == 1 {
            direction = Direction::new(&last, item);
        }
        let diff = last.abs_diff(*item);
        if diff > 3 || diff < 1 || Direction::new(&last, item) != direction {
            return false;
        }
        last = *item;
    }
    true
}

fn parse_report(ln: &str) -> Vec<u32> {
    let mut report = vec![];
    for int in ln.trim().split(" ") {
        if int.is_empty() {
            continue;
        }
        report.push(int.parse().expect("valid u32 level"))
    }
    report
}

fn parse_input(buf: String) -> Vec<Vec<u32>> {
    let mut found = vec![];
    for report in buf.trim().split("\n") {
        found.push(parse_report(report))
    }
    found
}

fn main() {
    let args = common::args();
    let buf = args.file();
    let with_tolerance = args.boolean_flag("--with-tolerance");

    let parsed = parse_input(buf);

    let sln: Vec<bool> = parsed
        .into_iter()
        .map(if with_tolerance {
            is_report_safe_with_tolerance
        } else {
            is_report_safe
        })
        .collect();

    let count = sln.iter().filter(|this| **this).count();
    println!("{}/{} reports are safe", count, sln.len())
}
