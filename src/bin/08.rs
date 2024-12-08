use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::BufRead;
use itertools::Itertools;


#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct UnsafePoint {
    x: i32,
    y: i32,
}

impl UnsafePoint {
    fn to_safe(&self, size: &Size) -> Option<Point> {
        if self.x >= 0 && self.x < size.x_size as i32 && self.y >= 0 && self.y < size.y_size as i32 {
            Some(Point { x: self.x as usize, y: self.y as usize })
        } else {
            None
        }
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn to_unsafe(&self) -> UnsafePoint {
        UnsafePoint { x: self.x as i32, y: self.y as i32 }
    }
}


#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Size {
    x_size: usize,
    y_size: usize,
}

fn parse_input<R: BufRead>(reader: R) -> (HashMap<char, Vec<Point>>, Size) {
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let mut x_size: Option<usize> = None;

    let lines: Vec<String> = reader.lines().flatten().collect();
    for (y, line) in lines.iter().enumerate() {
        if x_size.is_none() {
            x_size = Some(line.len())
        }
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                antennas.entry(ch).or_default().push(Point { x, y });
            }
        }
    }

    (
        antennas,
        Size {
            x_size: x_size.unwrap(),
            y_size: lines.len(),
        },
    )
}

fn find_antinodes(freq_antennas: &[Point], size: Size) -> HashSet<Point> {
    let mut antinodes = HashSet::new();

    for i in 0..freq_antennas.len() {
        for j in (i + 1)..freq_antennas.len() {
            let antenna1 = freq_antennas[i].to_unsafe();
            let antenna2 = freq_antennas[j].to_unsafe();

            let dx = antenna1.x - antenna2.x;
            let dy = antenna1.y - antenna2.y;

            antinodes.insert(UnsafePoint { x: antenna1.x + dx, y: antenna1.y + dy });
            antinodes.insert(UnsafePoint { x: antenna2.x - dx, y: antenna2.y - dy });
        }
    }

    antinodes
        .iter()
        .filter_map(|&(point)| point.to_safe(&size))
        .collect()
}

fn debug_print_antinodes(antinodes: &HashSet<Point>, size: &Size) {
    for y in 0..size.y_size {
        for x in 0..size.x_size {
            if antinodes.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let (antennas, size) = parse_input(reader);

    println!("Antennas:\n{}\n",
             antennas.iter().map(|entry| format!("{}: {:?}", entry.0, entry.1)).join("\n"));

    let mut total_antinodes = HashSet::new();
    for (ch, freq_antennas) in antennas {
        let freq_antinodes = find_antinodes(&freq_antennas, size);
        total_antinodes.extend(&freq_antinodes);

        println!("Frequency: {}", ch);
        debug_print_antinodes(&freq_antinodes, &size);
    }

    debug_print_antinodes(&total_antinodes, &size);

    Ok(total_antinodes.len() as i64)
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
    use super::*;
    use indoc::indoc;
    use std::io::BufReader;

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
                14,
                indoc! {"
                    ............
                    ........0...
                    .....0......
                    .......0....
                    ....0.......
                    ......A.....
                    ............
                    ............
                    ........A...
                    .........A..
                    ............
                    ............
                "},
            );
        }

        #[test]
        fn part1_final() {
            assert_eq!(423i64, run_on_day_input(day!(), part1).unwrap());
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
                34,
                indoc! {"
                    ............
                    ........0...
                    .....0......
                    .......0....
                    ....0.......
                    ......A.....
                    ............
                    ............
                    ........A...
                    .........A..
                    ............
                    ............
                "},
            );
        }

        #[test]
        fn part2_final() {
            part2_result().unwrap();
        }
    }
}
