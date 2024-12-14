use std::collections::{HashMap, HashSet, VecDeque};
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Region {
    plots: Vec<(usize, usize)>,
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }

    fn count_sides(&self, map: &Vec<Vec<char>>) -> usize {
        let plant_type = map[self.plots[0].0][self.plots[0].1];
        let rows = map.len() as i32;
        let cols = map[0].len() as i32;

        #[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
        enum WallDirection {
            UP,
            RIGHT,
            DOWN,
            LEFT,
        }

        let mut sides: HashMap<(WallDirection, usize), Vec<usize>> = HashMap::new();

        let is_other = |row: usize, col: usize, dr: i32, dc: i32| -> bool {
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;
            nr < 0 || nc < 0 || nr >= rows || nc >= cols || map[nr as usize][nc as usize] != plant_type
        };

        for &(r, c) in &self.plots {
            if is_other(r, c, -1, 0) {
                sides.entry((WallDirection::UP, r)).or_default().push(c);
            }

            if is_other(r, c, 1, 0) {
                sides.entry((WallDirection::DOWN, r + 1)).or_default().push(c);
            }

            if is_other(r, c, 0, -1) {
                sides.entry((WallDirection::LEFT, c)).or_default().push(r);
            }

            if is_other(r, c, 0, 1) {
                sides.entry((WallDirection::RIGHT, c + 1)).or_default().push(r);
            }
        }

        let sides_count = sides.values().map(|v| {
            assert_ne!(v.len(), 0);
            let sorted_sides: Vec<usize> = v.iter().sorted().copied().collect();
            let mut count = 1;
            for i in 1..sorted_sides.len() {
                if sorted_sides[i] != sorted_sides[i - 1] + 1 {
                    count += 1;
                }
            }
            count
        }).sum();

        // println!(
        //     "Region: {}, Sides count: {}.\n  Plots: {:?}\n  Sides: {:?}",
        //     plant_type, sides_count, self.plots, sides);

        sides_count
    }
}


fn find_regions(map: &Vec<Vec<char>>) -> Vec<(char, Region)> {
    let rows = map.len();
    let cols = map[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut regions: Vec<(char, Region)> = Default::default();

    for r in 0..rows {
        for c in 0..cols {
            if visited[r][c] {
                continue;
            }

            let plant_type = map[r][c];
            let region = bfs_map(map, &mut visited, r, c, plant_type);

            regions.push((plant_type, Region { plots: region }));
        }
    }

    regions
}

fn bfs_map(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    start_row: usize,
    start_column: usize,
    plant_type: char
) -> Vec<(usize, usize)> {
    let mut region: Vec<(usize, usize)> = Vec::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let rows = map.len();
    let cols = map[0].len();

    queue.push_back((start_row, start_column));
    while let Some((row, column)) = queue.pop_front() {
        if row >= rows || column >= cols ||
            visited[row][column] || map[row][column] != plant_type {
            continue;
        }

        visited[row][column] = true;
        region.push((row, column));

        for (dr, dc) in directions {
            let nr = row as i32 + dr;
            let nc = column as i32 + dc;
            if nr >= 0 && nc >= 0 {
                queue.push_back((nr as usize, nc as usize));
            }
        }
    }

    region
}

//noinspection DuplicatedCode
fn part1<R: BufRead>(_reader: R) -> Result<i64> {
    Ok(0)
}

//noinspection DuplicatedCode
fn part2<R: BufRead>(reader: R) -> Result<i64> {
    let map: Vec<Vec<char>> = reader
        .lines()
        .flatten()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let regions = find_regions(&map);
    println!("{}", regions.len());

    Ok(regions.iter()
        .map(|(_, region)|
            (region.area() * region.count_sides(&map)) as i64
        )
        .sum())
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
                0,
                indoc! {"
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
            test_part2(
                80,
                indoc! {"
                    AAAA
                    BBCD
                    BBCC
                    EEEC
                "}
            );
        }

        #[test]
        fn test2() {
            test_part2(
                436,
                indoc! {"
                    OOOOO
                    OXOXO
                    OOOOO
                    OXOXO
                    OOOOO
                "}
            );
        }

        #[test]
        fn test3() {
            test_part2(
                236,
                indoc! {"
                    EEEEE
                    EXXXX
                    EEEEE
                    EXXXX
                    EEEEE
                "}
            );
        }

        #[test]
        fn test4() {
            test_part2(
                368,
                indoc! {"
                    AAAAAA
                    AAABBA
                    AAABBA
                    ABBAAA
                    ABBAAA
                    AAAAAA
                "}
            );
        }

        #[test]
        fn test5() {
            test_part2(
                1206,
                indoc! {"
                    RRRRIICCFF
                    RRRRIICCCF
                    VVRRRCCFFF
                    VVRCCCJFFF
                    VVVVCJJCFE
                    VVIVCCJJEE
                    VVIIICJJEE
                    MIIIIIJJEE
                    MIIISIJEEE
                    MMMISSJEEE
                "}
            );
        }

        #[test]
        fn part2_final() {
            fn part2_final() {
                assert_eq!(911750, run_on_day_input(day!(), part2).unwrap());
            }
        }
    }
}
