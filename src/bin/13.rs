use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};
use regex::Regex;

#[derive(Debug, Clone)]
struct ClawMachine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

fn parse_input<R: BufRead>(reader: R) -> Vec<ClawMachine> {
    let button_regex = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();

    reader
        .lines()
        .flatten()
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|chunk| {
            assert_eq!(chunk.len(), 3);

            let a_caps = button_regex.captures(&chunk[0])
                .ok_or_else(|| anyhow!("Invalid input for Button A"))?;
            let b_caps = button_regex.captures(&chunk[1])
                .ok_or_else(|| anyhow!("Invalid input for Button B"))?;
            let prize_caps = prize_regex.captures(&chunk[2])
                .ok_or_else(|| anyhow!("Invalid input for Prize"))?;

            let a_x = a_caps[1].parse::<i64>().unwrap();
            let a_y = a_caps[2].parse::<i64>().unwrap();
            let b_x = b_caps[1].parse::<i64>().unwrap();
            let b_y = b_caps[2].parse::<i64>().unwrap();
            let prize_x = prize_caps[1].parse::<i64>().unwrap();
            let prize_y = prize_caps[2].parse::<i64>().unwrap();

            Ok(ClawMachine {
                a_x, a_y,
                b_x, b_y,
                prize_x, prize_y,
            })
        })
        .collect::<Result<Vec<ClawMachine>>>()
        .unwrap()
}

fn min_cost_to_win(machine: &ClawMachine) -> Option<i64> {
    let a_top = machine.prize_y * machine.b_x - machine.prize_x * machine.b_y;
    let a_bottom = machine.a_y * machine.b_x - machine.a_x * machine.b_y;

    if a_bottom == 0 {
        let a: Option<i64> = if machine.prize_y % machine.a_y == 0 && machine.prize_x % machine.a_x == 0 &&
            machine.prize_y / machine.a_y == machine.prize_x / machine.a_x {
            Some(machine.prize_y / machine.a_y)
        } else {
            None
        };
        let b: Option<i64> = if machine.prize_y % machine.b_y == 0 && machine.prize_x % machine.b_x == 0 &&
            machine.prize_y / machine.b_y == machine.prize_x / machine.b_x {
            Some(machine.prize_y / machine.b_y)
        } else {
            None
        };

        return [a.and_then(|x| Some(x * 3)), b].iter().filter_map(|&x| x).min();
    }

    if a_top % a_bottom != 0 {
        return None;
    }

    let a = a_top / a_bottom;
    if a < 0 {
        return None;
    }

    let b_top = machine.prize_x - a * machine.a_x;
    let b_bottom = machine.b_x;

    if (b_top % b_bottom) != 0 {
        return None;
    }
    let b = b_top / b_bottom;
    if b < 0 {
        return None;
    }

    Some(a * 3 + b)
}

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let machines = parse_input(reader);

    let result: i64 = machines
        .iter()
        .filter_map(|machine| min_cost_to_win(machine))
        .sum();

    Ok(result)
}

//noinspection DuplicatedCode
fn part2<R: BufRead>(reader: R) -> Result<i64> {
    let machines = parse_input(reader);

    let result: i64 = machines
        .iter()
        .filter_map(|machine| min_cost_to_win(&ClawMachine {
            a_x: machine.a_x,
            a_y: machine.a_y,
            b_x: machine.b_x,
            b_y: machine.b_y,
            prize_x: machine.prize_x + 10000000000000,
            prize_y: machine.prize_y + 10000000000000,
        }))
        .sum();

    Ok(result)
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

    #[cfg(test)]
    mod min_cost_to_win_tests {
        use super::*;

        fn test_min_cost_to_win(expect: Option<i64>, machine: &ClawMachine) {
            assert_eq!(expect, min_cost_to_win(machine));
        }

        #[test]
        fn test1() {
            test_min_cost_to_win(Some(2), &ClawMachine {
                a_x: 1, a_y: 1,
                b_x: 2, b_y: 2,
                prize_x: 4, prize_y: 4,
            });
        }

        #[test]
        fn test2() {
            test_min_cost_to_win(Some(6), &ClawMachine {
                a_x: 4, a_y: 4,
                b_x: 1, b_y: 1,
                prize_x: 8, prize_y: 8,
            });
        }

        #[test]
        fn test3() {
            test_min_cost_to_win(None, &ClawMachine {
                a_x: 2, a_y: 3,
                b_x: 4, b_y: 6,
                prize_x: 9, prize_y: 9,
            });
        }

        #[test]
        fn test4() {
            test_min_cost_to_win(Some(2), &ClawMachine {
                a_x: 12, a_y: 12,
                b_x: 4, b_y: 4,
                prize_x: 8, prize_y: 8,
            });
        }

        #[test]
        fn test5() {
            test_min_cost_to_win(Some(1), &ClawMachine {
                a_x: 8, a_y: 8,
                b_x: 8, b_y: 8,
                prize_x: 8, prize_y: 8,
            });
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
            test_part1(
                480,
                indoc! {"
                    Button A: X+94, Y+34
                    Button B: X+22, Y+67
                    Prize: X=8400, Y=5400

                    Button A: X+26, Y+66
                    Button B: X+67, Y+21
                    Prize: X=12748, Y=12176

                    Button A: X+17, Y+86
                    Button B: X+84, Y+37
                    Prize: X=7870, Y=6450

                    Button A: X+69, Y+23
                    Button B: X+27, Y+71
                    Prize: X=18641, Y=10279
                "},
            );
        }

        #[test]
        fn part1_final() {
            assert_eq!(26005, run_on_day_input(day!(), part1).unwrap());
        }
    }

    //noinspection SpellCheckingInspection
    #[cfg(test)]
    mod part2_tests {
        use super::*;

        #[test]
        fn part2_final() {
            assert_eq!(105620095782547, run_on_day_input(day!(), part2).unwrap());
        }
    }
}
