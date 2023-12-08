use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::prelude::*;

pub fn run_day() -> Result<()> {
    let input = include_str!("../input/8.txt");
    println!("Task 1: {}", task1(input)?);
    println!("Task 2: {}", task2(input)?);
    Ok(())
}

fn task1(input: &str) -> Result<usize> {
    let (instructions, map) = parse_task1_input(input);

    let mut node = "AAA".to_string();
    let count = instructions
        .chars()
        .cycle()
        .take_while(|dir| {
            node = map.get(&node).map(|lr| take_dir(lr, dir)).unwrap();
            node != "ZZZ"
        })
        .count();

    Ok(count + 1)
}

fn take_dir(tuple: &(String, String), c: &char) -> String {
    match c {
        'L' => tuple.0.clone(),
        'R' => tuple.1.clone(),
        _ => panic!("Can only be called with L or R"),
    }
}

fn task2(input: &str) -> Result<usize> {
    let (instructions, map) = parse_task1_input(input);

    let start_positions: Vec<_> = map.keys().filter(|key| key.ends_with('A')).collect();

    let result = start_positions
        .into_iter()
        .map(|pos| {
            let mut node = pos.clone();

            instructions
                .chars()
                .cycle()
                .take_while(|dir| {
                    node = map.get(&node).map(|lr| take_dir(lr, dir)).unwrap();
                    !node.ends_with('Z')
                })
                .count()
        })
        .map(|n| n + 1)
        .flat_map(prime_factors)
        .collect::<HashSet<usize>>()
        .iter()
        .product();

    Ok(result)
}

fn prime_factors(n: usize) -> Vec<usize> {
    let mut result = Vec::new();
    let mut n = n;

    while n % 2 == 0 {
        result.push(2);
        n /= 2
    }

    while n % 3 == 0 {
        result.push(3);
        n /= 3
    }

    for i in (3..=(n / 2)).step_by(2) {
        while n % i == 0 {
            result.push(i);
            n /= i
        }
    }

    result
}

fn parse_task1_input(input: &str) -> (String, HashMap<String, (String, String)>) {
    let mut chunks = input.split("\n\n");

    let instructions = chunks.next().unwrap();

    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    let map = chunks
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                caps.get(1).unwrap().as_str().to_string(),
                (
                    caps.get(2).unwrap().as_str().to_string(),
                    caps.get(3).unwrap().as_str().to_string(),
                ),
            )
        })
        .collect();

    (instructions.to_string(), map)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_task1() {
        let result = task1(TEST).unwrap();
        assert_eq!(result, 2)
    }

    #[test]
    fn test_task1_other() {
        let result = task1(TEST2).unwrap();
        assert_eq!(result, 6)
    }

    #[test]
    fn test_task2() {
        let result = task2(TASK2_TEST).unwrap();
        assert_eq!(result, 6)
    }

    #[test]
    fn test_parse_task1() {
        let (instructions, map) = parse_task1_input(TEST);

        assert_eq!(instructions, "RL");
        assert_eq!(map.len(), 7);
        assert_eq!(
            map.get("CCC").unwrap(),
            &("ZZZ".to_string(), "GGG".to_string())
        );
    }

    const TEST: &str = r#"RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)"#;

    const TEST2: &str = r#"LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)"#;

    const TASK2_TEST: &str = r#"LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)"#;
}
