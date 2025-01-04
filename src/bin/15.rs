use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};
use array2d::Array2D;
use advent_of_code2024_rust::matrix::{Array2DExt, Coordinate, Direction};
use Direction::{Down, Left, Right, Up};
use Tile::{Empty, Robot, Wall, Box};

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Box,
    Robot,
}

#[derive(Debug, Clone)]
struct Warehouse {
    map: Array2D<Tile>,
    robot_pos: (usize, usize),
}

//noinspection DuplicatedCode
impl Warehouse {
    fn try_move(&mut self, direction: &Direction) -> bool {
        let d = direction.to_offset();

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

#[derive(Debug, Clone, PartialEq)]
enum ExtendedTile {
    Wall,
    Empty,
    BoxLeft,
    BoxRight,
    Robot,
}

#[derive(Debug, Clone)]
struct ExtendedWarehouse {
    map: Array2D<ExtendedTile>,
    robot_pos: (usize, usize),
}

//noinspection DuplicatedCode
impl ExtendedWarehouse {
    fn from_warehouse(warehouse: &Warehouse) -> Self {
        let rows = warehouse.map.num_rows();
        let cols = warehouse.map.num_columns();

        let mut extended_rows: Vec<Vec<ExtendedTile>> = Vec::with_capacity(rows);
        for y in 0..rows {
            let mut extended_row = Vec::with_capacity(cols * 2);
            for x in 0..cols {
                match warehouse.map.get(y, x).unwrap() {
                    Wall => {
                        extended_row.push(ExtendedTile::Wall);
                        extended_row.push(ExtendedTile::Wall);
                    },
                    Empty => {
                        extended_row.push(ExtendedTile::Empty);
                        extended_row.push(ExtendedTile::Empty);
                    },
                    Box => {
                        extended_row.push(ExtendedTile::BoxLeft);
                        extended_row.push(ExtendedTile::BoxRight);
                    },
                    Robot => {
                        extended_row.push(ExtendedTile::Robot);
                        extended_row.push(ExtendedTile::Empty);
                    }
                }
            }

            extended_rows.push(extended_row);
        }

        let robot_pos = (
            warehouse.robot_pos.0,
            warehouse.robot_pos.1 * 2
        );

        ExtendedWarehouse {
            map: Array2D::from_rows(&extended_rows).unwrap(),
            robot_pos,
        }
    }

    fn push_box(
        &mut self,
        is_try: bool,
        coordinate: &Coordinate,
        direction: &Direction,
    ) -> bool {
        assert_eq!(self.map.get_safe(&coordinate), Some(&ExtendedTile::BoxLeft), "Expected to be box left corner {}", coordinate);

        let d = direction.to_offset();
        let spaces = match direction {
            &Up | &Down => Vec::from([*coordinate + d, *coordinate + (0, 1) + d]),
            &Right => Vec::from([*coordinate + (0, 1) + d]),
            &Left => Vec::from([*coordinate + d])
        };

        if !self.push(is_try, &spaces, direction) {
            if !is_try {
                panic!("Cannot push the box {}", coordinate);
            }
            return false;
        }

        if !is_try {
            self.map.set_coord(&coordinate, ExtendedTile::Empty).unwrap();
            self.map.set_coord(&(*coordinate + (0, 1)), ExtendedTile::Empty).unwrap();
            self.map.set_coord(&(*coordinate + d), ExtendedTile::BoxLeft).unwrap();
            self.map.set_coord(&(*coordinate + (0, 1) + d), ExtendedTile::BoxRight).unwrap();
        }

        true
    }

    fn push_robot(&mut self, is_try: bool, direction: &Direction) -> bool {
        let robot_coordinate = Coordinate {
            row: self.robot_pos.0 as isize,
            column: self.robot_pos.1 as isize,
        };
        assert_eq!(self.map.get_safe(&robot_coordinate), Some(&ExtendedTile::Robot), "Robot is not on the map");
        let desired_robot_coordinate = robot_coordinate + direction.to_offset();
        if !self.push(is_try, &[desired_robot_coordinate].to_vec(), direction) {
            if !is_try {
                panic!("Cannot push the robot {}", &robot_coordinate);
            }
            return false;
        }

        if !is_try {
            self.map.set_coord(&robot_coordinate, ExtendedTile::Empty).unwrap();
            self.map.set_coord(&(desired_robot_coordinate), ExtendedTile::Robot).unwrap();
            self.robot_pos = (desired_robot_coordinate.row as usize, desired_robot_coordinate.column as usize);
        }

        true
    }

    fn push(&mut self, is_try: bool, spaces: &Vec<Coordinate>, direction: &Direction) -> bool {
        let mut visited_boxes: HashSet<Coordinate> = HashSet::new();
        for space in spaces {
            if let Some(tile) = self.map.get_safe(space) {
                match tile {
                    ExtendedTile::Wall => {
                        if !is_try {
                            panic!("Cannot push the wall {}", space);
                        }
                        return false;
                    }
                    ExtendedTile::Empty => {}
                    ExtendedTile::BoxLeft => {
                        if !visited_boxes.contains(space) {
                            visited_boxes.insert(space.clone());
                            if !self.push_box(is_try, space, direction) {
                                return false
                            }
                        }
                    }
                    ExtendedTile::BoxRight => {
                        let box_coordinate = *space + (0, -1);
                        if !visited_boxes.contains(&box_coordinate) {
                            visited_boxes.insert(box_coordinate.clone());
                            if !self.push_box(is_try, &box_coordinate, direction) {
                                return false
                            }
                        }
                    }
                    ExtendedTile::Robot => {
                        panic!("No actors to push the robot {}", space);
                    }
                }
            } else {
                if !is_try {
                    panic!("Cannot push outside the map {}", space);
                }
                return false;
            }
        }
        true
    }

    fn move_robot(&mut self, moves: &Vec<Direction>) {
        for direction in moves {
            if self.push_robot(true, direction) {
                self.push_robot(false, direction);
            }
        }
    }

    fn calculate_gps_coordinates(&self) -> usize {
        let mut coordinates = 0;
        for y in 0..self.map.num_rows() {
            for x in 0..self.map.num_columns() {
                if self.map.get(y, x).unwrap() == &ExtendedTile::BoxLeft {
                    coordinates += 100 * y + x;
                }
            }
        }
        coordinates
    }
}

impl Display for ExtendedWarehouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.map.num_rows() {
            for x in 0..self.map.num_columns() {
                write!(f, "{}", match self.map.get(y, x).unwrap() {
                    ExtendedTile::Wall => '#',
                    ExtendedTile::Empty => '.',
                    ExtendedTile::BoxLeft => '[',
                    ExtendedTile::BoxRight => ']',
                    ExtendedTile::Robot => '@',
                })?;
            }
            writeln!(f)?;
        }
        Result::Ok(())
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
            '^' => Up,
            'v' => Down,
            '<' => Left,
            '>' => Right,
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
fn part2<R: BufRead>(reader: R) -> Result<i64> {
    let (warehouse, moves) = parse_input(reader)?;
    let mut warehouse = ExtendedWarehouse::from_warehouse(&warehouse);
    warehouse.move_robot(&moves);
    Ok(warehouse.calculate_gps_coordinates() as i64)
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
        fn extend_warehouse() {
            let warehouse = parse_warehouse(indoc! {"
                #######
                #...#.#
                #.....#
                #..OO@#
                #..O..#
                #.....#
                #######
            "});
            let warehouse = ExtendedWarehouse::from_warehouse(&warehouse);
            let display_string = format!("{}", warehouse);
            assert_eq!(
                indoc! {"
                    ##############
                    ##......##..##
                    ##..........##
                    ##....[][]@.##
                    ##....[]....##
                    ##..........##
                    ##############
                "},
                display_string,
            );
        }

        #[test]
        fn test1() {
            test_part2(
                618,
                indoc! {"
                    #######
                    #...#.#
                    #.....#
                    #..OO@#
                    #..O..#
                    #.....#
                    #######

                    <vv<<^^<<^^
                "}
            );
        }

        #[test]
        fn test2() {
            test_part2(
                9021,
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
                "}
            );
        }

        #[test]
        fn part2_final() {
            assert_eq!(1471049, run_on_day_input(day!(), part2).unwrap());
        }
    }
}
