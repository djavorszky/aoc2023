use crate::prelude::*;

pub fn run_day() -> Result<()> {
    let input = include_str!("../input/9.txt");

    println!("task 1: {}", task1(input)?);
    Ok(())
}

fn task1(input: &str) -> Result<i32> {
    let result = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|seq| predict_next(&seq))
        .sum();

    Ok(result)
}

fn predict_next(seq: &[i32]) -> i32 {
    if seq.iter().all(|&n| n == 0) {
        return 0;
    }

    seq.last().unwrap()
        + predict_next(
            &seq.windows(2)
                .map(|win| win[1] - win[0])
                .collect::<Vec<i32>>(),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task1() {
        let result = task1(TEST).unwrap();

        assert_eq!(result, 114);
    }

    #[test]
    fn test_predict_next() {
        let test1: Vec<i32> = "0 3 6 9 12 15"
            .split_ascii_whitespace()
            .map(|c| c.parse::<i32>().unwrap())
            .collect();

        assert_eq!(predict_next(&test1), 18);
    }
    #[test]
    fn test_predict_next_2() {
        let test1: Vec<i32> = "1 3 6 10 15 21"
            .split_ascii_whitespace()
            .map(|c| c.parse::<i32>().unwrap())
            .collect();

        assert_eq!(predict_next(&test1), 28);
    }

    const TEST: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
    "#;
}
