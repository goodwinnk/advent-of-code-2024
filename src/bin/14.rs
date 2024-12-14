use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};

struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}
fn parse_input<R: BufRead>(reader: R) -> Vec<Robot> {
    let mut robots = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<_> = line.split_whitespace().collect();
        assert_eq!(parts.len(), 2);

        let pos = parts[0]
            .strip_prefix("p=")
            .and_then(|s| s.split_once(','))
            .and_then(|(x, y)|
                Some((x.parse::<i32>().ok()?, y.parse::<i32>().ok()?))
            ).unwrap();
        let vel = parts[1]
            .strip_prefix("v=")
            .and_then(|s| s.split_once(','))
            .and_then(|(x, y)|
                Some((x.parse::<i32>().ok()?, y.parse::<i32>().ok()?))
            ).unwrap();

        robots.push(Robot { pos, vel });
    }

    robots
}


fn part1<R: BufRead>(reader: R) -> Result<i64> {
    part1_ext(reader, 101, 103)
}

//noinspection DuplicatedCode
fn part1_ext<R: BufRead>(reader: R, width: i32, height: i32) -> Result<i64> {
    let robots = parse_input(reader);

    let final_positions: Vec<(i32, i32)> = robots
        .into_iter()
        .map(|robot: Robot| {
            (
                (robot.pos.0 + 100 * robot.vel.0).rem_euclid(width),
                (robot.pos.1 + 100 * robot.vel.1).rem_euclid(height),
            )
        })
        .collect();

    // Count robots in each quadrant
    let mut quadrants = [0; 4];
    let mid_x = width / 2;
    let mid_y = height / 2;
    for (x, y) in final_positions {
        if x == mid_x || y == mid_y {
            continue; // Skip robots on the middle grid lines
        }
        match (x < mid_x, y < mid_y) {
            (true, true) => quadrants[0] += 1,  // Top-left
            (false, true) => quadrants[1] += 1, // Top-right
            (true, false) => quadrants[2] += 1, // Bottom-left
            (false, false) => quadrants[3] += 1, // Bottom-right
        }
    }

    // Calculate safety factor
    Ok(quadrants.iter().product())
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

        fn test_part1(expect: i64, width: i32, height: i32, input: &str) {
            assert_eq!(expect, part1_ext(BufReader::new(input.as_bytes()), width, height).unwrap());
        }

        #[test]
        fn test1() {
            test_part1(
                12,
                11, 7,
                indoc! {"
                    p=0,4 v=3,-3
                    p=6,3 v=-1,-3
                    p=10,3 v=-1,2
                    p=2,0 v=2,-1
                    p=0,0 v=1,3
                    p=3,0 v=-2,-2
                    p=7,6 v=-1,-3
                    p=3,0 v=-1,-2
                    p=9,3 v=2,3
                    p=7,3 v=-1,2
                    p=2,4 v=2,-3
                    p=9,5 v=-3,-3
                "},
            );
        }

        #[test]
        fn part1_final() {
            assert_eq!(231852216, run_on_day_input(day!(), part1).unwrap());
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
