use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};
use array2d::Array2D;
use itertools::Itertools;
use Cell::{End, Path, Start, Wall};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Wall, Path, Start, End,
}

impl Cell {
    fn from_char(c: char) -> Cell {
        match c {
            '#' => Wall,
            '.' => Path,
            'S' => Start,
            'E' => End,
            _ => panic!("Invalid character in map: {}", c),
        }
    }

    fn is_walkable(&self) -> bool {
        matches!(self, Path | Start | End)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn new(row: i32, col: i32) -> Self {
        Point { row, col }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(r:{}, c:{})", self.row, self.col)
    }
}

struct RaceTrack {
    map: Array2D<Cell>,
    start: Point,
    end: Point,
    rows: i32,
    cols: i32,
}

#[derive(Debug)]
struct SavingRoute {
    before: i64,
    cheat_start: Point,
    cheat_end: Point,
    cheat_steps: i64,
    after: i64,
    savings: i64,
}

impl Display for SavingRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SavingRoute {{ before: {}, cheat_start: {}, cheat_end: {}, cheat_steps: {}, after: {}, savings: {} }}",
               self.before, self.cheat_start, self.cheat_end, self.cheat_steps, self.after, self.savings)
    }
}

impl RaceTrack {
    fn from_reader<R: BufRead>(reader: R) -> Result<Self> {
        let mut cells = Vec::new();
        let mut start = Point::new(0, 0);
        let mut end = Point::new(0, 0);
        let mut cols = 0;

        for (row, line) in reader.lines().enumerate() {
            let line = line?;
            cols = line.len();
            for (col, c) in line.chars().enumerate() {
                let cell = Cell::from_char(c);
                if cell == Start {
                    start = Point::new(row as i32, col as i32);
                } else if cell == End {
                    end = Point::new(row as i32, col as i32);
                }
                cells.push(cell);
            }
        }

        let rows = cells.len() / cols;
        let map = Array2D::from_row_major(&cells, rows, cols)
            .expect("Failed to create Array2D");

        Ok(RaceTrack {
            map,
            start,
            end,
            rows: rows as i32,
            cols: cols as i32,
        })
    }

    fn build_cost_matrix(&self, from: Point) -> Array2D<i64> {
        let mut costs = Array2D::filled_with(i64::MAX, self.rows as usize, self.cols as usize);
        let mut queue = VecDeque::new();

        queue.push_back((from, 0));
        costs[(from.row as usize, from.col as usize)] = 0;

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some((current, cost)) = queue.pop_front() {
            for (dr, dc) in directions.iter() {
                let next = Point::new(current.row + dr, current.col + dc);

                if self.is_valid_point(next) {
                    let next_idx = (next.row as usize, next.col as usize);
                    if self.map[next_idx].is_walkable() && costs[next_idx] == i64::MAX {
                        costs[next_idx] = cost + 1;
                        queue.push_back((next, cost + 1));
                    }
                }
            }
        }

        costs
    }

    fn find_reachable_by_walls(&self, start: Point, max_steps: i64) -> Vec<(Point, i64)> {
        let mut visited = HashSet::new();
        let mut points = Vec::new();
        let mut queue = VecDeque::new();

        assert!(self.map[(start.row as usize, start.col as usize)].is_walkable());
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dr, dc) in directions.iter() {
            let next = Point::new(start.row + dr, start.col + dc);
            if !self.is_valid_point(next) || self.map[(next.row as usize, next.col as usize)].is_walkable() {
                continue;
            }

            // Wall
            queue.push_back((next, 1));
        }

        visited.insert(start);

        while let Some((current, steps)) = queue.pop_front() {
            if steps > max_steps || !self.is_valid_point(current) {
                continue;
            }

            if visited.contains(&current) {
                continue;
            }

            visited.insert(current);

            if self.map[(current.row as usize, current.col as usize)].is_walkable() {
                points.push((current, steps));
            } else {
                for (dr, dc) in directions.iter() {
                    let next = Point::new(current.row + dr, current.col + dc);
                    queue.push_back((next, steps + 1));
                }
            }
        }

        points
    }

    fn find_all_reachable_points(&self, start: Point, max_steps: i64) -> Vec<(Point, i64)> {
        let mut visited = Array2D::filled_with(false, self.rows as usize, self.cols as usize);
        let mut points = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back((start, 0));
        visited[(start.row as usize, start.col as usize)] = true;

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some((current, steps)) = queue.pop_front() {
            if steps > max_steps {
                continue;
            }

            points.push((current, steps));

            for (dr, dc) in directions.iter() {
                let next = Point::new(current.row + dr, current.col + dc);

                if self.is_valid_point(next) {
                    let next_idx = (next.row as usize, next.col as usize);
                    if !visited[next_idx] {
                        queue.push_back((next, steps + 1));
                        visited[next_idx] = true;
                    }
                }
            }
        }

        points
    }

    fn find_best_cheats(&self, min_savings: i64, max_cheat_duration: i64) -> Vec<SavingRoute> {
        let from_start = self.build_cost_matrix(self.start);
        let from_end = self.build_cost_matrix(self.end);

        let baseline = from_start[(self.end.row as usize, self.end.col as usize)];
        let mut cheats = Vec::new();

        // For each walkable point adjacent to a wall
        for row in 0..self.rows {
            for col in 0..self.cols {
                let current = Point::new(row, col);
                if !self.is_valid_point(current) || !self.map[(row as usize, col as usize)].is_walkable() {
                    continue;
                }

                let cost_to_start = from_start[(row as usize, col as usize)];
                if cost_to_start == i64::MAX {
                    continue;
                }

                // Find all points reachable within max_cheat_duration steps through walls
                let reachable_points = self.find_reachable_by_walls(current, max_cheat_duration);
                for (end_point, cheat_steps) in reachable_points {
                    if !self.map[(end_point.row as usize, end_point.col as usize)].is_walkable() {
                        continue;
                    }

                    let cost_to_end = from_end[(end_point.row as usize, end_point.col as usize)];
                    if cost_to_end == i64::MAX {
                        continue;
                    }

                    let total_cost = cost_to_start + cheat_steps + cost_to_end;
                    let savings = baseline - total_cost;

                    if savings >= min_savings {
                        cheats.push(SavingRoute {
                            before: cost_to_start,
                            cheat_start: current,
                            cheat_end: end_point,
                            cheat_steps,
                            after: cost_to_end,
                            savings,
                        });
                    }
                }
            }
        }

        cheats
    }

    fn is_valid_point(&self, point: Point) -> bool {
        point.row >= 0 && point.row < self.rows && point.col >= 0 && point.col < self.cols
    }
}

fn part1_general<R: BufRead>(reader: R, min_saving: i64) -> Result<i64> {
    let track = RaceTrack::from_reader(reader)?;
    let cheats = track.find_best_cheats(min_saving, 2);
    Ok(cheats.len() as i64)
}

fn part1<R: BufRead>(reader: R) -> Result<i64> {
    part1_general(reader, 100)
}

fn part2<R: BufRead>(reader: R) -> Result<i64> {
    let track = RaceTrack::from_reader(reader)?;
    let cheats = track.find_best_cheats(100, 20);
    Ok(cheats.len() as i64)
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

        fn test_part1(expect: i64, input: &str, min_saving: i64) {
            assert_eq!(expect, part1_general(BufReader::new(input.as_bytes()), min_saving).unwrap());
        }

        #[test]
        fn test1() {
            test_part1(
                1,
                indoc! {"
                    ###############
                    #...#...#.....#
                    #.#.#.#.#.###.#
                    #S#...#.#.#...#
                    #######.#.#.###
                    #######.#.#...#
                    #######.#.###.#
                    ###..E#...#...#
                    ###.#######.###
                    #...###...#...#
                    #.#####.#.###.#
                    #.#...#.#.#...#
                    #.#.#.#.#.#.###
                    #...#...#...###
                    ###############
                "},
                64
            );
        }

        #[test]
        fn test2() {
            test_part1(
                2,
                indoc! {"
                    ###############
                    #...#...#.....#
                    #.#.#.#.#.###.#
                    #S#...#.#.#...#
                    #######.#.#.###
                    #######.#.#...#
                    #######.#.###.#
                    ###..E#...#...#
                    ###.#######.###
                    #...###...#...#
                    #.#####.#.###.#
                    #.#...#.#.#...#
                    #.#.#.#.#.#.###
                    #...#...#...###
                    ###############
                "},
                40
            );
        }

        #[test]
        fn test3() {
            test_part1(
                5,
                indoc! {"
                    ###############
                    #...#...#.....#
                    #.#.#.#.#.###.#
                    #S#...#.#.#...#
                    #######.#.#.###
                    #######.#.#...#
                    #######.#.###.#
                    ###..E#...#...#
                    ###.#######.###
                    #...###...#...#
                    #.#####.#.###.#
                    #.#...#.#.#...#
                    #.#.#.#.#.#.###
                    #...#...#...###
                    ###############
                "},
                20
            );
        }

        #[test]
        fn test4() {
            test_part1(
                10,
                indoc! {"
                    ###############
                    #...#...#.....#
                    #.#.#.#.#.###.#
                    #S#...#.#.#...#
                    #######.#.#.###
                    #######.#.#...#
                    #######.#.###.#
                    ###..E#...#...#
                    ###.#######.###
                    #...###...#...#
                    #.#####.#.###.#
                    #.#...#.#.#...#
                    #.#.#.#.#.#.###
                    #...#...#...###
                    ###############
                "},
                10
            );
        }

        #[test]
        fn part1_final() {
            assert_eq!(1327, run_on_day_input(day!(), part1).unwrap());
        }
    }

    //noinspection SpellCheckingInspection
    #[cfg(test)]
    mod part2_tests {
        use itertools::Itertools;
        use super::*;

        fn test_cheats(saving: i64, expected_cheats: usize) {
            let input = indoc! {"
                ###############
                #...#...#.....#
                #.#.#.#.#.###.#
                #S#...#.#.#...#
                #######.#.#.###
                #######.#.#...#
                #######.#.###.#
                ###..E#...#...#
                ###.#######.###
                #...###...#...#
                #.#####.#.###.#
                #.#...#.#.#...#
                #.#.#.#.#.#.###
                #...#...#...###
                ###############
            "};

            let track = RaceTrack::from_reader(BufReader::new(input.as_bytes())).unwrap();
            let cheats = track.find_best_cheats(saving, 20);
            let filtered_cheats = cheats.iter().filter(|&route| route.savings == saving).collect_vec();
            println!("{}", filtered_cheats.iter().map(|r| format!("{}", r)).sorted().join("\n"));
            assert_eq!(expected_cheats, filtered_cheats.len());
        }

        #[test]
        fn cheat_only_once() {
            let input = indoc! {"
                #######
                #.....#
                #.#.#.#
                #S#.#E#
                #######
            "};

            let track = RaceTrack::from_reader(BufReader::new(input.as_bytes())).unwrap();
            let best_cheat_distance = track.find_best_cheats(4, 4);

            // Can't use cheats twice
            assert_eq!(best_cheat_distance.len(), 0);
        }

        #[test]
        fn no_optiomal_cheats() {
            let input = indoc! {"
                ######
                #....#
                #.##.#
                #S#E.#
                ######
                ######
            "};

            let track = RaceTrack::from_reader(BufReader::new(input.as_bytes())).unwrap();
            let best_cheat_distance = track.find_best_cheats(4, 3);
            println!("{}", best_cheat_distance.iter().map(|r| format!("{}", r)).sorted().join("\n"));

            // Can't use cheats twice
            assert_eq!(best_cheat_distance.len(), 2);
        }

        #[test]
        fn comparison_test() {
            let input = indoc! {"
                ###############
                #...#...#.....#
                #.#.#.#.#.###.#
                #S#...#.#.#...#
                #######.#.#.###
                #######.#.#...#
                #######.#.###.#
                ###..E#...#...#
                ###.#######.###
                #...###...#...#
                #.#####.#.###.#
                #.#...#.#.#...#
                #.#.#.#.#.#.###
                #...#...#...###
                ###############
            "};

            let track = RaceTrack::from_reader(BufReader::new(input.as_bytes())).unwrap();
            let reachable_by_walls: Vec<(Point, i64)> =
                track.find_reachable_by_walls(Point { row: 3, col: 1 }, 7).iter()
                .filter(|(_, s)| *s == 7)
                .map(|(p, s)| (*p, *s))
                .collect();
            let reachable_walkable_points: Vec<(Point, i64)> =
                track.find_all_reachable_points(Point { row: 3, col: 1 }, 7).iter()
                    .filter(|(p, _)| track.map[(p.row as usize, p.col as usize)].is_walkable())
                    .filter(|(_, s)| *s == 7)
                    .map(|(p, s)| (*p, *s))
                    .collect();

            assert_eq!(reachable_walkable_points.iter().map(|p| format!("{:?}", p)).sorted().join("\n"),
                       reachable_by_walls.iter().map(|p| format!("{:?}", p)).sorted().join("\n"));

        }

        #[test]
        fn test_long_way() {
            let input = indoc! {"
                ###############
                #...#...#.....#
                #.#.#.#.#.###.#
                #S#...#.#.#...#
                #######.#.#.###
                #######.#.#...#
                #######.#.###.#
                ###..E#...#...#
                ###.#######.###
                #...###...#...#
                #.#####.#.###.#
                #.#...#.#.#...#
                #.#.#.#.#.#.###
                #...#...#...###
                ###############
            "};

            let track = RaceTrack::from_reader(BufReader::new(input.as_bytes())).unwrap();
            let reachable_by_walls: Vec<(Point, i64)> =
                track.find_reachable_by_walls(Point { row: 1, col: 2 }, 20);
            println!("{}", reachable_by_walls.iter().map(|p| format!("{:?}", p)).sorted().join("\n"));
        }

        #[test]
        fn test_50() {
            test_cheats(50, 32);
        }

        #[test]
        fn test_70() {
            test_cheats(70, 12);
        }

        #[test]
        fn test_74() {
            test_cheats(74, 4);
        }

        #[test]
        fn test_76() {
            test_cheats(76, 3);
        }

        #[test]
        fn part2_final() {
            assert_eq!(985737, run_on_day_input(day!(), part2).unwrap());
        }
    }
}
