use std::collections::{HashMap, HashSet};
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};
use itertools::Itertools;
use linked_hash_set::LinkedHashSet;
use crate::Action::{Move, TurnRight};
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

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
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

#[derive(Hash, Eq, PartialEq)]
enum Action {
    Move, TurnRight
}

fn step_forward_coordinate(position: &Position, map: &Map) -> Option<Coordinate> {
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

    Some(Coordinate {
        x: next_position.0 as usize,
        y: next_position.1 as usize
    })
}

fn move_guard(position: &Position, map: &Map) -> (Action, Option<Position>) {
    match step_forward_coordinate(position, map) {
        None => (Move, None),
        Some(next_coordinate) => {
            if map.map[next_coordinate.y][next_coordinate.x] == Wall {
                return (TurnRight, Some(Position {
                    direction: position.direction.turn_right(),
                    coordinate: position.coordinate
                }));
            }

            (Move, Some(Position {
                direction: position.direction.clone(),
                coordinate: next_coordinate
            }))
        }
    }
}

fn backward_empty_cell(position: &Position, map: &Map) -> Option<Position> {
    let backward_position = match position.direction {
        Direction::UP => (position.coordinate.x as i32, position.coordinate.y as i32 + 1),
        Direction::RIGHT => (position.coordinate.x as i32 - 1, position.coordinate.y as i32),
        Direction::DOWN => (position.coordinate.x as i32, position.coordinate.y as i32 - 1),
        Direction::LEFT => (position.coordinate.x as i32 + 1, position.coordinate.y as i32)
    };

    if !(
        (0..map.x_size as i32).contains(&backward_position.0) &&
            (0 .. map.y_size as i32).contains(&backward_position.1)
    ) {
        return None;
    }

    let next_coordinate = Coordinate {
        x: backward_position.0 as usize,
        y: backward_position.1 as usize
    };

    if map.map[next_coordinate.y][next_coordinate.x] != Empty {
        return None;
    }
    
    Some(Position {
        direction: position.direction,
        coordinate: next_coordinate
    })
}

fn visited_map(map: &Map, visited: &LinkedHashSet<Position>) -> String {
    let trace_map: HashMap<Coordinate, Direction> =
        visited.iter().map(|position| (position.coordinate, position.direction)).collect();

    map.map.iter().enumerate().map(|(y, row)| {
        let row_string: String = row.iter().enumerate()
            .map(|(x, cell)| {
                match cell {
                    Wall => '#',
                    Empty => {
                        let visited = trace_map.get(&Coordinate { x, y });
                        if visited.is_some() {
                            match visited.unwrap() {
                                Direction::UP => '^',
                                Direction::RIGHT => '>',
                                Direction::DOWN => '|',
                                Direction::LEFT => '<'
                            }
                        } else {
                            '.'
                        }
                    }
                }
            })
            .into_iter()
            .collect();
        row_string
    }).join("\n")
}

fn read_input<R: BufRead>(reader: R) -> Result<(Coordinate, Map)> {
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
    Ok((start.unwrap(), map))
}

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let (start, map) = read_input(reader)?;

    let mut cur_position = Some(Position {
        direction: Direction::UP,
        coordinate: start
    });

    let mut trace: HashSet<Position> = HashSet::new();
    trace.insert(cur_position.unwrap());

    while cur_position.is_some() {
        let (_, next_position) = move_guard(cur_position.as_ref().unwrap(), &map);
        if next_position.is_some() {
            if !trace.insert(next_position.unwrap()) {
                break
            }
        }
        cur_position = next_position;
    }

    let visited_coordinates = trace.iter()
        .map(|position: &Position| position.coordinate)
        .collect::<HashSet<Coordinate>>();

    Ok(visited_coordinates
        .len() as i64)
}

//noinspection DuplicatedCode
fn part2<R: BufRead>(reader: R) -> Result<i64> {
    let (start, map) = read_input(reader)?;

    let mut cur_position = Position {
        direction: Direction::UP,
        coordinate: start
    };

    let mut back_on_track: HashSet<Position> = HashSet::new();
    let build_back_on_track = |position: Position| -> Vec<Position> {
        std::iter::successors(Some(position.clone()), |pos| {
            backward_empty_cell(pos, &map)
        }).collect()
    };

    let mut trace: LinkedHashSet<Position> = LinkedHashSet::new();
    trace.insert(cur_position);
    back_on_track.extend(build_back_on_track(cur_position));

    let mut obstacles_coordinates: LinkedHashSet<Coordinate> = LinkedHashSet::new();

    loop {
        let (action, next_position_opt) = move_guard(&cur_position, &map);
        if next_position_opt.is_none() {
            break
        }

        let next_position = next_position_opt.unwrap();
        if !trace.insert(next_position) {
            panic!("Loop in the original track. It's not expected.")
        }

        if action == TurnRight {
            back_on_track.extend(build_back_on_track(next_position));
        } else {
            let turn_right_position = Position {
                direction: next_position.direction.turn_right(),
                coordinate: next_position.coordinate,
            };
            if trace.contains(&turn_right_position) || back_on_track.contains(&turn_right_position) {
                let next_coordinate_opt = step_forward_coordinate(&next_position, &map);
                if let Some(obstacle_coordinate) = next_coordinate_opt {
                    if map.map[obstacle_coordinate.y][obstacle_coordinate.x] == Empty {
                        if !(obstacle_coordinate.x == start.x && obstacle_coordinate.y == start.y) {
                            obstacles_coordinates.insert(obstacle_coordinate);
                        }
                    }
                }
            }

            if let Some(backward_position) = backward_empty_cell(next_position_opt.as_ref().unwrap(), &map) {
                back_on_track.insert(backward_position);
            }
        }

        cur_position = next_position;
    }

    Ok(obstacles_coordinates.iter().count() as i64)
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

//noinspection SpellCheckingInspection
#[cfg(test)]
mod part1_tests {
    use std::io::BufReader;
    use indoc::indoc;
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
    use std::io::BufReader;
    use indoc::indoc;
    use super::*;

    fn test_part2(expect: i64, input: &str) {
        assert_eq!(expect, part2(BufReader::new(input.as_bytes())).unwrap());
    }

    #[test]
    fn test1() {
        test_part2(
            6,
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
    fn part2_final() {
        part2_result().unwrap();
    }
}
