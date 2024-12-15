use std::{collections::HashSet, fs::read_to_string, vec};

pub fn run() {
    println!("Day 9");

    let input = match read_to_string("./src/days/day_9/input.txt") {
        Ok(input) => input,
        Err(err) => panic!("Failed to read input: {}", err),
    };

    let input2: Vec<&str> = vec!["2333133121414131402"];

    let intermediate2 = input2
        .iter()
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let intermediate = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&intermediate));
    println!("Part 2: {}", part_2(&intermediate));
}

fn part_1(input: &Vec<u32>) -> u64 {
    let disk_by_id = get_disk_by_id(input)
        .into_iter()
        .flat_map(|region| region)
        .collect::<Vec<_>>();

    let mut defragged = Vec::new();

    let (mut front, mut back) = (0 as usize, disk_by_id.len() - 1);

    while front <= back {
        if disk_by_id[front].is_none() {
            //get the first non-None value before from the back
            while disk_by_id[back].is_none() {
                back -= 1;
            }
            defragged.push(disk_by_id[back].unwrap());
            back -= 1;
        } else {
            defragged.push(disk_by_id[front].unwrap());
        }
        front += 1;
    }

    calculate_checksum(&defragged)
}

fn get_disk_by_id(input: &[u32]) -> Vec<Vec<Option<u32>>> {
    input
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, &x)| {
            if i % 2 == 0 {
                acc.push(vec![Some(i as u32 / 2); x as usize]);
            } else {
                acc.push(vec![None; x as usize]);
            }
            acc
        })
}

fn calculate_checksum(input: &Vec<u32>) -> u64 {
    input
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &x)| acc + x as u64 * i as u64)
}

fn calculate_optional_checksum(input: &Vec<Option<u32>>) -> u64 {
    input.iter().enumerate().fold(0, |acc, (i, &x)| match x {
        None => acc,
        Some(x) => acc + x as u64 * i as u64,
    })
}

fn part_2(input: &Vec<u32>) -> u64 {
    let mut disk_by_id = get_disk_by_id(input)
        .iter()
        .map(|region| (region.len(), region.iter().find_map(|x| *x)))
        .collect::<Vec<_>>();

    let mut visited: HashSet<u32> = HashSet::new();

    //go from the back to the front so we can get the first non-None value
    //then find the first None space big enough to fit the first non-None value
    // if it exists, otherwise leave it where it is
    for i in (0..disk_by_id.len()).rev() {
        if (disk_by_id[i].1).is_some() {
            let value = disk_by_id[i].1.unwrap();
            if visited.contains(&disk_by_id[i].1.unwrap()) {
                continue;
            }
            match find_first_valid_empty_index(&disk_by_id, disk_by_id[i].0) {
                Some(index) => {
                    if index < i {
                        modified_swap(&mut disk_by_id, index, i);
                        concatenate_adjacent_none(&mut disk_by_id);
                    }
                }
                None => {}
            }
            visited.insert(value);
            //print_disk(&disk_by_id);
        }
    }

    let defragged = disk_by_id
        .iter()
        .map(|region| vec![region.1; region.0])
        .flatten()
        .collect::<Vec<_>>();

    //print_disk(&disk_by_id);

    calculate_optional_checksum(&defragged)
}

fn print_disk(disk: &Vec<(usize, Option<u32>)>) {
    let defragged = disk
        .iter()
        .map(|region| vec![region.1; region.0])
        .flatten()
        .collect::<Vec<_>>();

    for i in 0..defragged.len() {
        if defragged[i].is_none() {
            print!(".")
        } else {
            print!("{}", defragged[i].unwrap())
        }
    }
    println!();
}

fn find_first_valid_empty_index(input: &Vec<(usize, Option<u32>)>, size: usize) -> Option<usize> {
    for i in 0..input.len() {
        if input[i].1.is_none() && input[i].0 >= size {
            return Some(i);
        }
    }
    None
}

fn concatenate_adjacent_none(input: &mut Vec<(usize, Option<u32>)>) {
    let mut index = 0;
    while index < input.len() - 1 {
        if input[index].1.is_none() && input[index + 1].1.is_none() {
            input[index].0 += input[index + 1].0;
            input.remove(index + 1);
        } else {
            index += 1;
        }
    }
}

fn modified_swap(input: &mut Vec<(usize, Option<u32>)>, left: usize, right: usize) {
    if input[left].0 == input[right].0 {
        input.swap(left, right);
    } else {
        let diff = input[left].0 - input[right].0;
        let moving_right = input[right].clone();
        input[left].0 = diff;
        input[right].1 = None;
        input.insert(left, moving_right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2_funcs() {
        let input2: Vec<&str> = vec!["2333133121414131402"];

        let intermediate2 = input2
            .iter()
            .flat_map(|line| line.chars().collect::<Vec<_>>())
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        let mut input = get_disk_by_id(&intermediate2)
            .iter()
            .map(|region| (region.len(), region.iter().find_map(|x| *x)))
            .collect::<Vec<_>>();

        let last = input.len() - 1;

        let found = find_first_valid_empty_index(&input, input[last].0);

        println!("{:?}", found);

        modified_swap(&mut input, found.unwrap(), last);
        concatenate_adjacent_none(&mut input);

        print_disk(&input);
    }
}
