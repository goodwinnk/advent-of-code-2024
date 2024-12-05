use anyhow::*;
use indoc::indoc;
use std::io::{BufRead, BufReader};
use advent_of_code2024_rust::{day, run_on_day_input};

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

fn part2_result() -> Result<()>  {
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
    #[test]
    fn part1_example() {
        const INPUT: &str = indoc! {"
        "};

        assert_eq!(0i64, part1(BufReader::new(INPUT.as_bytes())).unwrap());
    }

    #[test]
    fn part1_final() {
        part1_result().unwrap();
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn part2_example() {
        const INPUT: &str = indoc! {"
            1   2
        "};

        assert_eq!(0i64, part2(BufReader::new(INPUT.as_bytes())).unwrap());
    }

    #[test]
    fn part2_final() {
        part2_result().unwrap();
    }
}