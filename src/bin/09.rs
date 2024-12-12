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

fn check_sum(blocks: &Vec<Block>) -> i64 {
    let mut hash: i64 = 0;
    let mut index: i64 = 0;
    for block in blocks {
        if block.is_file {
            for _j in 0..block.size {
                hash = hash.checked_add(index * block.id).unwrap();
                index += 1;
            }
        } else {
            index += block.size;
        }
    }
    hash
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
    let mut compact_blocks: Vec<(i64, i64, bool)> = Vec::new();

    while next <= last_file_index {
        let (id, size, is_file) = map[next];
        if is_file {
            compact_blocks.push((id as i64, size as i64, true));
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
            compact_blocks.push((last_id as i64, fill_size as i64, true));
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

    Ok(check_sum(compact_blocks.iter()
        .copied()
        .map(|(id, size, is_file) | {Block { id, size, is_file }})
        .collect::<Vec<Block>>()
        .as_ref()
    ))
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Block {
    id: i64,
    size: i64,
    is_file: bool,
}

fn part2<R: BufRead>(reader: R) -> Result<i64> {
    let blocks: Vec<Block> = parse_input(reader).iter().enumerate()
        .map(|(i, &b)| {
            Block { id: (i / 2) as i64, size: b as i64, is_file: (i % 2) == 0 }
        })
        .collect();

    if blocks.is_empty() {
        return Ok(0);
    }

    let mut movable_blocks: Vec<Vec<Block>> = blocks.iter().map(|block| vec![block.clone()]).collect();
    for index in (4..movable_blocks.len()).rev() {
        let file_block = movable_blocks[index].first().unwrap().clone();
        if !file_block.is_file {
            continue
        }

        for free_space_index in 1..index {
            let free_space_block = movable_blocks[free_space_index].first().unwrap().clone();
            if free_space_block.is_file || free_space_block.size < file_block.size {
                continue
            }

            assert_eq!(movable_blocks[free_space_index].len(), 1);

            (&mut movable_blocks[free_space_index])[0] = Block {
                id: -free_space_block.id,
                size: free_space_block.size - file_block.size,
                is_file: false
            };

            movable_blocks[free_space_index - 1].push(file_block.clone());
            (&mut movable_blocks[index])[0] = Block {
                id: -file_block.id,
                size: file_block.size,
                is_file: false
            };

            break;
        }
    }

    let final_blocks: Vec<Block> = movable_blocks.iter().flatten().copied().collect();

    Ok(check_sum(final_blocks.as_ref()))
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
    mod utils_tests {
        use super::*;

        fn assert_checksum(expect: i64, blocks: &Vec<(i64, i64, bool)>) {
            assert_eq!(expect,
                check_sum(
                    blocks.iter()
                        .copied()
                        .map(|(id, size, is_file)| Block { id, size, is_file })
                        .collect::<Vec<Block>>()
                        .as_ref()
                )
            )
        }

        #[test]
        fn test1() {
            assert_checksum(2858, [
                // 00992111777.44.333....5555.6666.....8888..
                (0, 2, true),
                (9, 2, true),
                (2, 1, true),
                (1, 3, true),
                (7, 3, true),
                (0, 1, false),
                (4, 2, true),
                (0, 1, false),
                (3, 3, true),
                (0, 4, false),
                (5, 4, true),
                (0, 1, false),
                (6, 4, true),
                (0, 5, false),
                (8, 4, true),
                (0, 2, false)
            ].to_vec().as_ref());
        }

        #[test]
        fn test2() {
            assert_checksum(1928, [
                // 0099811188827773336446555566..............
                (0, 2, true),
                (9, 2, true),
                (8, 1, true),
                (1, 3, true),
                (8, 3, true),
                (2, 1, true),
                (7, 3, true),
                (3, 3, true),
                (6, 1, true),
                (4, 2, true),
                (6, 1, true),
                (5, 4, true),
                (6, 2, true),
                (50, 14, false)
            ].to_vec().as_ref());
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
                2858,
                indoc! {"
                    2333133121414131402
                "},
            );
        }

        #[test]
        fn test2() {
            test_part2(
                2,
                indoc! {"
                    111
                "},
            );
        }

        #[test]
        fn test3() {
            test_part2(16, "11122"); // 1.122
        }

        #[test]
        fn part2_final() {
            // Too high?
            let result = run_on_day_input(day!(), part2).unwrap();
            assert_eq!(6265268809555, result);
        }
    }
}
