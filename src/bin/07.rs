use core::result::Result::Ok;
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::BufRead;
use std::num::ParseIntError;

fn can_be_true_sum_mul(result: i64, numbers: &Vec<i64>, len: usize) -> bool {
    if len == 0 {
        return result == 0;
    }

    if result <= 0 {
        return false;
    }

    let last = numbers[len - 1];
    if result % last == 0 {
        if can_be_true_sum_mul(result / last, numbers, len - 1) {
            return true;
        }
    }

    can_be_true_sum_mul(result - last, numbers, len - 1)
}

fn concatenate(left: i64, right: i64) -> std::result::Result<i64, ParseIntError> {
    (left.to_string() + &right.to_string()).parse::<i64>()
}

fn can_be_true_sum_mul_concatenation(result: i64, numbers: &Vec<i64>, len: usize) -> bool {
    if len == 0 {
        return result == 0;
    }

    if result <= 0 {
        return false;
    }

    let last = numbers[len - 1];
    if result % last == 0 {
        if can_be_true_sum_mul_concatenation(result / last, numbers, len - 1) {
            return true;
        }
    }

    if can_be_true_sum_mul_concatenation(result - last, numbers, len - 1) {
        return true;
    }

    if len < 2 {
        return false;
    }

    let result_str = result.to_string();
    let last_str = last.to_string();
    if let Some(prefix) = result_str.strip_suffix(&last_str) {
        if !prefix.is_empty() {
            can_be_true_sum_mul_concatenation(prefix.parse::<i64>().unwrap(), numbers, len - 1)
        } else {
            false
        }
    } else {
        false
    }
}

fn read_input<R: BufRead>(reader: R) -> Result<Vec<(i64, Vec<i64>)>> {
    let equations: Vec<(i64, Vec<i64>)> = reader
        .lines()
        .flatten()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let result_rest: Vec<&str> = line.split(": ").collect();
            let result = result_rest[0].parse::<i64>().expect("Can't parse result");
            let rest = result_rest[1]
                .split_whitespace()
                .map(|s| s.parse::<i64>().expect("Can't parse number"))
                .collect();
            (result, rest)
        })
        .collect();
    Ok(equations)
}

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let equations: Vec<(i64, Vec<i64>)> = read_input(reader)?;

    Ok(equations
        .iter()
        .filter(|(result, numbers)| can_be_true_sum_mul(result.clone(), numbers, numbers.len()))
        .map(|(result, _)| result)
        .sum())
}

//noinspection DuplicatedCode
fn part2<R: BufRead>(reader: R) -> Result<i64> {
    let equations: Vec<(i64, Vec<i64>)> = read_input(reader)?;
    Ok(equations
        .iter()
        .filter(|(result, numbers)| can_be_true_sum_mul_concatenation(result.clone(), numbers, numbers.len()))
        .map(|(result, _)| result)
        .sum())
}

//#region

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

//#endregion

//noinspection SpellCheckingInspection
#[cfg(test)]
mod part1_tests {
    use super::*;
    use indoc::indoc;
    use std::io::BufReader;

    fn test_part1(expect: i64, input: &str) {
        assert_eq!(expect, part1(BufReader::new(input.as_bytes())).unwrap());
    }

    #[test]
    fn test1() {
        test_part1(
            3749,
            indoc! {"
                190: 10 19
                3267: 81 40 27
                83: 17 5
                156: 15 6
                7290: 6 8 6 15
                161011: 16 10 13
                192: 17 8 14
                21037: 9 7 18 13
                292: 11 6 16 20
            "},
        );
    }

    #[test]
    fn part1_final() {
        part1_result().unwrap();
    }
}

//noinspection SpellCheckingInspection
#[cfg(test)]
mod part2_tests {
    use super::*;
    use indoc::indoc;
    use std::io::BufReader;

    fn test_part2(expect: i64, input: &str) {
        assert_eq!(expect, part2(BufReader::new(input.as_bytes())).unwrap());
    }

    #[test]
    fn test1() {
        test_part2(
            11387,
            indoc! {"
                190: 10 19
                3267: 81 40 27
                83: 17 5
                156: 15 6
                7290: 6 8 6 15
                161011: 16 10 13
                192: 17 8 14
                21037: 9 7 18 13
                292: 11 6 16 20
            "},
        );
    }

    #[test]
    fn part2_final() {
        part2_result().unwrap();
    }
}
