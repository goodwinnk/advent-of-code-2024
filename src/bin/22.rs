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

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<u64> {
    Ok(reader.lines().flatten().map(
        |line| generate_nth_secret(line.parse().unwrap(), 2000)
    ).sum())
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
