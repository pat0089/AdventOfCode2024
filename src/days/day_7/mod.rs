use std::fs::read_to_string;

pub fn run() {
    println!("Day 7");

    let input = match read_to_string("./src/days/day_7/input.txt") {
        Ok(input) => input,
        Err(error) => panic!("{}", error),
    };

    let intermediate = input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(key, value)| {
            (
                key.parse::<u64>().unwrap(),
                value
                    .split(" ")
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let input2 = vec![
        "190: 10 19",
        "3267: 81 40 27",
        "83: 17 5",
        "156: 15 6",
        "7290: 6 8 6 15",
        "161011: 16 10 13",
        "192: 17 8 14",
        "21037: 9 7 18 13",
        "292: 11 6 16 20",
    ];

    let intermediate2 = input2
        .iter()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(key, value)| {
            (
                key.parse::<u64>().unwrap(),
                value
                    .split(" ")
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    //okay, this one is interesting: we need to sort ascending each line if the input for part 1

    println!("Part 1: {}", part_1(&intermediate));
    println!("Part 2: {}", part_2(&intermediate));
}

fn part_1(input: &Vec<(u64, Vec<u64>)>) -> u64 {
    input.iter().fold(0, |acc, (key, value)| {
        if is_safe(value, 0, *key) {
            acc + *key
        } else {
            acc
        }
    })
}

fn part_2(input: &Vec<(u64, Vec<u64>)>) -> u64 {
    input.iter().fold(0, |acc, (key, value)| {
        if is_safe_concatenate(value, 0, *key) {
            acc + *key
        } else {
            acc
        }
    })
}

fn is_safe(report: &[u64], accumulator: u64, target: u64) -> bool {
    if report.len() == 0 {
        return accumulator == target;
    } else {
        return is_safe(&report[1..], accumulator + report[0], target)
            || is_safe(&report[1..], accumulator * report[0], target);
    }
}

fn is_safe_concatenate(report: &[u64], accumulator: u64, target: u64) -> bool {
    if report.len() == 0 {
        return accumulator == target;
    } else {
        return is_safe_concatenate(&report[1..], concatenate(accumulator, report[0]), target)
            || is_safe_concatenate(&report[1..], accumulator + report[0], target)
            || is_safe_concatenate(&report[1..], accumulator * report[0], target);
    }
}

fn concatenate(accumulator: u64, report: u64) -> u64 {
    let exponent = 10u64.pow((report.to_string().len()) as u32);
    accumulator * exponent + report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenate() {
        assert_eq!(concatenate(1, 2), 12);
        assert_eq!(concatenate(12, 2), 122);
        assert_eq!(concatenate(123, 45), 12345);
    }
}
