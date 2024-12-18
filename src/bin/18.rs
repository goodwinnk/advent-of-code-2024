use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};
use array2d::Array2D;
use advent_of_code2024_rust::matrix::{Array2DExt, Coordinate, Direction};

fn parse_input<R: BufRead>(reader: R) -> Vec<(usize, usize)> {
    reader
        .lines()
        .filter_map(|line| {
            line.ok().and_then(|line| {
                let mut parts = line.trim().split(',');
                let x = parts.next()?.parse().ok()?;
                let y = parts.next()?.parse().ok()?;
                Some((x, y))
            })
        })
        .collect()
}

fn create_maze(rows: usize, cols: usize, walls: &[(usize, usize)]) -> Array2D<char> {
    let mut maze = Array2D::filled_with('.', rows, cols);
    for &(x, y) in walls {
        maze.set(y, x, '#').unwrap(); // Note: y is row, x is column
    }

    maze
}

fn find_shortest_path(maze: &Array2D<char>) -> Option<i32> {
    let rows = maze.num_rows();
    let cols = maze.num_columns();
    let mut visited = Array2D::filled_with(false, rows, cols);
    let mut queue = VecDeque::new();

    queue.push_back((0, 0, 0)); // (row, col, distance)
    visited.set(0, 0, true).unwrap();

    // Possible moves: right, down, left, up
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((row, col, dist)) = queue.pop_front() {
        // If we reached the bottom-right corner
        if row == rows - 1 && col == cols - 1 {
            return Some(dist);
        }

        for (dx, dy) in directions.iter() {
            let new_row = row as i32 + dx;
            let new_col = col as i32 + dy;

            // Check if the new position is valid
            if new_row >= 0 && new_row < rows as i32 &&
                new_col >= 0 && new_col < cols as i32 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                // Check if the cell is unvisited and not a wall
                if !visited.get(new_row, new_col).unwrap() &&
                    *maze.get(new_row, new_col).unwrap() == '.' {
                    visited.set(new_row, new_col, true).unwrap();
                    queue.push_back((new_row, new_col, dist + 1));
                }
            }
        }
    }

    None // No path found
}

fn part1_full<R: BufRead>(reader: R, rows: usize, cols: usize, bytes_len: usize) -> Result<u64> {
    let bytes = parse_input(reader);

    let maze = create_maze(rows, cols, &bytes[..bytes_len]);

    Ok(find_shortest_path(&maze).unwrap() as u64)
}

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<u64> {
    part1_full(reader, 71, 71, 1024)
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

        fn test_part1(expect: u64, input: &str, rows: usize, cols: usize, bytes_len: usize) {
            assert_eq!(expect, part1_full(BufReader::new(input.as_bytes()), rows, cols, bytes_len).unwrap());
        }

        #[test]
        fn test1() {
            test_part1(
                22,
                indoc! {"
                    5,4
                    4,2
                    4,5
                    3,0
                    2,1
                    6,3
                    2,4
                    1,5
                    0,6
                    3,3
                    2,6
                    5,1
                    1,2
                    5,5
                    2,5
                    6,5
                    1,4
                    0,4
                    6,4
                    1,1
                    6,1
                    1,0
                    0,5
                    1,6
                    2,0
                "},
                7, 7, 12
            );
        }

        #[test]
        fn part1_final() {
            assert_eq!(0, run_on_day_input(day!(), part1).unwrap());
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
