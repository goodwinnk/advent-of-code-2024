use std::collections::HashSet;
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use indoc::indoc;
use std::io::{BufRead, BufReader};

struct Input {
    rules: Vec<(i32, i32)>,
    updates: Vec<Vec<i32>>
}

fn parse_input<R: BufRead>(reader: R) -> Input {
    let lines: Vec<String> = reader.lines()
        .flatten()
        .collect();
    let mut split = lines.split(|line| line.is_empty());
    let rules_strings = split.next().unwrap();
    let updates_strings = split.next().unwrap();

    let rules: Vec<(i32, i32)> = rules_strings.iter().map(|line| {
        let mut split = line.split('|');
        let before = split.next().unwrap().parse::<i32>().unwrap();
        let after = split.next().unwrap().parse::<i32>().unwrap();
        assert_ne!(before, after);
        (before, after)
    }).collect();

    let updates: Vec<Vec<i32>> = updates_strings.iter()
        .map(|line| {
            let pages: Vec<i32> = line.split(',').map(|page_str| page_str.parse::<i32>().unwrap()).collect();
            pages
        }).collect();

    Input {
        rules,
        updates
    }
}

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let input = parse_input(reader);

    let working_rules = |rule: &Vec<i32>| -> Vec<(i32, i32)> {
        let rule_set: HashSet<i32> = HashSet::from_iter(rule.iter().cloned());
        input.rules.iter()
            .filter(|(before, after)| {
                rule_set.contains(before) && rule_set.contains(after)
            })
            .map(|(before, after)| (*before, *after))
            .collect()
    };

    let is_right_order = |rule: &Vec<i32>| -> bool {
        let mut working_rules = working_rules(rule);
        for page in rule {
            if working_rules.iter().any(|(before, after)| after == page) {
                return false;
            }
            working_rules.retain(|(before, _)| before != page);
        }
        true
    };

    Ok(input.updates.iter()
        .filter(|rule| is_right_order(rule))
        .map(|rule| {
            let len = rule.len();
            assert_eq!(len % 2, 1);
            rule[len / 2] as i64
        }).
        sum())
}

//noinspection DuplicatedCode
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
            143,
            indoc! {"
                47|53
                97|13
                97|61
                97|47
                75|29
                61|13
                75|53
                29|13
                97|29
                53|29
                61|53
                97|53
                61|29
                47|13
                75|47
                97|75
                47|61
                75|61
                47|29
                75|13
                53|13

                75,47,61,53,29
                97,61,53,29,13
                75,29,13
                75,97,47,61,53
                61,13,29
                97,13,75,29,47
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

    fn test_part2(expect: i64, input: &str) {
        assert_eq!(expect, part2(BufReader::new(input.as_bytes())).unwrap());
    }

    #[test]
    fn test1() {
        test_part2(0, indoc! {"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
        "});
    }

    #[test]
    fn part2_final() {
        part2_result().unwrap();
    }
}
