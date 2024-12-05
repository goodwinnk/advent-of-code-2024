use anyhow::*;
use indoc::indoc;
use std::io::{BufRead, BufReader};
use advent_of_code2024_rust::{day, run_on_day_input};

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let lines: Vec<Vec<char>> = reader.lines()
        .flatten()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    if lines.is_empty() { return Ok(0) }

    let y_range = 0 .. lines.len() as i32;
    let x_range = 0 .. lines[0].len() as i32;

    let word: Vec<char> = "XMAS".chars().collect();

    let count_word = |y: i32, x: i32| -> i32 {
        let mut matrix: [[bool; 3]; 3] = [
            [true, true, true],
            [true, false, true],
            [true, true, true]
        ];

        let mut letter = 1;
        let mut true_count = 8;

        while true_count > 0 && letter < word.len() {
            true_count = 0;
            for i in 0..3 {
                for j in 0..3 {
                    if matrix[i][j] {
                        let n_x = x + (j as i32 - 1) * letter as i32;
                        let n_y = y + (i as i32 - 1) * letter as i32;
                        if x_range.contains(&n_x) && y_range.contains(&n_y) && lines[n_y as usize][n_x as usize] == word[letter] {
                            true_count += 1;
                        } else {
                            matrix[i][j] = false;
                        }
                    }
                }
            }
            letter += 1;
        }

        true_count
    };

    let mut result = 0;
    for i in 0..lines.len() {
        let line = &lines[i];
        for j in 0..line.len() {
            if line[j] == word[0] {
                result += count_word(i as i32, j as i32);
            }
        }
    }

    Ok(result as i64)
}

//noinspection DuplicatedCode
fn part2<R: BufRead>(reader: R) -> Result<i64> {
    let lines: Vec<Vec<char>> = reader.lines()
        .flatten()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    if lines.is_empty() { return Ok(0) }

    let is_x_mas = |y: usize, x: usize| -> bool {
        fn check_ms(c1: char, c2: char) -> bool {
            (c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M')
        }

        lines[y][x] == 'A' &&
            check_ms(lines[y - 1][x - 1], lines[y + 1][x + 1]) &&
            check_ms(lines[y - 1][x + 1], lines[y + 1][x - 1])
    };

    let mut result = 0;
    for y in 1..lines.len() - 1 {
        for x in 1..(lines[y].len() - 1) {
            if lines[y][x] == 'A' && is_x_mas(y, x) {
                result += 1;
            }
        }
    }

    Ok(result as i64)
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

//noinspection SpellCheckingInspection
#[cfg(test)]
mod part1_tests {
    use super::*;

    fn test_part1(expect: i64, input: &str) {
        assert_eq!(expect, part1(BufReader::new(input.as_bytes())).unwrap());
    }

    #[test]
    fn part1_0() {
        test_part1(4, indoc! {"
            ..X...
            .SAMX.
            .A..A.
            XMAS.S
            .X....
        "});
    }

    #[test]
    fn part1_1() {
        test_part1(18, indoc! {"
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        "});
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
    fn test_1() {
        test_part2(1, indoc! {"
            M.S
            .A.
            M.S
        "});
    }

    #[test]
    fn test_2() {
        test_part2(9, indoc! {"
            .M.S......
            ..A..MSMS.
            .M.S.MAA..
            ..A.ASMSM.
            .M.S.M....
            ..........
            S.S.S.S.S.
            .A.A.A.A..
            M.M.M.M.M.
            ..........
        "});
    }

    #[test]
    fn test_3() {
        test_part2(9, indoc! {"
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        "});
    }

    #[test]
    fn part2_final() {
        part2_result().unwrap();
    }
}
