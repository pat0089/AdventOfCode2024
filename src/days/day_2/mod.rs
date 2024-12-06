use std::fs::read_to_string;

pub fn run() {
    println!("Day 2");

    let input = match read_to_string("./src/days/day_2/input.txt") {
        Ok(input) => input,
        Err(err) => panic!("Failed to read input: {}", err),
    };

    let intermediate = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(intermediate.clone()));
    println!("Part 2: {}", part_2(intermediate.clone()));
}

fn part_1(intermediate: Vec<Vec<u32>>) -> u32 {
    intermediate
        .iter()
        .fold(0, |acc, report| if is_safe(report) { acc + 1 } else { acc })
}

fn is_safe(report: &Vec<u32>) -> bool {
    let diffs = report
        .windows(2)
        .map(|w| w[1] as i32 - w[0] as i32)
        .collect::<Vec<i32>>();

    if diffs.iter().all(|&d| d.abs() >= 1 && d.abs() <= 3)
        && diffs.iter().all(|&d| d > 0) ^ diffs.iter().all(|&d| d < 0)
    {
        true
    } else {
        false
    }
}

fn part_2(intermediate: Vec<Vec<u32>>) -> u32 {
    intermediate.iter().fold(0, |acc, report| {
        if is_safe(report) {
            acc + 1
        } else {
            //try again without the first element that doesn't fit the rules
            if report
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    report
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| *j != i)
                        .map(|(_, &r)| r)
                        .collect::<Vec<u32>>()
                })
                .any(|report| is_safe(&report))
            {
                acc + 1
            } else {
                acc
            }
        }
    })
}
