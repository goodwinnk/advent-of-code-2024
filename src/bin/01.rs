use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use indoc::indoc;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let input_pairs: Vec<(i64, i64)> = reader.lines()
        .flatten()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let numbers: Vec<i64> = line.split("   ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            (numbers[0], numbers[1])
        })
        .collect();

    let first = input_pairs.iter().map(|(a, _b)| a).sorted();
    let second = input_pairs.iter().map(|(_a, b)| b).sorted();

    let mut total_distance: i64 = 0;
    for (f, s) in first.zip(second) {
        total_distance += (f - s).abs();
    };

    Ok(total_distance)
}

fn part2<R: BufRead>(reader: R) -> Result<i64> {
    let count = reader.lines().count();
    Ok(count as i64)
}

fn part1_result() -> Result<()> {
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    Ok(())
}

fn part2_result() -> Result<()> {
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
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
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "};

        assert_eq!(11i64, part1(BufReader::new(INPUT.as_bytes())).unwrap());
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

        assert_eq!(1i64, part2(BufReader::new(INPUT.as_bytes())).unwrap());
    }

    #[test]
    fn part2_final() {
        part2_result().unwrap();
    }
}