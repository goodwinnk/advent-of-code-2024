use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};
use std::ops::Add;
use array2d::Array2D;
use Direction::{DOWN, LEFT, RIGHT, UP};
use Tile::{Empty, Robot, Wall, Box};

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Box,
    Robot,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Debug, Clone)]
struct Warehouse {
    map: Array2D<Tile>,
    robot_pos: (usize, usize),
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Coordinate {
    row: isize,
    column: isize,
}

impl Add<(isize, isize)> for Coordinate {
    type Output = Coordinate;

    fn add(self, other: (isize, isize)) -> Coordinate {
        Coordinate {
            row: self.row + other.0,
            column: self.column + other.1,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum Array2DErrorExt {
    InvalidCoordinate(Coordinate),
    Base(array2d::Error)
}

trait Array2DExt<T> {
    fn get_safe(&self, coordinate: &Coordinate) -> Option<&T>;
    fn set_coord(&mut self, coordinate: &Coordinate, value: T) -> Result<(), Array2DErrorExt>;
}

// Implement the trait for Array2D
impl<T> Array2DExt<T> for Array2D<T> {
    #[inline(always)]
    fn get_safe(&self, coordinate: &Coordinate) -> Option<&T> {
        if coordinate.row >= 0 && coordinate.column >= 0 {
            self.get(coordinate.row as usize, coordinate.column as usize)
        } else {
            None
        }
    }

    #[inline(always)]
    fn set_coord(&mut self, coordinate: &Coordinate, value: T) -> Result<(), Array2DErrorExt> {
        if coordinate.row >= 0 && coordinate.column >= 0 {
            self.set(coordinate.row as usize, coordinate.column as usize, value)
                .map_err(|e| Array2DErrorExt::Base(e))
        } else {
            Err(Array2DErrorExt::InvalidCoordinate(coordinate.clone()))
        }
    }
}

impl Warehouse {
    fn try_move(&mut self, direction: &Direction) -> bool {
        let d = match direction {
            UP => (-1, 0),
            DOWN => (1, 0),
            LEFT => (0, -1),
            RIGHT => (0, 1)
        };

        let robot_coordinate = Coordinate {
            row: self.robot_pos.0 as isize,
            column: self.robot_pos.1 as isize,
        };
        assert_eq!(self.map.get_safe(&robot_coordinate), Some(&Robot), "Robot is not on the map");

        let robot_next = robot_coordinate + d;
        if self.map.get_safe(&robot_next).is_none() {
            return false;
        }

        if self.map.get_safe(&(robot_next)) == Some(&Box) {
            let mut curr = robot_coordinate + d;
            while self.map.get_safe(&curr) == Some(&Box) {
                curr = curr + d;
            }

            // Check if the space after the last box is empty
            if self.map.get_safe(&curr) != Some(&Empty) {
                return false;
            }

            // Put the first box to the empty place
            self.map.set_coord(&curr, Box).unwrap();
            self.map.set_coord(&(robot_next), Empty).unwrap();
        }

        if self.map.get_safe(&(robot_next)) != Some(&Empty) {
            return false
        }

        self.map.set_coord(&robot_coordinate, Empty).unwrap();
        self.map.set_coord(&(robot_next), Robot).unwrap();
        self.robot_pos = (robot_next.row as usize, robot_next.column as usize);

        true
    }

    fn move_robot(&mut self, moves: &Vec<Direction>) {
        for direction in moves {
            self.try_move(direction);
        }
    }

    fn calculate_gps_coordinates(&self) -> usize {
        let mut coordinates = 0;
        for y in 0..self.map.num_rows() {
            for x in 0..self.map.num_columns() {
                if self.map.get(y, x).unwrap() == &Box {
                    coordinates += 100 * y + x;
                }
            }
        }
        coordinates
    }

    #[allow(dead_code)]
    fn display(&self) {
        for y in 0..self.map.num_rows() {
            for x in 0..self.map.num_columns() {
                print!("{}", match self.map.get(y, x).unwrap() {
                    Wall => '#',
                    Empty => '.',
                    Box => 'O',
                    Robot => '@',
                });
            }
            println!();
        }
    }
}


fn parse_warehouse(input: &str) -> Warehouse {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    let mut robot_pos = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            let tile = match ch {
                '#' => Wall,
                '.' => Empty,
                'O' => Box,
                '@' => {
                    robot_pos = (y, x);
                    Robot
                },
                _ => continue,
            };
            row.push(tile);
        }
        if !row.is_empty() {
            map.push(row);
        } else {
            break;
        }
    }

    Warehouse {
        map: Array2D::from_rows(map.as_slice()).unwrap(),
        robot_pos
    }
}

fn parse_moves(input: &str) -> Vec<Direction> {
    input.chars()
        .filter(|&c| matches!(c, '^' | 'v' | '<' | '>'))
        .map(|c| match c {
            '^' => UP,
            'v' => DOWN,
            '<' => LEFT,
            '>' => RIGHT,
            _ => unreachable!(),
        })
        .collect()
}

fn parse_input<R: BufRead>(mut reader: R) -> Result<(Warehouse, Vec<Direction>)> {
    let mut input_string = String::new();
    reader.read_to_string(&mut input_string)?;

    let warehouse = parse_warehouse(input_string.as_str());
    let moves = parse_moves(input_string.as_str());

    Ok((warehouse, moves))
}

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let (mut warehouse, moves) = parse_input(reader)?;
    warehouse.move_robot(&moves);
    Ok(warehouse.calculate_gps_coordinates() as i64)
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
                10092,
                indoc! {"
                    ##########
                    #..O..O.O#
                    #......O.#
                    #.OO..O.O#
                    #..O@..O.#
                    #O#..O...#
                    #O..O..O.#
                    #.OO.O.OO#
                    #....O...#
                    ##########

                    <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
                    vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
                    ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
                    <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
                    ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
                    ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
                    >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
                    <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
                    ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
                    v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
                "},
            );
        }

        #[test]
        fn test2() {
            test_part1(
                2028,
                indoc! {"
                    ########
                    #..O.O.#
                    ##@.O..#
                    #...O..#
                    #.#.O..#
                    #...O..#
                    #......#
                    ########

                    <^^>>>vv<v>>v<<
                "}
            )
        }

        #[test]
        fn part1_final() {
            assert_eq!(1465523, run_on_day_input(day!(), part1).unwrap());
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
