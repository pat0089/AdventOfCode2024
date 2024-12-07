use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn run() {
    let input1 = match read_to_string("./src/days/day_5/input.txt") {
        Ok(input) => input,
        Err(err) => panic!("Failed to read input: {}", err),
    };

    let input2 = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    let (rules, input) = parse_input(&input1);

    println!("Day 5 Part 1: {}", part_1(&rules, &input));
    println!("Day 5 Part 2: {}", part_2(&rules, &input));
}

fn part_1(rules: &HashMap<u32, HashSet<u32>>, input: &Vec<Vec<u32>>) -> u32 {
    input.iter().fold(0, |acc, updates| {
        if is_valid_update_list(updates, rules) {
            acc + updates[updates.len() / 2]
        } else {
            acc
        }
    })
}

fn part_2(rules: &HashMap<u32, HashSet<u32>>, input: &Vec<Vec<u32>>) -> u32 {
    let input = input
        .iter()
        .filter(|&updates| !is_valid_update_list(updates, rules))
        .collect::<Vec<_>>();

    input.iter().fold(0, |acc, &updates| {
        let mut fixed_updates = updates.clone();
        fixed_updates.sort_by(|a, b| match rules.get(a) {
            Some(rules) => {
                if rules.contains(b) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            }
            None => std::cmp::Ordering::Less,
        });

        println!("{:?}", fixed_updates);

        acc + fixed_updates[fixed_updates.len() / 2]
    })
}

fn parse_input(input: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let (rules, input) = input
        .split_once("\n\n")
        .or_else(|| input.split_once("\r\n\r\n"))
        .unwrap();

    let rules = rules
        .lines()
        .map(|line| {
            let (from, to) = line.split_once("|").unwrap();
            (from.parse::<u32>().unwrap(), to.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();

    let rules = rules.iter().fold(HashMap::new(), |mut rules, (from, to)| {
        rules.entry(*from).or_insert(HashSet::new()).insert(*to);
        rules
    });

    let input = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (rules, input)
}

fn is_valid_update_list(updates: &Vec<u32>, rules: &HashMap<u32, HashSet<u32>>) -> bool {
    updates.iter().enumerate().all(|(i, update)| {
        //last entry is always valid since it has no rules
        if i + 1 == updates.len() {
            return true;
        }
        //if the update is not in the rules, then it is invalid
        //otherwise do the check for the rest of the updates
        match rules.get(update) {
            None => false,
            Some(rules) => updates[i + 1..].iter().enumerate().all(|(i, update)| {
                if i + 1 == updates.len() {
                    return true;
                }
                rules.contains(update)
            }),
        }
    })
}
