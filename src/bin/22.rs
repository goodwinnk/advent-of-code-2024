use std::collections::{HashMap, HashSet, VecDeque};
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};

fn mix(secret: u64, value: u64) -> u64 {
    secret ^ value
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn calculate_next_secret(mut secret: u64) -> u64 {
    // Step 1: Multiply by 64
    let result = secret * 64;
    secret = mix(secret, result);
    secret = prune(secret);

    // Step 2: Divide by 32
    let result = secret / 32;
    secret = mix(secret, result);
    secret = prune(secret);

    // Step 3: Multiply by 2048
    let result = secret * 2048;
    secret = mix(secret, result);
    secret = prune(secret);

    secret
}
fn generate_nth_secret(initial: u64, n: usize) -> u64 {
    let mut secret = initial;
    for _ in 0..n {
        secret = calculate_next_secret(secret);
    }
    secret
}

fn parse_input<R: BufRead>(reader: R) -> Vec<u64> {
    reader.lines()
        .flatten()
        .map(|line| line.parse().unwrap())
        .collect()
}

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<u64> {
    Ok(parse_input(reader).iter().map(|initial| generate_nth_secret(*initial, 2000)).sum())
}

fn process_buyer(initial_secret: u64, n: usize, sequence_prices: &mut HashMap<Vec<i32>, u64>) {
    let size = 4;

    let mut secret = initial_secret;
    let mut seen_sequences = HashSet::new();
    let mut changes = VecDeque::with_capacity(size);

    let mut prev_price = secret % 10;

    for i in 1..=n {
        secret = calculate_next_secret(secret);
        let current_price = secret % 10;
        if i > size {
            changes.pop_front();
        }
        changes.push_back(current_price as i32 - prev_price as i32);
        prev_price = current_price;

        if changes.len() < size {
            continue;
        }

        let sequence: Vec<i32> = changes.iter().copied().collect();
        if seen_sequences.insert(sequence.clone()) {
            sequence_prices.entry(sequence)
                .and_modify(|total| *total += current_price)
                .or_insert(current_price);
        }
    }
}

//noinspection DuplicatedCode
fn part2<R: BufRead>(reader: R) -> Result<u64> {
    let initial_secrets = parse_input(reader);
    let mut sequence_prices: HashMap<Vec<i32>, u64> = HashMap::new();
    for &secret in &initial_secrets {
        process_buyer(secret, 2000, &mut sequence_prices);
    }

    Ok(*sequence_prices
        .iter()
        .max_by_key(|&(_, price)| price)
        .unwrap().1)
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

        #[test]
        fn test_sequence_for_initial_123() {
            let mut secret = 123;
            let expected = vec![
                15887950, 16495136, 527345, 704524, 1553684,
                12683156, 11100544, 12249484, 7753432, 5908254
            ];

            for &expected_value in expected.iter() {
                secret = calculate_next_secret(secret);
                assert_eq!(secret, expected_value);
            }
        }

        #[test]
        fn test_example_input() {
            let expected_values = vec![
                (1, 8685429),
                (10, 4700978),
                (100, 15273692),
                (2024, 8667524)
            ];

            for (initial, expected) in expected_values {
                assert_eq!(generate_nth_secret(initial, 2000), expected);
            }
        }

        fn test_part1(expect: u64, input: &str) {
            assert_eq!(expect, part1(BufReader::new(input.as_bytes())).unwrap());
        }

        #[test]
        fn test1() {
            test_part1(
                37327623,
                indoc! {"
                    1
                    10
                    100
                    2024
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

        fn test_part2(expect: u64, input: &str) {
            assert_eq!(expect, part2(BufReader::new(input.as_bytes())).unwrap());
        }

        #[test]
        fn test1() {
            test_part2(
                24,
                indoc! {"
                    1
                    10
                    100
                    2024
                "},
            );
        }

        #[test]
        fn part2_final() {
            fn part2_final() {
                assert_eq!(2272, run_on_day_input(day!(), part2).unwrap());
            }
        }
    }
}
