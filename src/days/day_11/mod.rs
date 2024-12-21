use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn run() {
    println!("Day 11");

    let input = match read_to_string("./src/days/day_11/input.txt") {
        Ok(input) => input,
        Err(err) => panic!("Failed to read input: {}", err),
    };

    let input2 = "125 17".to_string();

    let intermediate = input
        .split_ascii_whitespace()
        .map(|str| str.to_string().parse().unwrap())
        .collect::<Vec<u64>>();

    let mut intermediate_map: HashMap<u64, u64> = HashMap::new();
    intermediate
        .iter()
        .for_each(|stone| *intermediate_map.entry(*stone).or_insert(0) += 1);

    let intermediate2 = input2
        .split_ascii_whitespace()
        .map(|x| x.to_string().parse().unwrap())
        .collect::<Vec<u64>>();

    let mut intermediate2_map: HashMap<u64, u64> = HashMap::new();
    intermediate2
        .iter()
        .for_each(|stone| *intermediate2_map.entry(*stone).or_insert(0) += 1);

    println!("Part 1: {}", part_1(&intermediate_map));
    println!("Part 2: {}", part_2(&intermediate_map));
}

fn part_1(input: &HashMap<u64, u64>) -> u64 {
    let mut stones = input.clone();
    for _ in 0..25 {
        stones = blink(&stones);
    }
    stones.values().fold(0, |acc, x| acc + x)
}

fn part_2(input: &HashMap<u64, u64>) -> u64 {
    let mut stones = input.clone();
    for _ in 0..75 {
        stones = blink(&stones);
    }
    stones.values().fold(0, |acc, x| acc + x)
}

fn blink(input: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut stones = HashMap::new();

    for (stone, count) in input.iter() {
        if *stone == 0 {
            *stones.entry(1).or_insert(0) += count;
        } else {
            let stone_string = stone.to_string();
            let length = stone_string.len();
            if length % 2 == 0 {
                let half = length / 2;
                let left = &stone_string[0..half];
                let right = &stone_string[half..length];
                let left_stone = left.parse::<u64>().unwrap();
                let right_stone = right.parse::<u64>().unwrap();
                *stones.entry(left_stone).or_insert(0) += count;
                *stones.entry(right_stone).or_insert(0) += count;
            } else {
                *stones.entry(stone * 2024).or_insert(0) += count;
            }
        }
    }

    stones
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let stones = vec![
            "0".to_string(),
            "1".to_string(),
            "10".to_string(),
            "99".to_string(),
            "999".to_string(),
        ]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();

        let mut input = HashMap::new();
        stones.iter().for_each(|s| {
            *input.entry(*s).or_insert(0) += 1;
        });

        println!("input: {:#?}", input);
        println!("blink: {:#?}", blink(&input));
    }

    #[test]
    fn run_test_1() {
        let input2 = "125 17".to_string();

        let intermediate2 = input2
            .split_ascii_whitespace()
            .map(|x| x.to_string().parse().unwrap())
            .collect::<Vec<u64>>();

        let mut intermediate2_map: HashMap<u64, u64> = HashMap::new();
        intermediate2
            .iter()
            .for_each(|stone| *intermediate2_map.entry(*stone).or_insert(0) += 1);

        assert_eq!(55312, part_1(&intermediate2_map));
    }
}
