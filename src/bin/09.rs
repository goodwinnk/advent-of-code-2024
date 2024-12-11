use std::cmp::min;
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};
use priority_queue::PriorityQueue;

fn parse_input<R: BufRead>(reader: R) -> Vec<usize> {
    let str: String = reader.lines().next().unwrap().unwrap();
    
    str.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn check_sum(blocks: &Vec<Block>) -> i64 {
    blocks.iter().fold((0i64, 0i64), |(index, sum), block: &Block| {
        let block_size = block.size;
        if block.is_file {
            let block_id = block.id;
            let sum_dx: i64 = (index + (block_size - 1) + index) * block_size / 2 * block_id;
            (index + block_size, sum + sum_dx)
        } else {
            (index + block_size, sum)
        }
    }).1
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

//noinspection DuplicatedCode
fn part2<R: BufRead>(reader: R) -> Result<i64> {
    let blocks: Vec<Block> = parse_input(reader).iter().enumerate()
        .map(|(i, &b)| {
            Block { id: (i / 2) as i64, size: b as i64, is_file: (i % 2) == 0 }
        })
        .collect();
    if blocks.is_empty() {
        return Ok(0);
    }

    let blocks_with_indexes: Vec<(Block, i64)> = blocks.iter()
        .scan(0i64, |index, &block| {
            let current_index = *index;
            *index += block.size;
            Some((block, current_index))
        })
        .collect();

    let mut queues: [PriorityQueue<usize, i64>; 10] = Default::default();
    let mut free_spaces: Vec<Vec<Block>> = blocks.iter()
        .filter(|block| !block.is_file)
        .inspect(|block| {
            let queue = &mut queues[block.size as usize];
            queue.push(block.id as usize, -block.id);
        })
        .map(|&block| vec![block])
        .collect();

    if (blocks.len() % 2) == 0 {
        let last_free_spase = blocks.last().unwrap();
        assert!(!last_free_spase.is_file, "free space expected");
        (&mut queues[last_free_spase.size as usize]).remove(&(last_free_spase.id as usize));
    }

    let files: Vec<Block> = blocks.iter()
        .filter(|block| block.is_file)
        .map(|&block| block)
        .collect();

    let mut resulting_file_blocks_inv: Vec<Block> = Default::default();

    for (file_block_index, file_block) in files.iter().enumerate().rev() {
        let size = file_block.size as usize;

        if file_block_index == 0 {
            resulting_file_blocks_inv.push(file_block.clone());
            break;
        }

        {
            // 0.0 - 1.f - 2.1 - 3.f - 4.2
            // Remove the block before the file from the queue
            let last_free_spase_block = blocks[file_block_index * 2 - 1];
            assert!(!last_free_spase_block.is_file, "free space expected");
            (&mut queues[last_free_spase_block.size as usize]).remove(&(last_free_spase_block.id as usize));
        }

        // id of the queue (size of free space) - id of the free space
        let min_queue_index: Option<(usize, usize)> = (size .. 9)
            .map(|i| (i, queues[i].peek().and_then(|(&free_space_id, _)| Some(free_space_id))))
            .filter(|(_, queue_top)| queue_top.is_some())
            .map(|(i, queue_top)| (i, queue_top.unwrap()))
            .min_by_key(|(_, free_space_id)| *free_space_id);

        if let Some((queue_id, free_space_id)) = min_queue_index {
            let free_space_vec = &mut free_spaces[free_space_id];
            let free_space_block = free_space_vec.remove(free_space_vec.len() - 1);
            assert!(!free_space_block.is_file);
            assert!(free_space_block.size as usize >= size);

            (&mut queues[queue_id]).pop().inspect(|(space_id, priority)| {
                assert_eq!(*space_id, free_space_id);
                assert_eq!(*priority, -free_space_block.id);
            });

            // Move file
            free_space_vec.push(file_block.clone());
            resulting_file_blocks_inv.push( Block {
                    id: -file_block.id,
                    size: file_block.size,
                    is_file: false
                }
            );

            // Update the rest of free space
            let updated_free_space_block = Block {
                id: free_space_block.id,
                size: free_space_block.size - file_block.size,
                is_file: free_space_block.is_file
            };
            if updated_free_space_block.size != 0 {
                free_space_vec.push(updated_free_space_block);
                let queue = &mut queues[updated_free_space_block.size as usize];
                queue.push(updated_free_space_block.id as usize, -updated_free_space_block.id);
            }
        } else {
            resulting_file_blocks_inv.push(file_block.clone());
        }
    }

    let resulting_file_blocks = resulting_file_blocks_inv.iter().rev().copied().collect::<Vec<Block>>();
    assert_eq!(resulting_file_blocks.len(), files.len());

    let final_blocks = (0..resulting_file_blocks.len()).map(|i| {
        let mut blocks = vec![resulting_file_blocks[i]];
        if let Some(free_spaces_blocks) = free_spaces.get(i) {
            blocks.extend(free_spaces_blocks.iter().copied());
        }

        blocks
    }).flatten().collect::<Vec<Block>>();

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
        fn part2_final() {
            part2_result().unwrap();
        }
    }
}
