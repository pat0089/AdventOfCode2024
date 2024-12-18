use std::{collections::HashSet, fs::read_to_string, iter};

use crate::util::DIRECTIONS;

pub fn run() {
    println!("Day 10");

    let input = match read_to_string("./src/days/day_10/input.txt") {
        Ok(input) => input,
        Err(err) => panic!("Failed to read input: {}", err),
    };

    let input2 = vec![
        "89010123", "78121874", "87430965", "96549874", "45678903", "32019012", "01329801",
        "10456732",
    ];

    let intermediate = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .map(|c| {
            c.iter()
                .map(|&c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let intermediate2 = input2
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .map(|c| {
            c.iter()
                .map(|&c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let trailheads = get_trailheads(&intermediate);

    let trailheads2 = get_trailheads(&intermediate);

    println!("Part 1: {}", part_1(&intermediate, &trailheads));
    println!("Part 2: {}", part_2(&intermediate, &trailheads2));
}

fn part_1(input: &[Vec<u32>], trailheads: &HashSet<(i32, i32)>) -> u32 {
    //println!("trailheads: {}", trailheads.len());
    trailheads.iter().fold(0, |acc, &(row, col)| {
        //println!("trailhead: ({}, {})", row, col);
        let score = try_trailhead(input, row, col);
        //println!("score: {}", score);
        acc + score
    })
}

fn part_2(input: &[Vec<u32>], trailheads: &HashSet<(i32, i32)>) -> u32 {
    trailheads.iter().fold(0, |acc, &(row, col)| {
        let score = try_trailhead2(input, row, col);
        acc + score
    })
}

fn try_trailhead(input: &[Vec<u32>], x: i32, y: i32) -> u32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    DIRECTIONS.iter().fold(0, |acc, &(r, c)| {
        acc + try_step(input, x + r, y + c, 1, &mut visited)
    })
}

fn try_trailhead2(input: &[Vec<u32>], x: i32, y: i32) -> u32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    DIRECTIONS.iter().fold(0, |acc, &(r, c)| {
        acc + try_step2(input, x + r, y + c, 1, &mut visited)
    })
}

fn try_step(
    input: &[Vec<u32>],
    x: i32,
    y: i32,
    target: u32,
    visited: &mut HashSet<(i32, i32)>,
) -> u32 {
    //check if stepping out of bounds, return 0
    if x >= input.len() as i32 || y >= input[0].len() as i32 || x < 0 || y < 0 {
        return 0;
    }

    if input[x as usize][y as usize] != target {
        return 0;
    }

    //check if at a 9 and target is also 9, to return 1
    if target == 9 {
        visited.insert((x, y));
        return 1;
    }

    // Try all directions recursively and sum the results
    let result = DIRECTIONS.iter().fold(0, |acc, &(dx, dy)| {
        if !visited.contains(&(x + dx, y + dy)) {
            return acc + try_step(input, x + dx, y + dy, target + 1, visited);
        }
        acc
    });

    return result;
}

fn try_step2(
    input: &[Vec<u32>],
    x: i32,
    y: i32,
    target: u32,
    visited: &mut HashSet<(i32, i32)>,
) -> u32 {
    //check if stepping out of bounds, return 0
    if x >= input.len() as i32 || y >= input[0].len() as i32 || x < 0 || y < 0 {
        return 0;
    }

    if input[x as usize][y as usize] != target {
        return 0;
    }

    visited.insert((x, y));

    //check if at a 9 and target is also 9, to return 1
    if target == 9 {
        return 1;
    }

    // Try all directions recursively and sum the results
    let result = DIRECTIONS.iter().fold(0, |acc, &(dx, dy)| {
        acc + try_step2(input, x + dx, y + dy, target + 1, visited)
    });

    visited.remove(&(x, y));

    return result;
}

fn get_trailheads(intermediate: &[Vec<u32>]) -> HashSet<(i32, i32)> {
    intermediate
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &c)| c == 0)
                .map(move |(col, _)| (row as i32, col as i32))
        })
        .collect::<HashSet<_>>()
}
