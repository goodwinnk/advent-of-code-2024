use std::collections::HashSet;
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};

#[derive(Debug)]
struct Input {
    patterns: Vec<String>,
    designs: Vec<String>,
}

fn parse_input<R: BufRead>(reader: R) -> Input {
    let mut patterns = Vec::new();
    let mut designs = Vec::new();
    let mut reading_patterns = true;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            reading_patterns = false;
            continue;
        }

        if reading_patterns {
            patterns.extend(
                line.split(", ").map(|s| s.trim().to_string())
            );
        } else {
            designs.push(line.trim().to_string());
        }
    }

    Input { patterns, designs }
}

fn can_make_pattern(target: &str, available_patterns: &[String], memo: &mut HashSet<String>) -> bool {
    if target.is_empty() {
        return true;
    }

    if memo.contains(target) {
        return false;
    }

    for pattern in available_patterns {
        if target.starts_with(pattern) {
            let remaining = &target[pattern.len()..];
            if can_make_pattern(remaining, available_patterns, memo) {
                return true;
            }
        }
    }

    memo.insert(target.to_string());
    false
}

fn count_possible_designs(input: &Input) -> usize {
    let mut count = 0;
    for design in &input.designs {
        let mut memo = HashSet::new();
        if can_make_pattern(design, &input.patterns, &mut memo) {
            count += 1;
        }
    }
    count
}

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let input = parse_input(reader);
    Ok(count_possible_designs(&input) as i64)
}

//noinspection DuplicatedCode
fn part2<R: BufRead>(_reader: R) -> Result<i64> {
    Ok(0)
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

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use indoc::indoc;
    use super::*;

    //noinspection SpellCheckingInspection
    #[cfg(test)]
    mod part1_tests {
        use super::*;

        fn test_part1(expect: i64, input: &str) {
            assert_eq!(expect, part1(BufReader::new(input.as_bytes())).unwrap());
        }

        #[test]
        fn test1() {
            test_part1(
                6,
                indoc! {"
                    r, wr, b, g, bwu, rb, gb, br

                    brwrr
                    bggr
                    gbbr
                    rrbgbr
                    ubwu
                    bwurrg
                    brgr
                    bbrgwb
                "},
            );
        }

        #[test]
        fn test_individual_patterns() {
            let test_cases = vec![
                ("brwrr", true),   // can be made with br + wr + r
                ("bggr", true),    // can be made with b + g + g + r
                ("gbbr", true),    // can be made with gb + br
                ("rrbgbr", true),  // can be made with r + rb + g + br
                ("ubwu", false),   // impossible
                ("bwurrg", true),  // can be made with bwu + r + r + g
                ("brgr", true),    // can be made with br + g + r
                ("bbrgwb", false), // impossible
            ];

            let patterns = vec![
                "r", "wr", "b", "g", "bwu", "rb", "gb", "br"
            ].into_iter().map(String::from).collect::<Vec<_>>();

            for (design, expected) in test_cases {
                let mut memo = HashSet::new();
                assert_eq!(
                    can_make_pattern(design, &patterns, &mut memo),
                    expected,
                    "Failed for design: {}",
                    design
                );
            }
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

        fn test_part2(expect: i64, input: &str) {
            assert_eq!(expect, part2(BufReader::new(input.as_bytes())).unwrap());
        }

        #[test]
        fn test1() {
            test_part2(
                0,
                indoc! {"
                1   2
            "}
            );
        }

        #[test]
        fn part2_final() {
            part2_result().unwrap();
        }
    }
}
