use std::cmp::min;
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};

fn parse_input<R: BufRead>(reader: R) -> Vec<usize> {
    let str: String = reader.lines().next().unwrap().unwrap();
    
    str.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let blocks = parse_input(reader);
    if blocks.is_empty() {
        return Ok(0);
    }

    // true - file, false - free spase
    let mut map: Vec<(usize, usize, bool)> =
        blocks.iter().enumerate().map(|(i, &b)| (i / 2, b, i % 2 == 0)).collect();

    let mut last_file_index = if (map.len() % 2) == 0 { map.len() - 2 } else { map.len() - 1 };
    let mut next = 0;
    let mut compact_blocks: Vec<(i64, i64)> = Vec::new();

    while next <= last_file_index {
        let (id, size, is_file) = map[next];
        if is_file {
            compact_blocks.push((id as i64, size as i64));
            next += 1;
            continue;
        }

        let (last_id, last_size, last_is_file) = map[last_file_index];
        assert!(last_is_file,
                "next(index: {}, id: {}, size: {}), last(index: {}, id: {}, size:{})",
                next, id, size, last_file_index, last_id, last_size
        );

        let fill_size = min(size, last_size);
        if fill_size != 0 {
            compact_blocks.push((last_id as i64, fill_size as i64));
        }

        if fill_size == size {
            next += 1;
        } else {
            map[next] = (id, size - fill_size, false)
        }

        if fill_size == last_size {
            last_file_index -= 2;
        } else {
            map[last_file_index] = (last_id, last_size - fill_size, true)
        }
    }

    let (_, sum) = compact_blocks.iter().fold((0i64, 0i64), |(index, sum), (id, size)| {
        let sum_dx : i64 = (index + (size - 1) + index) * size / 2 * id;
        (index + size, sum + sum_dx)
    });

    Ok(sum)
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
                1928,
                indoc! {"
                    2333133121414131402
                "},
            );
        }

        #[test]
        fn part1_final() {
            assert_eq!(6241633730082, run_on_day_input(day!(), part1).unwrap());
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
