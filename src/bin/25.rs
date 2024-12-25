use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};

#[derive(Debug, PartialEq)]
struct Lock {
    heights: Vec<usize>,
}

#[derive(Debug, PartialEq)]
struct Key {
    heights: Vec<usize>,
}

fn parse_input<R: BufRead>(reader: R) -> (Vec<Lock>, Vec<Key>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let groups = reader.lines()
        .flatten()
        .collect::<Vec<String>>();

    for line_group in groups.split(|line| line.is_empty()) {
        if let Some(lock) = parse_lock(line_group) {
            locks.push(lock);
        } else if let Some(key) = parse_key(line_group) {
            keys.push(key);
        } else {
            panic!("Invalid input {:?}", line_group);
        }
    }

    (locks, keys)
}

fn parse_lock(grid: &[String]) -> Option<Lock> {
    if grid.is_empty() || grid[0].chars().any(|c| c != '#') {
        return None;
    }

    let height = grid.len();
    let width = grid[0].len();

    let mut heights = Vec::with_capacity(width);
    for col in 0..width {
        let non_hash_index = (0..height)
            .find(|&row| { grid[row].chars().nth(col).unwrap() != '#' })?;
        heights.push(non_hash_index - 1);
    }

    Some(Lock { heights })
}

fn parse_key(grid: &[String]) -> Option<Key> {
    if grid.is_empty() || grid[0].chars().any(|c| c != '.') {
        return None;
    }

    let height = grid.len();
    let width = grid[0].len();

    let mut heights = Vec::with_capacity(width);
    for col in 0..width {
        let first_hash_index = (0..height)
            .find(|&row| grid[row].chars().nth(col) == Some('#'))?;
        heights.push(height - 1 - first_hash_index);
    }

    Some(Key { heights })
}


//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let (locks, keys) = parse_input(reader);
    Ok(
        locks.iter()
            .map(|lock| {
                keys.iter().filter(|key| {
                    lock.heights.iter()
                        .zip(key.heights.iter())
                        .all(|(lock_height, key_height)| {
                            lock_height + key_height <= 5
                        })
                }).count() as i64
            })
            .sum()
    )
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

    #[test]
    fn test_parse_input() {
        let input = indoc! {"
            #####
            .####
            .####
            .####
            .#.#.
            .#...
            .....

            .....
            #....
            #....
            #...#
            #.#.#
            #.###
            #####
        "};

        let (locks, keys) = parse_input(BufReader::new(input.as_bytes()));
        assert_eq!(locks, vec![
            Lock { heights: vec![0, 5, 3, 4, 3] }
        ]);
        assert_eq!(keys, vec![
            Key { heights: vec![5, 0, 2, 1, 3] }
        ]);
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
                3,
                indoc! {"
                    #####
                    .####
                    .####
                    .####
                    .#.#.
                    .#...
                    .....

                    #####
                    ##.##
                    .#.##
                    ...##
                    ...#.
                    ...#.
                    .....

                    .....
                    #....
                    #....
                    #...#
                    #.#.#
                    #.###
                    #####

                    .....
                    .....
                    #.#..
                    ###..
                    ###.#
                    ###.#
                    #####

                    .....
                    .....
                    .....
                    #....
                    #.#..
                    #.#.#
                    #####
                "},
            );
        }

        #[test]
        fn part1_final() {
            assert_eq!(3127, run_on_day_input(day!(), part1).unwrap());
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
