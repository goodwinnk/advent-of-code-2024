use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};
use itertools::Itertools;
use linked_hash_set::LinkedHashSet;
use crate::Cell::{Empty, Wall};

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
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
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Position {
    direction: Direction,
    coordinate: Coordinate,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Cell {
    Wall,
    Empty,
}

struct Map {
    map: Vec<Vec<Cell>>,
    x_size: usize,
    y_size: usize,
}

impl Map {
    fn get_cell(&self, coordinate: Coordinate) -> Cell {
        let y = coordinate.y;
        let x = coordinate.x;
        self.map[y][x]
    }
}

fn apply_shift(coordinate: &Coordinate, map: &Map, shift: (i32, i32)) -> Option<Coordinate> {
    let (dx, dy) = shift;
    let x = coordinate.x as i32 + dx;
    let y = coordinate.y as i32 + dy;
    if 0 <= x && x < map.x_size as i32 && 0 <= y && y < map.y_size as i32 {
        Some(Coordinate {
            x: x as usize,
            y: y as usize,
        })
    } else {
        None
    }
}

fn left(coordinate: &Coordinate, map: &Map) -> Option<Coordinate> {
    apply_shift(coordinate, map, (-1, 0))
}

fn right(coordinate: &Coordinate, map: &Map) -> Option<Coordinate> {
    apply_shift(coordinate, map, (1, 0))
}

fn up(coordinate: &Coordinate, map: &Map) -> Option<Coordinate> {
    apply_shift(coordinate, map, (0, -1))
}

fn down(coordinate: &Coordinate, map: &Map) -> Option<Coordinate> {
    apply_shift(coordinate, map, (0, 1))
}

fn step_forward_coordinate(position: &Position, map: &Map) -> Option<Coordinate> {
    match position.direction {
        Direction::UP => up(&position.coordinate, map),
        Direction::RIGHT => right(&position.coordinate, map),
        Direction::DOWN => down(&position.coordinate, map),
        Direction::LEFT => left(&position.coordinate, map)
    }
}

fn move_guard(position: &Position, map: &Map, additional_wall: Option<Coordinate>) -> Option<Position> {
    let next_coordinate = step_forward_coordinate(position, map)?;

    if map.get_cell(next_coordinate) == Wall ||
        additional_wall.map_or(false, |coordinate| coordinate == next_coordinate) {
        return Some(Position {
            direction: position.direction.turn_right(),
            coordinate: position.coordinate,
        });
    }

    Some(Position {
        direction: position.direction.clone(),
        coordinate: next_coordinate,
    })
}

#[allow(dead_code)]
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
            y_size,
        }
    };
    Ok((start.unwrap(), map))
}

fn build_trace(position: &Position, map: &Map, additional_wall: Option<Coordinate>) -> (bool, LinkedHashSet<Position>) {
    let mut trace: LinkedHashSet<Position> = LinkedHashSet::new();
    let mut cur_position = position.clone();

    loop {
        if !trace.insert(cur_position) {
            return (true, trace);
        }

        let next_position = move_guard(&cur_position, &map, additional_wall);
        if next_position.is_none() {
            return (false, trace);
        }

        cur_position = next_position.unwrap();
    }
}

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let (start, map) = read_input(reader)?;
    let start_position = Position {
        direction: Direction::UP,
        coordinate: start,
    };

    let (_, trace) = build_trace(&start_position, &map, None);

    let visited_coordinates = trace.iter()
        .map(|position: &Position| position.coordinate)
        .collect::<HashSet<Coordinate>>();

    Ok(visited_coordinates.len() as i64)
}

//noinspection DuplicatedCode
fn part2<R: BufRead>(reader: R) -> Result<i64> {
    let (start, map) = read_input(reader)?;

    let starting_position = Position {
        direction: Direction::UP,
        coordinate: start,
    };

    let (_, trace) = build_trace(&starting_position, &map, None);

    Ok(trace.iter()
        .map(|position: &Position| position.coordinate)
        .collect::<LinkedHashSet<Coordinate>>().iter()
        .filter(|&&coordinate| coordinate != start)
        .filter(|&&coordinate| {
            build_trace(&starting_position, &map, Some(coordinate)).0
        })
        .count() as i64)
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
mod tests {
    use super::*;
    use std::io::BufReader;
    use indoc::indoc;

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
            assert_eq!(5239i64, run_on_day_input(day!(), part1).unwrap());
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
        fn test2() {
            test_part2(
                2,
                indoc! {"
                .#...
                ....#
                #^...
                ..#..
            "},
            );
        }

        #[test]
        fn test3() {
            test_part2(
                2,
                indoc! {"
                .#...
                ....#
                #....
                .^#..
            "},
            );
        }

        #[test]
        fn test4() {
            test_part2(
                1,
                indoc! {"
                ....
                ...#
                #...
                .^#.
            "},
            );
        }

        #[test]
        fn test5() {
            test_part2(
                1,
                indoc! {"
                ....
                .^.#
                #...
                ..#.
            "},
            );
        }

        #[test]
        fn test6() {
            test_part2(
                1,
                indoc! {"
                ...#...
                ......#
                ..#....
                #^...#.
                ..#....
            "},
            );
        }

        #[test]
        fn part2_final() {
            assert_eq!(1753i64, run_on_day_input(day!(), part2).unwrap());
        }
    }
}

