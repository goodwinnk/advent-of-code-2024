use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};
use array2d::Array2D;
use advent_of_code2024_rust::matrix::{Array2DExt, Coordinate, Direction};

fn parse_input<R: BufRead>(reader: R) -> Array2D<char> {
    let rows: Vec<Vec<char>> = reader
        .lines()
        .flatten()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().chars().collect())
        .collect();
    
    Array2D::from_rows(&rows).unwrap()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct State {
    position: Coordinate,
    direction: Direction,
    cost: u64
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_position(maze: &Array2D<char>, target: char) -> Coordinate {
    for (y, row) in maze.rows_iter().enumerate() {
        for (x, &cell) in row.enumerate() {
            if cell == target {
                return Coordinate {
                    row: y as isize,
                    column: x as isize,
                };
            }
        }
    }

    panic!("Position not found!")
}

fn can_move(
    maze: &Array2D<char>,
    current: Coordinate,
    direction: Direction
) -> Option<Coordinate> {
    let d = direction.to_offset();
    let next = current + d;
    if let Some(&char) = maze.get_safe(&next) {
        if char != '#' {
            return Some(next);
        }
    }
    None
}

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<u64> {
    let maze = parse_input(reader);

    let start = find_position(&maze, 'S');
    let end = find_position(&maze, 'E');

    let directions = [
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up
    ];

    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();

    heap.push(State {
        position: start,
        direction: Direction::Right,
        cost: 0
    });

    while let Some(current) = heap.pop() {
        if current.position == end {
            return Ok(current.cost);
        }

        let state_key = (current.position, current.direction);
        if visited.contains(&state_key) {
            continue;
        }
        visited.insert(state_key);

        for &next_direction in &directions {
            if next_direction.opposite_direction() == current.direction {
                continue
            }

            if let Some(next_pos) = can_move(&maze, current.position, next_direction) {
                let turn_cost = if next_direction != current.direction { 1000 } else { 0 };
                let next_state = State {
                    position: next_pos,
                    direction: next_direction,
                    cost: current.cost + 1 + turn_cost
                };

                heap.push(next_state);
            }
        }
    }

    Err(anyhow!("No path found!"))
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

        fn test_part1(expect: u64, input: &str) {
            assert_eq!(expect, part1(BufReader::new(input.as_bytes())).unwrap());
        }

        #[test]
        fn test1() {
            test_part1(
                7036,
                indoc! {"
                    ###############
                    #.......#....E#
                    #.#.###.#.###.#
                    #.....#.#...#.#
                    #.###.#####.#.#
                    #.#.#.......#.#
                    #.#.#####.###.#
                    #...........#.#
                    ###.#.#####.#.#
                    #...#.....#.#.#
                    #.#.#.###.#.#.#
                    #.....#...#.#.#
                    #.###.#.#.#.#.#
                    #S..#.....#...#
                    ###############
                "},
            );
        }

        #[test]
        fn test2() {
            test_part1(
                11048,
                indoc! {"
                    #################
                    #...#...#...#..E#
                    #.#.#.#.#.#.#.#.#
                    #.#.#.#...#...#.#
                    #.#.#.#.###.#.#.#
                    #...#.#.#.....#.#
                    #.#.#.#.#.#####.#
                    #.#...#.#.#.....#
                    #.#.#####.#.###.#
                    #.#.#.......#...#
                    #.#.###.#####.###
                    #.#.#...#.....#.#
                    #.#.#.#####.###.#
                    #.#.#.........#.#
                    #.#.#.#########.#
                    #S#.............#
                    #################
                "}
            )
        }

        #[test]
        fn test3() {
            test_part1(
                3004,
                indoc! {"
                    #####
                    #...#
                    #S#E#
                    #####
                "}
            )
        }

        #[test]
        fn part1_final() {
            assert_eq!(107468, run_on_day_input(day!(), part1).unwrap());
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
