use crate::prelude::*;

pub fn run_day() -> Result<()> {
    let input = include_str!("../input/6.txt");

    println!("Task 1: {}", task1(input)?);
    Ok(())
}

fn task1(input: &str) -> Result<u32> {
    let input = parse_input(input)?
        .iter()
        .map(Record::breaking_distances)
        .map(|d| d.len() as u32)
        .product();
    Ok(input)
}

impl Record {
    fn breaking_distances(&self) -> Vec<u32> {
        (1..self.time)
            .map(|held| (self.time - held) * held)
            .filter(|distance| distance > &self.distance)
            .collect()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Record {
    time: u32,
    distance: u32,
}

fn parse_input(s: &str) -> Result<Vec<Record>> {
    let mut lines = s.lines();

    let times = lines
        .next()
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .map(|n| n.parse::<u32>().unwrap())
        })
        .unwrap();

    let distance = lines
        .next()
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .map(|n| n.parse::<u32>().unwrap())
        })
        .unwrap();

    Ok(times
        .zip(distance)
        .map(|(time, distance)| Record { time, distance })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST).unwrap();

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

    const TEST: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;
}
