use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use indoc::indoc;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn read_reports<R: BufRead>(reader: R) -> Result<Vec<Vec<i64>>> {
    let reports: Vec<Vec<i64>> = reader.lines()
        .flatten()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let level = line.split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            level
        })
        .collect();
    Ok(reports)
}

// Number of safe levels
fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let reports: Vec<Vec<i64>> = read_reports(reader)?;

    fn is_safe(report: &Vec<i64>) -> bool {
        if report.len() <= 1 { return true; }
        let sign = match report[1] - report[0] {
            0 => return false,
            diff if diff > 0 => 1,
            _ => -1,
        };
        for i in 1..report.len() {
            let diff = report[i] - report[i - 1];
            if diff * sign <= 0 { return false; }
            let abs = diff.abs();
            if !((abs >= 1) && (abs <= 3)) { return false; }
        }
        true
    }

    Ok(reports.iter().filter(|level| is_safe(level)).count())
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let reports: Vec<Vec<i64>> = read_reports(reader)?;

    fn unsafe_level_index(report: &Vec<i64>) -> i32 {
        if report.len() <= 1 { return -1; }
        let sign = match report[1] - report[0] {
            0 => return 1,
            diff if diff > 0 => 1,
            _ => -1,
        };
        for i in 1..report.len() {
            let diff = report[i] - report[i - 1];
            if diff * sign <= 0 { return i as i32; }
            let abs = diff.abs();
            if !((abs >= 1) && (abs <= 3)) { return i as i32; }
        }
        -1
    }

    fn is_safe_with_dumper(report: &Vec<i64>) -> bool {
        let unsafe_level = unsafe_level_index(report);
        if unsafe_level < 0 { return true; }

        fn remove_from_vector<T: Clone>(vec: &Vec<T>, index: usize) -> Vec<T> {
            let mut new_vec = vec.clone();
            new_vec.remove(index);
            new_vec
        }

        if unsafe_level_index(&remove_from_vector(report, unsafe_level as usize)) < 0 {
            return true;
        }

        if unsafe_level_index(&remove_from_vector(report, (unsafe_level - 1) as usize)) < 0 {
            return true;
        }

        if unsafe_level == 2 && unsafe_level_index(&remove_from_vector(report, (unsafe_level - 2) as usize)) < 0 {
            return true;
        }

        false
    }

    Ok(reports.iter().filter(|level| is_safe_with_dumper(level)).count())
}

//noinspection DuplicatedCode
fn part1_result() -> Result<()> {
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    Ok(())
}


//noinspection DuplicatedCode
fn part2_result() -> Result<()> {
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    Ok(())
}

fn main() {
    // part1_result().unwrap();
    // part2_result().unwrap();
}

#[cfg(test)]
mod part1_tests {
    use super::*;
    #[test]
    fn part1_example() {
        const INPUT: &str = indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "};

        assert_eq!(2, part1(BufReader::new(INPUT.as_bytes())).unwrap());
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
    fn part2_example1() {
        const INPUT: &str = indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "};

        assert_eq!(4, part2(BufReader::new(INPUT.as_bytes())).unwrap());
    }

    #[test]
    fn part2_example2() {
        const INPUT: &str = indoc! {"
            3 0 1 2
        "};

        assert_eq!(1, part2(BufReader::new(INPUT.as_bytes())).unwrap());
    }

    #[test]
    fn part2_final() {
        part2_result().unwrap();
    }
}