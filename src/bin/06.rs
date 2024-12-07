use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};
use std::result;
use itertools::Itertools;
use linked_hash_set::LinkedHashSet;
use crate::Action::{Move, TurnRight};
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

    fn turn_left(&self) -> Direction {
        match self {
            Direction::UP => Direction::LEFT,
            Direction::RIGHT => Direction::UP,
            Direction::DOWN => Direction::RIGHT,
            Direction::LEFT => Direction::DOWN,
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

#[derive(Hash, Eq, PartialEq)]
enum Action {
    Move,
    TurnRight,
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

fn left_hand_coordinate(position: &Position, map: &Map) -> Option<Coordinate> {
    match position.direction {
        Direction::UP => left(&position.coordinate, map),
        Direction::RIGHT => up(&position.coordinate, map),
        Direction::DOWN => right(&position.coordinate, map),
        Direction::LEFT => down(&position.coordinate, map)
    }
}

fn step_forward_coordinate(position: &Position, map: &Map) -> Option<Coordinate> {
    match position.direction {
        Direction::UP => up(&position.coordinate, map),
        Direction::RIGHT => right(&position.coordinate, map),
        Direction::DOWN => down(&position.coordinate, map),
        Direction::LEFT => left(&position.coordinate, map)
    }
}

fn move_guard(position: &Position, map: &Map) -> (Action, Option<Position>) {
    match step_forward_coordinate(position, map) {
        None => (Move, None),
        Some(next_coordinate) => {
            if map.map[next_coordinate.y][next_coordinate.x] == Wall {
                return (TurnRight, Some(Position {
                    direction: position.direction.turn_right(),
                    coordinate: position.coordinate,
                }));
            }

            (Move, Some(Position {
                direction: position.direction.clone(),
                coordinate: next_coordinate,
            }))
        }
    }
}

fn backward_empty_cell(position: &Position, map: &Map) -> Option<Position> {
    let backward_coordinate = match position.direction {
        Direction::UP => down(&position.coordinate, map),
        Direction::RIGHT => left(&position.coordinate, map),
        Direction::DOWN => up(&position.coordinate, map),
        Direction::LEFT => right(&position.coordinate, map)
    }?;

    if map.get_cell(backward_coordinate) != Empty {
        return None;
    }

    Some(Position {
        direction: position.direction,
        coordinate: backward_coordinate,
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
            y_size,
        }
    };
    Ok((start.unwrap(), map))
}

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let (start, map) = read_input(reader)?;

    let mut cur_position = Some(Position {
        direction: Direction::UP,
        coordinate: start,
    });

    let mut trace: HashSet<Position> = HashSet::new();
    trace.insert(cur_position.unwrap());

    while cur_position.is_some() {
        let (_, next_position) = move_guard(cur_position.as_ref().unwrap(), &map);
        if next_position.is_some() {
            if !trace.insert(next_position.unwrap()) {
                break;
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

fn build_back_on_track(
    position: &Position,
    map: &Map,
    back_on_track: &HashSet<Position>,
) -> LinkedHashSet<Position> {
    let mut result = LinkedHashSet::new();
    let mut queue = Vec::new();
    queue.push(position.clone());
    while !queue.is_empty() {
        let next = queue.pop().unwrap();
        if !result.insert(next) || back_on_track.contains(&next) {
            continue
        }

        if let Some(left_hand) = left_hand_coordinate(&next, &map) {
            if map.get_cell(left_hand) == Wall {
                queue.push(Position {
                    direction: next.direction.turn_left(),
                    coordinate: next.coordinate.clone(),
                });
            }
        }
        if let Some(backward) = backward_empty_cell(&next, &map) {
            queue.push(backward);
        }
    }

    result
}

//noinspection DuplicatedCode
fn part2<R: BufRead>(reader: R) -> Result<i64> {
    let (start, map) = read_input(reader)?;

    let mut cur_position = Position {
        direction: Direction::UP,
        coordinate: start,
    };

    let mut back_on_track: HashSet<Position> = HashSet::new();
    let mut trace: LinkedHashSet<Position> = LinkedHashSet::new();

    let mut obstacles_coordinates: HashSet<Coordinate> = HashSet::new();

    loop {
        if !trace.insert(cur_position) {
            panic!("Loop in the original track. It's not expected.")
        }

        back_on_track.extend(build_back_on_track(&cur_position, &map, &back_on_track));

        let turn_right_position = Position {
            direction: cur_position.direction.turn_right(),
            coordinate: cur_position.coordinate,
        };

        if back_on_track.contains(&turn_right_position) {
            if let Some(obstacle_coordinate) = step_forward_coordinate(&cur_position, &map) {
                if map.get_cell(obstacle_coordinate) == Empty {
                    if obstacle_coordinate != start {
                        obstacles_coordinates.insert(obstacle_coordinate);
                    }
                }
            }
        }

        let (_, next_position_opt) = move_guard(&cur_position, &map);
        if next_position_opt.is_none() {
            break;
        }

        cur_position = next_position_opt.unwrap();
    }

    println!(
        "{}",
        visited_map(&map, &trace));

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
    fn part2_final() {
        part2_result().unwrap();
    }
}
