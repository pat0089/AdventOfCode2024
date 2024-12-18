use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

use crate::util::DIRECTIONS;

pub fn run() {
    println!("Day 6");

    let input = match read_to_string("./src/days/day_6/input.txt") {
        Ok(input) => input,
        Err(err) => panic!("{}", err),
    };

    let input2 = vec![
        "....#.....",
        ".........#",
        "..........",
        "..#.......",
        ".......#..",
        "..........",
        ".#..^.....",
        "........#.",
        "#.........",
        "......#...",
    ];

    let intermediate = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let intermediate2 = input2
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&intermediate));
    println!("Part 2: {}", part_2(&intermediate));
}

fn part_1(input: &Vec<Vec<char>>) -> u32 {
    //get starting position from input
    let starting_position = get_starting_position(input);

    //get all obstacles from original input
    let obstacles = get_obstacles(input);

    let input_size = (input.len(), input[0].len());

    //simulate and output the number of unique positions visited
    simulate(input_size, &obstacles, starting_position).len() as u32
}

fn simulate(
    input_size: (usize, usize),
    obstacles: &HashSet<(usize, usize)>,
    starting_position: (usize, usize),
) -> HashSet<(usize, usize)> {
    let mut direction_index = 0;

    let mut visited = HashSet::new();

    let mut position: (i32, i32) = (starting_position.0 as i32, starting_position.1 as i32);

    while position_is_valid(input_size, position) {
        //do the movement:

        //add current position to visited
        if !visited.contains(&(position.0 as usize, position.1 as usize)) {
            visited.insert((position.0 as usize, position.1 as usize));
        }

        //actually move
        position = movement(position, &mut direction_index, obstacles);
    }

    visited
}

fn position_is_valid(size: (usize, usize), position: (i32, i32)) -> bool {
    if position.0 < 0
        || position.0 >= size.0 as i32
        || position.1 < 0
        || position.1 >= size.1 as i32
    {
        false
    } else {
        true
    }
}

fn get_next_position(position: (i32, i32), direction_index: usize) -> (i32, i32) {
    (
        (position.0 + DIRECTIONS[direction_index].0),
        (position.1 + DIRECTIONS[direction_index].1),
    )
}

fn should_turn(obstacles: &HashSet<(usize, usize)>, next_position: &(i32, i32)) -> bool {
    let next_position = (next_position.0 as usize, next_position.1 as usize);
    obstacles.contains(&next_position)
}

fn get_starting_position(input: &Vec<Vec<char>>) -> (usize, usize) {
    input
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .enumerate()
                .find_map(|(j, c)| if *c == '^' { Some((i, j)) } else { None })
        })
        .unwrap()
}

fn get_obstacles(input: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter().enumerate().filter_map(
                move |(j, c)| {
                    if *c == '#' {
                        Some((i, j))
                    } else {
                        None
                    }
                },
            )
        })
        .collect()
}

fn part_2(input: &Vec<Vec<char>>) -> u32 {
    //get starting position from input
    let starting_position = get_starting_position(input);

    //get all obstacles from original input
    let obstacles = get_obstacles(input);

    //get the size of the input for simulation bounds checking
    let input_size = (input.len(), input[0].len());

    //find all visited positions
    let visited = simulate(input_size, &obstacles, starting_position);

    //filter out the starting position because we wouldn't be able to put an obstacle there
    let new_obstacles = visited
        .iter()
        .fold(HashSet::new(), |mut obstacles, &position| {
            if position != starting_position {
                obstacles.insert(position);
            }
            obstacles
        });

    new_obstacles.iter().fold(0, |acc, &position| {
        //add this obstacle before simulation
        let mut new_obstacles = obstacles.clone();
        new_obstacles.insert(position);

        //simulate to find if it loops
        let loops = simulate_with_loops(input_size, &new_obstacles, starting_position);

        acc + if loops { 1 } else { 0 }
    })
}

fn movement(
    position: (i32, i32),
    index: &mut usize,
    obstacles: &HashSet<(usize, usize)>,
) -> (i32, i32) {
    //check if we need to turn, then turn as many times as needed,
    // updating the next position accordingly
    let mut next_position = get_next_position(position, *index);

    while should_turn(obstacles, &next_position) {
        *index += 1;
        *index = *index % 4;
        next_position = get_next_position(position, *index);
    }
    next_position
}

fn simulate_with_loops(
    input_size: (usize, usize),
    obstacles: &HashSet<(usize, usize)>,
    starting_position: (usize, usize),
) -> bool {
    let mut slow_index = 0;
    let mut fast_index = 0;

    let mut slow = (starting_position.0 as i32, starting_position.1 as i32);
    let mut fast = (slow.0, slow.1);

    while position_is_valid(input_size, fast) {
        //do the movement:

        //actually move
        slow = movement(slow, &mut slow_index, obstacles);
        fast = movement(fast, &mut fast_index, obstacles);
        if position_is_valid(input_size, fast) {
            fast = movement(fast, &mut fast_index, obstacles);
        }

        if slow == fast && slow_index == fast_index {
            return true;
        }
    }

    false
}
