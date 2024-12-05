use anyhow::*;
use indoc::indoc;
use std::io::{BufRead, BufReader};
use regex::Regex;
use advent_of_code2024_rust::{day, run_on_day_input};

fn part1<R: BufRead>(mut reader: R) -> Result<i64> {
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    let r = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")?;
    let result = r.captures_iter(&content)
        .map(|c| {
            let a = c[1].parse::<i64>().unwrap();
            let b = c[2].parse::<i64>().unwrap();
            a * b
        })
        .sum();

    Ok(result)
}

fn part2<R: BufRead>(mut reader: R) -> Result<i64> {
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    let r = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\)")?;

    let mut state: bool = true;
    let result = r.captures_iter(&content)
        .map(|c| {
            match &c[0] {
                "do()" => {
                    state = true;
                    0
                },
                "don't()" => {
                    state = false;
                    0
                },
                _ => {
                    if state {
                        let a = c[1].parse::<i64>().unwrap();
                        let b = c[2].parse::<i64>().unwrap();
                        a * b
                    } else {
                        0
                    }
                }
            }
        })
        .sum();

    Ok(result)
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
    fn part1_1() {
        const INPUT: &str = indoc! {"
            xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
        "};

        assert_eq!(161, part1(BufReader::new(INPUT.as_bytes())).unwrap());
    }

    #[test]
    fn part1_2() {
        const INPUT: &str = indoc! {"
            xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+
            mul(32,64]then(mul(11,8)mul(8,5))
        "};

        assert_eq!(161, part1(BufReader::new(INPUT.as_bytes())).unwrap());
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
    fn part2_1() {
        const INPUT: &str = indoc! {"
            xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
        "};

        assert_eq!(48, part2(BufReader::new(INPUT.as_bytes())).unwrap());
    }

    #[test]
    fn part2_2() {
        const INPUT: &str = indoc! {"
            don't()_do()_don't()_mul(2,4)
        "};

        assert_eq!(0, part2(BufReader::new(INPUT.as_bytes())).unwrap());
    }

    #[test]
    fn part2_3() {
        const INPUT: &str = indoc! {"
            don't()_do()_don't()_do()mul(2,4)_don't()
        "};

        assert_eq!(8, part2(BufReader::new(INPUT.as_bytes())).unwrap());
    }

    #[test]
    fn part2_final() {
        part2_result().unwrap();
    }
}