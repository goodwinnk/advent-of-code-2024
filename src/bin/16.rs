use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};
use array2d::Array2D;
use advent_of_code2024_rust::matrix::{Array2DExt, Coordinate, Direction};

fn parse_input<R: BufRead>(reader: R) -> Array2D<char> {
    let rows: Vec<Vec<char>> = reader
        .lines()
        .flatten()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().chars().collect())
        .collect();

    Array2D::from_rows(&rows).unwrap()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Node {
    position: Coordinate,
    direction: Direction
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct State {
    node: Node,
    previous: Option<Node>,
    cost: u64
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_position(maze: &Array2D<char>, target: char) -> Coordinate {
    for (y, row) in maze.rows_iter().enumerate() {
        for (x, &cell) in row.enumerate() {
            if cell == target {
                return Coordinate {
                    row: y as isize,
                    column: x as isize,
                };
            }
        }
    }

    panic!("Position not found!")
}

fn can_move(
    maze: &Array2D<char>,
    current: Coordinate,
    direction: Direction
) -> Option<Coordinate> {
    let d = direction.to_offset();
    let next = current + d;
    if let Some(&char) = maze.get_safe(&next) {
        if char != '#' {
            return Some(next);
        }
    }
    None
}

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<u64> {
    let maze = parse_input(reader);

    let start = find_position(&maze, 'S');
    let end = find_position(&maze, 'E');

    let directions = [
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up
    ];

    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();

    heap.push(State {
        node: Node {
            position: start,
            direction: Direction::Right,
        },
        previous: None,
        cost: 0
    });

    while let Some(current) = heap.pop() {
        if current.node.position == end {
            return Ok(current.cost);
        }

        if visited.contains(&current.node) {
            continue;
        }
        visited.insert(current.node);

        for &next_direction in &directions {
            if next_direction.opposite_direction() == current.node.direction {
                continue
            }

            if let Some(next_pos) = can_move(&maze, current.node.position, next_direction) {
                let turn_cost = if next_direction != current.node.direction { 1000 } else { 0 };
                let next_state = State {
                    node: Node {
                        position: next_pos,
                        direction: next_direction,
                    },
                    previous: None,
                    cost: current.cost + 1 + turn_cost
                };

                heap.push(next_state);
            }
        }
    }

    Err(anyhow!("No path found!"))
}

//noinspection DuplicatedCode
fn part2<R: BufRead>(reader: R) -> Result<u64> {
    let maze = parse_input(reader);

    let start = find_position(&maze, 'S');
    let end = find_position(&maze, 'E');

    let directions = [
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up
    ];

    let mut distances: HashMap<Node, (u64, Vec<Node>)> = HashMap::new();
    let mut best_states = BinaryHeap::new();

    let start_node = Node {
        position: start,
        direction: Direction::Right,
    };
    best_states.push(State {
        node: start_node.clone(),
        previous: None,
        cost: 0
    });

    let mut min_distance_to_end = None;

    while let Some(current) = best_states.pop() {
        if min_distance_to_end.is_some() {
            if current.cost > min_distance_to_end.unwrap() {
                // Any other path is worse than this one
                break;
            };
        }

        if let Some((distance, previous)) = distances.get_mut(&current.node) {
            if current.cost > *distance {
                continue;
            } else if current.cost == *distance {
                previous.push(current.previous.unwrap());
                continue;
            } else {
                panic!("Shouldn't be less because of using BinaryHeap! \
                    node: {:?}, cost: {}, distance: {}", current.node, current.cost, *distance);
            }
        }

        if current.node != start_node {
            distances.insert(current.node, (current.cost, vec![current.previous.unwrap()]));
        } else {
            distances.insert(current.node, (0, vec![]));
        }

        if current.node.position == end {
            min_distance_to_end = Some(current.cost);
            continue;
        }

        for &next_direction in &directions {
            if next_direction.opposite_direction() == current.node.direction {
                continue
            }

            if let Some(next_pos) = can_move(&maze, current.node.position, next_direction) {
                let turn_cost = if next_direction != current.node.direction { 1000 } else { 0 };
                let next_state = State {
                    node: Node {
                        position: next_pos,
                        direction: next_direction,
                    },
                    previous: Some(current.node),
                    cost: current.cost + 1 + turn_cost
                };

                best_states.push(next_state);
            }
        }
    }

    let mut optimal_path_coordinates: HashSet<Coordinate> = HashSet::default();
    let mut path_traverse: VecDeque<Node> = VecDeque::default();

    optimal_path_coordinates.insert(end);
    for direction in directions.iter() {
        if let Some((_, nodes)) = distances.get(&Node { position: end, direction: *direction }) {
            path_traverse.extend(nodes.iter());
        };
    }
    while let Some(node) = path_traverse.pop_front() {
        optimal_path_coordinates.insert(node.position);
        path_traverse.extend(distances[&node].1.iter());
    }

    Ok(optimal_path_coordinates.len() as u64)
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

        fn test_part1(expect: u64, input: &str) {
            assert_eq!(expect, part1(BufReader::new(input.as_bytes())).unwrap());
        }

        #[test]
        fn test1() {
            test_part1(
                7036,
                indoc! {"
                    ###############
                    #.......#....E#
                    #.#.###.#.###.#
                    #.....#.#...#.#
                    #.###.#####.#.#
                    #.#.#.......#.#
                    #.#.#####.###.#
                    #...........#.#
                    ###.#.#####.#.#
                    #...#.....#.#.#
                    #.#.#.###.#.#.#
                    #.....#...#.#.#
                    #.###.#.#.#.#.#
                    #S..#.....#...#
                    ###############
                "},
            );
        }

        #[test]
        fn test2() {
            test_part1(
                11048,
                indoc! {"
                    #################
                    #...#...#...#..E#
                    #.#.#.#.#.#.#.#.#
                    #.#.#.#...#...#.#
                    #.#.#.#.###.#.#.#
                    #...#.#.#.....#.#
                    #.#.#.#.#.#####.#
                    #.#...#.#.#.....#
                    #.#.#####.#.###.#
                    #.#.#.......#...#
                    #.#.###.#####.###
                    #.#.#...#.....#.#
                    #.#.#.#####.###.#
                    #.#.#.........#.#
                    #.#.#.#########.#
                    #S#.............#
                    #################
                "}
            )
        }

        #[test]
        fn test3() {
            test_part1(
                3004,
                indoc! {"
                    #####
                    #...#
                    #S#E#
                    #####
                "}
            )
        }

        #[test]
        fn part1_final() {
            assert_eq!(107468, run_on_day_input(day!(), part1).unwrap());
        }
    }

    //noinspection SpellCheckingInspection
    #[cfg(test)]
    mod part2_tests {
        use super::*;

        fn test_part2(expect: u64, input: &str) {
            assert_eq!(expect, part2(BufReader::new(input.as_bytes())).unwrap());
        }

        #[test]
        fn test1() {
            test_part2(
                45,
                indoc! {"
                ###############
                #.......#....E#
                #.#.###.#.###.#
                #.....#.#...#.#
                #.###.#####.#.#
                #.#.#.......#.#
                #.#.#####.###.#
                #...........#.#
                ###.#.#####.#.#
                #...#.....#.#.#
                #.#.#.###.#.#.#
                #.....#...#.#.#
                #.###.#.#.#.#.#
                #S..#.....#...#
                ###############
            "}
            );
        }

        #[test]
        fn test2() {
            test_part2(
                64,
                indoc! {"
                #################
                #...#...#...#..E#
                #.#.#.#.#.#.#.#.#
                #.#.#.#...#...#.#
                #.#.#.#.###.#.#.#
                #...#.#.#.....#.#
                #.#.#.#.#.#####.#
                #.#...#.#.#.....#
                #.#.#####.#.###.#
                #.#.#.......#...#
                #.#.###.#####.###
                #.#.#...#.....#.#
                #.#.#.#####.###.#
                #.#.#.........#.#
                #.#.#.#########.#
                #S#.............#
                #################
            "}
            );
        }

        #[test]
        fn part2_final() {
            assert_eq!(533, run_on_day_input(day!(), part2).unwrap());
        }
    }
}
