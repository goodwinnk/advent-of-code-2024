use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use indoc::indoc;
use std::io::{BufRead, BufReader};

fn part1<R: BufRead>(_reader: R) -> Result<i64> {
    Ok(0)
}

fn part2<R: BufRead>(_reader: R) -> Result<i64> {
    Ok(0)
}

fn part1_result() -> Result<()> {
    run_on_day_input(day!(), part1)?;
    Ok(())
}

fn part2_result() -> Result<()> {
    run_on_day_input(day!(), part2)?;
    Ok(())
}

fn main() {
    part1_result().unwrap();
    part2_result().unwrap();
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    fn test_part1(expect: i64, input: &str) {
        assert_eq!(expect, part1(BufReader::new(input.as_bytes())).unwrap());
    }

    #[test]
    fn part1_example() {
        test_part1(
            0,
            indoc! {"
            "},
        );
    }

    #[test]
    fn part1_final() {
        part1_result().unwrap();
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    fn test_part2(expect: i64, input: &str) {
        assert_eq!(expect, part2(BufReader::new(input.as_bytes())).unwrap());
    }

    #[test]
    fn part2_example() {
        test_part2(0, indoc! {"
            1   2
        "});
    }

    #[test]
    fn part2_final() {
        part2_result().unwrap();
    }
}
