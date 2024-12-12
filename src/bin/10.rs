use std::collections::{HashSet, VecDeque};
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};

fn parse_map<R: BufRead>(reader: R) -> Vec<Vec<u32>> {
    reader.lines()
        .flatten()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap_or(100)).collect())
        .collect()
}


//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let map = parse_map(reader);
    let result = solve_topographic_map(&map);

    Ok(result as i64)
}

fn solve_topographic_map(map: &Vec<Vec<u32>>) -> u32 {
    let rows = map.len();
    let cols = map[0].len();

    // Find all trailheads (positions with height 0)
    let trailheads: Vec<(usize, usize)> = (0..rows)
        .flat_map(|r| (0..cols).filter(move |&c| map[r][c] == 0).map(move |c| (r, c)))
        .collect();

    // Compute trailhead scores
    trailheads.iter()
        .map(|&trailhead| compute_trailhead_score(map, trailhead))
        .sum()
}

fn compute_trailhead_score(map: &Vec<Vec<u32>>, start: (usize, usize)) -> u32 {
    let rows = map.len();
    let cols = map[0].len();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start.0, start.1, 0));

    while let Some((r, c, current_height)) = queue.pop_front() {
        // Out of bounds or already visited
        if r >= rows || c >= cols || visited.contains(&(r, c)) {
            continue;
        }

        // Invalid height progression
        if map[r][c] != current_height {
            continue;
        }

        visited.insert((r, c));

        // Attempt moves in 4 directions
        let next_height = current_height + 1;
        let moves = [
            (r.wrapping_sub(1), c),    // Up
            (r + 1, c),                // Down
            (r, c.wrapping_sub(1)),    // Left
            (r, c + 1)                 // Right
        ];

        for (next_r, next_c) in moves {
            if next_r < rows && next_c < cols && map[next_r][next_c] == next_height {
                queue.push_back((next_r, next_c, next_height));
            }
        }
    }

    // Count 9-height positions reachable from this trailhead
    visited.iter()
        .filter(|&&(r, c)| map[r][c] == 9)
        .count() as u32
}

fn compute_trailhead_rating_all_paths(map: &Vec<Vec<u32>>, start: (usize, usize)) -> u32 {
    let rows = map.len();
    let cols = map[0].len();
    let mut trails_to_peaks = HashSet::new();

    // Queue stores: (row, col, current_height, current_trail)
    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, 0, vec![(start.0, start.1)]));

    while let Some((r, c, current_height, current_trail)) = queue.pop_front() {
        // Out of bounds
        if r >= rows || c >= cols {
            continue;
        }

        // Invalid height progression
        if map[r][c] != current_height {
            continue;
        }

        // Reached peak
        if current_height == 9 {
            trails_to_peaks.insert(current_trail.clone());
            continue;
        }

        // Attempt moves in 4 directions
        let next_height = current_height + 1;
        let moves = [
            (r.wrapping_sub(1), c),    // Up
            (r + 1, c),                // Down
            (r, c.wrapping_sub(1)),    // Left
            (r, c + 1)                 // Right
        ];

        for (next_r, next_c) in moves {
            if next_r < rows && next_c < cols && map[next_r][next_c] == next_height {
                let mut new_trail = current_trail.clone();
                new_trail.push((next_r, next_c));
                queue.push_back((next_r, next_c, next_height, new_trail));
            }
        }
    }

    // Return number of distinct trails to peaks
    trails_to_peaks.len() as u32
}

fn solve_topographic_map_2(map: &Vec<Vec<u32>>) -> u32 {
    let rows = map.len();
    let cols = map[0].len();

    // Find all trailheads (positions with height 0)
    let trailheads: Vec<(usize, usize)> = (0..rows)
        .flat_map(|r| (0..cols).filter(move |&c| map[r][c] == 0).map(move |c| (r, c)))
        .collect();

    // Compute trailhead ratings
    trailheads.iter()
        .map(|&trailhead| compute_trailhead_rating_all_paths(map, trailhead))
        .sum()
}

//noinspection DuplicatedCode
fn part2<R: BufRead>(reader: R) -> Result<i64> {
    let map = parse_map(reader);
    let result = solve_topographic_map_2(&map);

    Ok(result as i64)
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
                1,
                indoc! {"
                    0123
                    1234
                    8765
                    9876
                "},
            );
        }

        #[test]
        fn test2() {
            test_part1(
                2,
                indoc! {"
                    ...0...
                    ...1...
                    ...2...
                    6543456
                    7.....7
                    8.....8
                    9.....9
                "},
            );
        }

        #[test]
        fn test3() {
            test_part1(
                3,
                indoc! {"
                    10..9..
                    2...8..
                    3...7..
                    4567654
                    ...8..3
                    ...9..2
                    .....01
                "},
            );
        }

        #[test]
        fn test4() {
            test_part1(
                36,
                indoc! {"
                    89010123
                    78121874
                    87430965
                    96549874
                    45678903
                    32019012
                    01329801
                    10456732
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
                3,
                indoc! {"
                .....0.
                ..4321.
                ..5..2.
                ..6543.
                ..7..4.
                ..8765.
                ..9....
                "}
            );
        }

        #[test]
        fn test2() {
            test_part2(
                13,
                indoc! {"
                ..90..9
                ...1.98
                ...2..7
                6543456
                765.987
                876....
                987....
                "}
            );
        }

        #[test]
        fn test3() {
            test_part2(
                227,
                indoc! {"
                012345
                123456
                234567
                345678
                4.6789
                56789.
                "}
            );
        }

        #[test]
        fn test4() {
            test_part2(
                81,
                indoc! {"
                89010123
                78121874
                87430965
                96549874
                45678903
                32019012
                01329801
                10456732
                "}
            );
        }

        #[test]
        fn part2_final() {
            part2_result().unwrap();
        }
    }
}
