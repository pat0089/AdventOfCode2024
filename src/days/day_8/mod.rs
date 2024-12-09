use std::{
    collections::{btree_set::Intersection, HashMap, HashSet},
    fs::read_to_string,
    hash::Hash,
};

pub fn run() {
    println!("Day 8");

    let input = match read_to_string("./src/days/day_8/input.txt") {
        Ok(input) => input,
        Err(err) => panic!("Failed to read input: {}", err),
    };

    let intermediate = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let input2 = vec![
        "............",
        "........0...",
        ".....0......",
        ".......0....",
        "....0.......",
        "......A.....",
        "............",
        "............",
        "........A...",
        ".........A..",
        "............",
        "............",
    ];

    let intermediate2 = input2
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&intermediate));
    println!("Part 2: {}", part_2(&intermediate));
}

fn part_1(input: &Vec<Vec<char>>) -> u32 {
    //get input size
    let input_size = (input.len(), input[0].len());

    //map antennae frequency to a list of locations
    let antennae_locations = get_antennae_locations(input);

    let mut antinode_locations: HashSet<(i32, i32)> = HashSet::new();

    antennae_locations.iter().for_each(|(_, antennae)| {
        //pair up all antennae locations, then find the positive and negative delta between them
        // to add to the antinode locations list
        let combinations = get_combinations(antennae);

        combinations.iter().for_each(|(x, y)| {
            let dx = x.0 - y.0;
            let dy = x.1 - y.1;

            let delta = (dx, dy);

            let location1 = (x.0 + delta.0, x.1 + delta.1);
            let location2 = (y.0 - delta.0, y.1 - delta.1);

            if is_valid_location(input_size, location1) {
                antinode_locations.insert(location1);
            }
            if is_valid_location(input_size, location2) {
                antinode_locations.insert(location2);
            }
        });
    });
    antinode_locations.len() as u32
}

fn get_antennae_locations(input: &[Vec<char>]) -> HashMap<char, Vec<(i32, i32)>> {
    input
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, line)| {
            line.iter().enumerate().for_each(|(j, &c)| match c {
                '.' => {}
                c => {
                    acc.entry(c)
                        .or_insert(Vec::new())
                        .push((i as i32, j as i32));
                }
            });
            acc
        })
}

fn get_combinations(antennae: &Vec<(i32, i32)>) -> Vec<((i32, i32), (i32, i32))> {
    antennae
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            antennae
                .iter()
                .skip(i + 1)
                .map(move |y| (x.clone(), y.clone()))
        })
        .collect::<Vec<_>>()
}

fn is_valid_location(input_size: (usize, usize), position: (i32, i32)) -> bool {
    if position.0 < 0
        || position.0 >= input_size.0 as i32
        || position.1 < 0
        || position.1 >= input_size.1 as i32
    {
        false
    } else {
        true
    }
}

fn part_2(input: &Vec<Vec<char>>) -> u32 {
    //get input size
    let input_size = (input.len(), input[0].len());

    //map antennae frequency to a list of locations
    let antennae_locations = get_antennae_locations(input);

    let mut antinode_locations: HashSet<(i32, i32)> = HashSet::new();

    antennae_locations.iter().for_each(|(_, antennae)| {
        //pair up all antennae locations, then find the positive and negative delta between them
        // to add to the antinode locations list
        let combinations = get_combinations(antennae);

        combinations.iter().for_each(|(x, y)| {
            if antennae.len() > 1 {
                antinode_locations.insert(*x);
                antinode_locations.insert(*y);
            }

            let dx = x.0 - y.0;
            let dy = x.1 - y.1;

            let delta = (dx, dy);
            let neg_delta = (-dx, -dy);

            let mut location1 = (x.0 + delta.0, x.1 + delta.1);
            let mut location2 = (y.0 + neg_delta.0, y.1 + neg_delta.1);

            while is_valid_location(input_size, location1) {
                antinode_locations.insert(location1);
                location1 = (location1.0 + delta.0, location1.1 + delta.1);
            }
            while is_valid_location(input_size, location2) {
                antinode_locations.insert(location2);
                location2 = (location2.0 + neg_delta.0, location2.1 + neg_delta.1);
            }
        });
    });

    antinode_locations.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations() {
        let antennae = vec![(0, 0), (1, 1), (2, 2), (3, 3)];
        let combinations = get_combinations(&antennae);
        assert_eq!(combinations.len(), 6);
    }
}
