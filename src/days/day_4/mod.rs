use std::{collections::VecDeque, fs::read_to_string};

pub fn run() {
    let input = match read_to_string("./src/days/day_4/input.txt") {
        Ok(input) => input,
        Err(err) => panic!("Failed to read input: {}", err),
    };

    let intermediate = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let first = vec![
        "MMMSXXMASM",
        "MSAMXMSMSA",
        "AMXSXMAAMM",
        "MSAMASMSMX",
        "XMASAMXAMM",
        "XXAMMXXAMA",
        "SMSMSASXSS",
        "SAXAMASAAA",
        "MAMMMXMMMM",
        "MXMXAXMASX",
    ];

    let first_intermediate = first
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&intermediate));

    println!("Part 2: {}", part_2(&intermediate));
}

fn part_1(intermediate: &Vec<Vec<char>>) -> u32 {
    let directions = (-1..=1)
        .into_iter()
        .flat_map(|x| (-1..=1).into_iter().map(move |y| (x, y)))
        .filter(|(x, y)| *x != 0 || *y != 0)
        .collect::<Vec<_>>();

    let to_find = "XMAS".chars().collect::<VecDeque<_>>();
    let target = to_find[0];

    let locations = find_locations(&intermediate, target);
    let mut found_locations = vec![vec!['.'; intermediate[0].len()]; intermediate.len()];

    let found = locations.iter().fold(0, |accum, &location| {
        accum
            + directions
                .iter()
                .filter_map(|&direction| {
                    if find_word(
                        &intermediate,
                        &mut found_locations,
                        location,
                        to_find.clone(),
                        direction,
                    ) {
                        Some(1)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .len() as u32
    });

    found as u32
}

fn find_locations(intermediate: &Vec<Vec<char>>, target: char) -> Vec<(i32, i32)> {
    intermediate
        .iter()
        .enumerate() // Get row indices and rows
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate() // Get column indices and chars
                .filter_map(move |(col, &c)| {
                    if c == target {
                        Some((row as i32, col as i32))
                    } else {
                        None
                    }
                })
        })
        .collect::<Vec<_>>()
}

fn find_word(
    intermediate: &Vec<Vec<char>>,
    found_locations: &mut Vec<Vec<char>>,
    location: (i32, i32),
    word: VecDeque<char>,
    direction: (i32, i32),
) -> bool {
    let target = match word.get(0) {
        None => return false,
        Some(&c) => c,
    };
    let actual = match intermediate.get(location.0 as usize) {
        None => return false,
        Some(row) => match row.get(location.1 as usize) {
            None => return false,
            Some(&c) => c,
        },
    };
    if actual == target {
        let mut word = word.clone();
        word.pop_front();
        found_locations[location.0 as usize][location.1 as usize] = target;
        if word.len() == 0 {
            return true;
        } else {
            return find_word(
                intermediate,
                found_locations,
                (location.0 + direction.0, location.1 + direction.1),
                word,
                direction,
            );
        }
    } else {
        return false;
    }
}

fn part_2(intermediate: &Vec<Vec<char>>) -> u32 {
    let corner_pairs = vec![((1, 1), (-1, -1)), ((-1, 1), (1, -1))];

    let locations = find_locations(intermediate, 'A');
    //find all 'A's, then look at all of the directions to see if they are all valid,
    locations.iter().fold(0, |acc, &location| {
        if corner_pairs.iter().all(|(corner1, corner2)| {
            let location1 = (location.0 + corner1.0, location.1 + corner1.1);
            let location2 = (location.0 + corner2.0, location.1 + corner2.1);

            let char1 = match intermediate.get(location1.0 as usize) {
                None => return false,
                Some(row) => match row.get(location1.1 as usize) {
                    None => return false,
                    Some(&c) => c,
                },
            };
            let char2 = match intermediate.get(location2.0 as usize) {
                None => return false,
                Some(row) => match row.get(location2.1 as usize) {
                    None => return false,
                    Some(&c) => c,
                },
            };
            (char1 == 'M' || char1 == 'S') && (char2 == 'M' || char2 == 'S') && char1 != char2
        }) {
            acc + 1
        } else {
            acc
        }
    })
}
