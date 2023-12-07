use itertools::Itertools;

use crate::prelude::*;

pub fn run_day() -> Result<()> {
    let input = include_str!("../input/6.txt");

    println!("Task 1: {}", task1(input)?);
    println!("Task 2: {}", task2(input)?);
    Ok(())
}

fn task1(input: &str) -> Result<u32> {
    let input = parse_input_task1(input)?
        .iter()
        .map(Record::breaking_distances)
        .map(|d| d.len() as u32)
        .product();
    Ok(input)
}

fn task2(input: &str) -> Result<u128> {
    let record = parse_input_task2(input)?;

    Ok(record.breaking_distances().len() as u128)
}

impl Record {
    fn breaking_distances(&self) -> Vec<u128> {
        (1..self.time)
            .map(|held| (self.time - held) * held)
            .filter(|distance| distance > &self.distance)
            .collect()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Record {
    time: u128,
    distance: u128,
}

fn parse_input_task1(s: &str) -> Result<Vec<Record>> {
    let mut lines = s.lines();

    let times = lines
        .next()
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .map(|n| n.parse::<u128>().unwrap())
        })
        .unwrap();

    let distance = lines
        .next()
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .map(|n| n.parse::<u128>().unwrap())
        })
        .unwrap();

    Ok(times
        .zip(distance)
        .map(|(time, distance)| Record { time, distance })
        .collect())
}

fn parse_input_task2(s: &str) -> Result<Record> {
    let parse_num = |line: &str| {
        line.split_ascii_whitespace()
            .skip(1)
            .join("")
            .parse::<u128>()
    };

    let mut lines = s.lines();
    let time = lines.next().map(parse_num).unwrap()?;

    let distance = lines.next().map(parse_num).unwrap()?;

    Ok(Record { time, distance })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let result = parse_input_task1(TEST).unwrap();

        assert_eq!(
            result,
            vec![
                Record {
                    time: 7,
                    distance: 9
                },
                Record {
                    time: 15,
                    distance: 40
                },
                Record {
                    time: 30,
                    distance: 200
                }
            ]
        )
    }

    #[test]
    fn test_task_1() {
        let result = task1(TEST).unwrap();

        assert_eq!(result, 288)
    }

    #[test]
    fn test_parse_input_task2() {
        let result = parse_input_task2(TEST).unwrap();

        assert_eq!(
            result,
            Record {
                time: 71530,
                distance: 940200
            }
        )
    }

    #[test]
    fn test_task2() {
        let result = task2(TEST).unwrap();

        assert_eq!(result, 71503)
    }

    const TEST: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;
}
