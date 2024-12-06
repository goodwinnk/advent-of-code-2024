use std::collections::HashSet;
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use indoc::indoc;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use linked_hash_set::LinkedHashSet;

struct Input {
    rules: Vec<(i32, i32)>,
    updates: Vec<LinkedHashSet<i32>>
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

    let updates: Vec<LinkedHashSet<i32>> = updates_strings.iter()
        .map(|line| {
            let pages: Vec<i32> = line.split(',').map(|page_str| page_str.parse::<i32>().unwrap()).collect();
            let mut pages_set = LinkedHashSet::new();
            pages_set.extend(pages.clone());
            pages_set
        }).collect();

    Input {
        rules,
        updates
    }
}

// Function to extract working rules from a given rule
fn get_working_rules(update: &LinkedHashSet<i32>, rules: &[(i32, i32)]) -> Vec<(i32, i32)> {
    rules.iter()
        .filter(|(before, after)| update.contains(before) && update.contains(after))
        .map(|(before, after)| (*before, *after))
        .collect()
}

// Function to check if the rules are in the correct order
fn is_in_correct_order(update: &LinkedHashSet<i32>, rules: &[(i32, i32)]) -> bool {
    let mut working_rules = get_working_rules(update, rules);
    for page in update {
        if working_rules.iter().any(|(_, after)| after == page) {
            return false;
        }
        working_rules.retain(|(before, _)| before != page);
    }
    true
}

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let input = parse_input(reader);

    Ok(input.updates.iter()
        .filter(|update| is_in_correct_order(update, &input.rules))
        .map(|update| {
            let len = update.len();
            assert_eq!(len % 2, 1);
            *(update.iter().clone().collect::<Vec<&i32>>()[len / 2]) as i64
        }).
        sum())
}

//noinspection DuplicatedCode
fn part2<R: BufRead>(reader: R) -> Result<i64> {
    let input = parse_input(reader);

    Ok(input.updates.iter()
        .filter(|update| !is_in_correct_order(update, &input.rules))
        .map(|update| {
            let len = update.len();
            assert_eq!(len % 2, 1);
            *(update.iter().clone().collect::<Vec<&i32>>()[len / 2]) as i64
        }).
        sum())
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
        test_part2(123, indoc! {"
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
