use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::collections::HashMap;
use std::io::BufRead;

fn transform_stone(stone: usize) -> Vec<usize> {
    // Rule 1: If stone is 0, replace with 1
    if stone == 0 {
        return vec![1];
    }

    // Rule 2: If stone has even number of digits, split into two stones
    let stone_str = stone.to_string();
    if stone_str.len() % 2 == 0 {
        let mid = stone_str.len() / 2;
        let left: usize = stone_str[..mid].parse().unwrap();
        let right: usize = stone_str[mid..].parse().unwrap();
        return vec![left, right];
    }

    // Rule 3: Multiply stone by 2024
    vec![stone * 2024]
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Default)]
struct Task {
    stone: usize,
    blinks: usize,
}

fn simulate_blinks(task: Task, cache: &mut HashMap<Task, usize>) -> usize {
    if task.blinks == 0 {
        return 1;
    }

    if let Some(cached) = cache.get(&task) {
        return *cached;
    }

    let result: usize = transform_stone(task.stone).iter()
        .map(|stone| simulate_blinks(Task { stone: *stone, blinks: task.blinks - 1 }, cache))
        .sum();

    cache.insert(task, result);
    result
}

fn read_input<R: BufRead>(reader: R) -> Result<Vec<usize>> {
    let lines: Vec<String> = reader
        .lines()
        .flatten()
        .filter(|line| !line.is_empty())
        .collect();

    if lines.len() != 1 {
        return Err(anyhow!("Expected 1 line, got {}", lines.len()));
    }

    Ok(lines[0].split_whitespace().map(
        |stone| stone.parse().unwrap()
    ).collect())
}

fn blink_over_stones(stones: Vec<usize>, blinks: usize) -> usize {
    let mut cache = HashMap::new();
    stones.iter().map(
        |stone| simulate_blinks(Task { stone: *stone, blinks }, &mut cache)
    ).sum()
}

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let stones = read_input(reader)?;
    Ok(blink_over_stones(stones, 25) as i64)
}

//noinspection DuplicatedCode
fn part2<R: BufRead>(reader: R) -> Result<i64> {
    let stones = read_input(reader)?;
    Ok(blink_over_stones(stones, 75) as i64)
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
    use super::*;
    use std::io::BufReader;

    #[cfg(test)]
    mod blink_tests {
        use super::*;

        fn blinks_test(stones: Vec<usize>, expect: Vec<usize>, blinks: usize) {
            assert_eq!(expect.iter().len(), blink_over_stones(stones, blinks));
        }

        #[test]
        fn test1() {
            blinks_test(
                [125, 17].to_vec(),
                [253000, 1, 7].to_vec(),
                1
            )
        }

        #[test]
        fn test2() {
            blinks_test(
                [253000, 1, 7].to_vec(),
                [253, 0, 2024, 14168].to_vec(),
                1
            )
        }

        #[test]
        fn test3() {
            blinks_test(
                [253, 0, 2024, 14168].to_vec(),
                [512072, 1, 20, 24, 28676032].to_vec(),
                1
            )
        }

        #[test]
        fn test4() {
            blinks_test(
                [512072, 1, 20, 24, 28676032].to_vec(),
                [512, 72, 2024, 2, 0, 2, 4, 2867, 6032].to_vec(),
                1
            )
        }
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
            test_part1(55312, "125 17");
        }

        #[test]
        fn part1_final() {
            assert_eq!(193607, run_on_day_input(day!(), part1).unwrap());
        }
    }

    //noinspection SpellCheckingInspection
    #[cfg(test)]
    mod part2_tests {
        use super::*;

        #[test]
        fn part2_final() {
            assert_eq!(229557103025807, run_on_day_input(day!(), part2).unwrap());
        }
   }
}
