use std::{collections::HashMap, fs::read_to_string};

pub fn run() {
    println!("Day 1");

    let input = match read_to_string("./src/days/day_1/input.txt") {
        Ok(input) => input,
        Err(err) => panic!("Failed to read input: {}", err),
    };

    let (inter1, inter2): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .unzip();

    let inter1: Vec<_> = inter1
        .iter()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    let inter2: Vec<_> = inter2
        .iter()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    println!("Part 1: {}", part_1(inter1.clone(), inter2.clone()));

    println!("Part 2: {}", part_2(inter1.clone(), inter2.clone()));
}

fn part_1(mut inter: Vec<u32>, mut inter2: Vec<u32>) -> u32 {
    inter.sort();
    inter2.sort();
    inter.iter().zip(inter2.iter()).fold(0, |acc, (x, y)| acc + x.abs_diff(*y))    
}

fn part_2(left: Vec<u32>, right: Vec<u32>) -> u32 {

    let right_map = right.into_iter().fold(HashMap::new(), |mut acc, i| {
        *acc.entry(i).or_insert(0) += 1;
        acc
    });

    left.iter().fold(0, |acc: u32, l| {
        let x = match right_map.get(l) {
            Some(r) => acc + r * l,
            _ => acc,
        };
        x
    })
}