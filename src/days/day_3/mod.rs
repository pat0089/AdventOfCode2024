use std::fs::read_to_string;

use regex::Regex;

pub fn run() {
    println!("Day 3");

    let input = match read_to_string("./src/days/day_3/input.txt") {
        Ok(input) => input,
        Err(err) => panic!("Failed to read input: {}", err),
    };

    let regex = Regex::new(r"mul\((-?\d+(\.\d+)?),(-?\d+(\.\d+)?)\)").unwrap();

    let accumulate = |accumulator: f64, capture: regex::Captures| -> f64 {
        let x = capture[1].parse::<f64>().unwrap();
        let y = capture[3].parse::<f64>().unwrap();
        accumulator + x * y
    };

    println!(
        "Part 1: {}",
        regex.captures_iter(&input).fold(0.0, accumulate)
    );

    let do_regex = Regex::new(r"mul\((-?\d+(\.\d+)?),(-?\d+(\.\d+)?)\)|do\(\)|don't\(\)").unwrap();

    let mut doing = true;

    let accumulate2 = |accumulator: f64, capture: regex::Captures| -> f64 {
        if capture.get(1).is_some() && capture.get(3).is_some() && doing {
            let x = capture[1].parse::<f64>().unwrap();
            let y = capture[3].parse::<f64>().unwrap();

            return accumulator + x * y;
        }
        match capture.get(0) {
            Some(s) => match s.as_str() {
                "do()" => doing = true,
                "don't()" => doing = false,
                _ => (),
            },
            None => (),
        }
        accumulator
    };

    println!(
        "Part 2: {}",
        do_regex.captures_iter(&input).fold(0.0, accumulate2)
    );
}
