use std::collections::HashSet;
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use indoc::indoc;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use crate::Cell::{Empty, Wall};

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum Direction {
    UP, RIGHT, DOWN, LEFT
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Coordinate { x: usize, y: usize }

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    direction: Direction,
    coordinate: Coordinate
}

#[derive(Hash, Eq, PartialEq)]
enum Cell {
    Wall, Empty
}

struct Map {
    map: Vec<Vec<Cell>>,
    x_size : usize,
    y_size : usize
}

fn move_guard(position: &Position, map: &Map) -> Option<Position> {
    let next_position = match position.direction {
        Direction::UP => (position.coordinate.x as i32, position.coordinate.y as i32 - 1),
        Direction::RIGHT => (position.coordinate.x as i32 + 1, position.coordinate.y as i32),
        Direction::DOWN => (position.coordinate.x as i32, position.coordinate.y as i32 + 1),
        Direction::LEFT => (position.coordinate.x as i32 - 1, position.coordinate.y as i32)
    };

    if !(
        (0..map.x_size as i32).contains(&next_position.0) &&
        (0 .. map.y_size as i32).contains(&next_position.1)
    ) {
        return None;
    }

    let next_coordinate = Coordinate {
        x: next_position.0 as usize,
        y: next_position.1 as usize
    };

    if map.map[next_coordinate.y][next_coordinate.x] == Wall {
        return Some(Position {
            direction: position.direction.turn_right(),
            coordinate: position.coordinate
        });
    }

    Some(Position {
        direction: position.direction.clone(),
        coordinate: next_coordinate
    })
}

fn visited_map(map: &Map, visited: &HashSet<Coordinate>) -> String {
    map.map.iter().enumerate().map(|(y, row)| {
        let row_string: String = row.iter().enumerate()
            .map(|(x, cell)| {
                match cell {
                    Wall => '#',
                    Empty => if visited.contains(&Coordinate { x, y }) {
                        'X'
                    } else {
                        '.'
                    }
                }
            })
            .into_iter()
            .collect();
        row_string
    }).join("\n")
}

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let mut start: Option<Coordinate> = None;
    let map: Vec<Vec<Cell>> = reader.lines()
        .flatten()
        .enumerate()
        .filter(|(_, line)| !line.is_empty())
        .map(|(y, line)| {
            line.char_indices().map(|(x, c)| {
                match c {
                    '.' => Empty,
                    '#' => Wall,
                    '^' => {
                        start = Some(Coordinate { x, y });
                        Empty
                    }
                    _ => {
                        panic!("Unknown character: {}", c);
                    }
                }
            }).collect()
        })
        .collect();

    let map = if map.is_empty() || map[0].is_empty() {
        return Err(anyhow!("Map is empty"));
    } else {
        let x_size = map[0].len();
        let y_size = map.len();

        Map {
            map,
            x_size,
            y_size
        }
    };

    let mut cur_position = Some(Position {
        direction: Direction::UP,
        coordinate: start.unwrap()
    });

    let mut trace: HashSet<Position> = HashSet::new();
    trace.insert(cur_position.unwrap());

    while cur_position.is_some() {
        cur_position = move_guard(cur_position.as_ref().unwrap(), &map);
        if cur_position.is_some() {
            if !trace.insert(cur_position.unwrap()) {
                break
            }
        }
    }

    let visited_coordinates = trace.iter()
        .map(|position: &Position| position.coordinate)
        .collect::<HashSet<Coordinate>>();

    println!("{}", visited_map(&map, &visited_coordinates));

    Ok(visited_coordinates
        .len() as i64)
}

//noinspection DuplicatedCode
fn part2<R: BufRead>(_reader: R) -> Result<i64> {
    Ok(0)
}

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
            41,
            indoc! {"
                ....#.....
                .........#
                ..........
                ..#.......
                .......#..
                ..........
                .#..^.....
                ........#.
                #.........
                ......#...
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
        test_part2(0, indoc! {"
            1   2
        "});
    }

    #[test]
    fn part2_final() {
        part2_result().unwrap();
    }
}
