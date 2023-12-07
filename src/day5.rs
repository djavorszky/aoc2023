use std::{ops::RangeInclusive, str::FromStr};

use crate::prelude::*;

pub fn run_day() -> Result<()> {
    let input = include_str!("../input/5.txt");

    println!("task 1: {}", task1(input)?);

    Ok(())
}

fn task1(input: &str) -> Result<i128> {
    let input = input.parse::<Task1Input>()?;

    let seeds = input.seeds;
    let mappers = input.mappers;

    let minimum = seeds
        .into_iter()
        .map(|mut n| {
            for mapper in &mappers {
                n = transform(n, &mapper);
            }

            n
        })
        .min()
        .unwrap();

    Ok(minimum)
}

fn transform(n: i128, transformations: &Vec<Transformation>) -> i128 {
    transformations
        .iter()
        .find(|t| t.in_range(n))
        .map(|transform| n + transform.op)
        .unwrap_or(n)
}

#[derive(PartialEq, Eq, Debug)]
struct Transformation {
    range: RangeInclusive<i128>,
    op: i128,
}

impl Transformation {
    fn in_range(&self, n: i128) -> bool {
        self.range.contains(&n)
    }
}

impl FromStr for Transformation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split_ascii_whitespace();

        let end = parts.next().map(|c| c.parse::<i128>()).unwrap()?;
        let start = parts.next().map(|c| c.parse::<i128>()).unwrap()?;
        let size = parts.next().map(|c| c.parse::<i128>()).unwrap()?;

        Ok(Self {
            range: (start..=start + size),
            op: end - start,
        })
    }
}

#[derive(Debug)]
struct Task1Input {
    seeds: Vec<i128>,
    mappers: Vec<Vec<Transformation>>,
}

impl FromStr for Task1Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut chunks = s.split("\n\n");

        let seeds = chunks
            .next()
            .and_then(|line| line.split_once(": "))
            .map(|(_, nums)| {
                nums.split_ascii_whitespace()
                    .map(|n| n.parse::<i128>().unwrap())
            })
            .unwrap()
            .collect();

        let mappers = chunks
            .map(|chunk| {
                chunk
                    .lines()
                    .skip(1)
                    .map(|line| line.parse::<Transformation>().unwrap())
                    .collect()
            })
            .collect();

        Ok(Self { seeds, mappers })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_task1() {
        let result = task1(TEST).unwrap();

        assert_eq!(result, 35);
    }

    #[test]
    fn parse_input() {
        let parsed = TEST.parse::<Task1Input>().unwrap();

        assert_eq!(parsed.seeds.len(), 4);
        assert_eq!(parsed.mappers.len(), 7);
    }

    #[test]
    fn parse_transformation() {
        assert_eq!(
            "50 98 2".parse::<Transformation>().unwrap(),
            Transformation {
                op: -48,
                range: (98..=100)
            }
        )
    }

    const TEST: &'static str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
}
