use std::collections::{HashMap, VecDeque};
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};

#[derive(Debug, Clone, Copy)]
enum DirectionKeyboardAction {
    Up,
    Right,
    Down,
    Left,
    Press
}

const MOVE_ACTIONS: [DirectionKeyboardAction; 4] = [
    DirectionKeyboardAction::Up,
    DirectionKeyboardAction::Right,
    DirectionKeyboardAction::Down,
    DirectionKeyboardAction::Left
];

impl DirectionKeyboardAction {
    fn to_index(self) -> usize {
        match self {
            DirectionKeyboardAction::Up => 0,
            DirectionKeyboardAction::Right => 1,
            DirectionKeyboardAction::Down => 2,
            DirectionKeyboardAction::Left => 3,
            DirectionKeyboardAction::Press => 4,
        }
    }

    fn from_char(c: &char) -> Self {
        match c {
            '^' => DirectionKeyboardAction::Up,
            '>' => DirectionKeyboardAction::Right,
            'v' => DirectionKeyboardAction::Down,
            '<' => DirectionKeyboardAction::Left,
            'A' => DirectionKeyboardAction::Press,
            _ => panic!("Invalid char: {}", c)
        }
    }
}

#[derive(Debug)]
struct DirectionKeyboardCostMatrix {
    matrix: [[usize; 5]; 5],
}

impl DirectionKeyboardCostMatrix {
    fn new() -> Self {
        Self {
            matrix: [[0; 5]; 5]
        }
    }

    fn get(&self, from: DirectionKeyboardAction, to: DirectionKeyboardAction) -> usize {
        self.matrix[from.to_index()][to.to_index()]
    }

    fn set(&mut self, from: DirectionKeyboardAction, to: DirectionKeyboardAction, cost: usize) {
        self.matrix[from.to_index()][to.to_index()] = cost;
    }
}

const MANUAL_KEYBOARD_COST: DirectionKeyboardCostMatrix = DirectionKeyboardCostMatrix {
    matrix: [[1; 5]; 5]
};

// Point represents a position on any keypad
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct NumericKeypad {
    transition_cost: HashMap<(char, char), usize>
}

impl NumericKeypad {
    /*
    +---+---+---+
    | 7 | 8 | 9 |
    +---+---+---+
    | 4 | 5 | 6 |
    +---+---+---+
    | 1 | 2 | 3 |
    +---+---+---+
        | 0 | A |
        +---+---+
     */
    fn numeric_keypad(control: &DirectionKeyboardCostMatrix) -> Self {
        let buttons =
            [
                ((0, 0), '7'), ((1, 0), '8'), ((2, 0), '9'),
                ((0, 1), '4'), ((1, 1), '5'), ((2, 1), '6'),
                ((0, 2), '1'), ((1, 2), '2'), ((2, 2), '3'),
                /* --------- */((1, 3), '0'), ((2, 3), 'A'),
            ]
            .into_iter()
            .map(|((x, y), val)| (Point { x, y }, val))
            .collect();

        let cost_matrix = build_cost_matrix(&buttons, control);

        Self {
            transition_cost: cost_matrix
        }
    }
}


// Represents different types of keypads
#[derive(Debug)]
struct DirectionKeypad {
    press_cost: DirectionKeyboardCostMatrix
}

impl DirectionKeypad {
    /*
        +---+---+
        | ^ | A |
    +---+---+---+
    | < | v | > |
    +---+---+---+
     */
    fn remote_directional_keypad(control: &DirectionKeyboardCostMatrix) -> Self {
        let buttons: HashMap<Point, char> =
            [
                /* --------- */((1, 0), '^'), ((2, 0), 'A'),
                ((0, 1), '<'), ((1, 1), 'v'), ((2, 1), '>'),
            ]
            .into_iter()
            .map(|((x, y), val)| (Point { x, y }, val))
            .collect();

        let press_costs = build_cost_matrix(&buttons, control);
        let mut action_costs = DirectionKeyboardCostMatrix::new();
        for ((from, to), cost) in press_costs.iter() {
            let from_action = DirectionKeyboardAction::from_char(from);
            let to_action = DirectionKeyboardAction::from_char(to);
            action_costs.set(from_action, to_action, *cost);
        }

        Self {
            press_cost: action_costs
        }
    }
}

fn move_direction(pos: &Point, dir: &DirectionKeyboardAction) -> Point {
    match dir {
        DirectionKeyboardAction::Up => Point { x: pos.x, y: pos.y - 1 },
        DirectionKeyboardAction::Down => Point { x: pos.x, y: pos.y + 1 },
        DirectionKeyboardAction::Left => Point { x: pos.x - 1, y: pos.y },
        DirectionKeyboardAction::Right => Point { x: pos.x + 1, y: pos.y },
        _ => panic!("Shouldn't be called for {:?}", dir)
    }
}

fn count_path_cost<T>(
    path: &Vec<(T, DirectionKeyboardAction)>,
    cost_matrix: &DirectionKeyboardCostMatrix) -> usize
{
    let mut sum: usize = 0;
    for i in 0..path.len() {
        let action = path[i].1;
        if i == path.len() - 1 {
            sum += cost_matrix.get(action, DirectionKeyboardAction::Press);
        } else {
            sum += cost_matrix.get(action, path[i + 1].1);
        }
    }

    sum
}

fn build_cost_matrix(buttons: &HashMap<Point, char>, cost: &DirectionKeyboardCostMatrix) -> HashMap<(char, char), usize> {
    let mut cost_matrix: HashMap<(char, char), usize> = HashMap::new();

    for (from_point, from_char) in buttons.iter() {
        let mut visited_path_length: HashMap<Point, usize> = HashMap::new();

        let mut paths_queue: VecDeque<Vec<((Point, char), DirectionKeyboardAction)>> = VecDeque::new();
        paths_queue.push_back(vec![((from_point.clone(), *from_char), DirectionKeyboardAction::Press)]);
        visited_path_length.insert(from_point.clone(), 0);

        while let Some(path) = paths_queue.pop_front() {
            let (last_point, to_char) = path.last().unwrap().0;
            let from_to_path = (*from_char, to_char);

            let cost = count_path_cost(&path, &cost);
            let known_cost = cost_matrix.get(&from_to_path).unwrap_or(&usize::MAX);
            if cost < *known_cost {
                cost_matrix.insert(from_to_path, cost);
                visited_path_length.insert(last_point.clone(), path.len());
            }

            for action in MOVE_ACTIONS.iter() {
                let next = move_direction(&last_point, action);
                let Some(to_char) = buttons.get(&next) else { continue }; // No such button
                if let Some(known_path_length) = visited_path_length.get(&next) {
                    if *known_path_length < path.len() + 1 {
                        // There's known path that is shorter - don't add a new one
                        continue;
                    }
                }

                let mut new_path = path.clone();
                new_path.push(((next, *to_char), action.clone()));
                paths_queue.push_back(new_path);
            }
        }
    }

    cost_matrix
}

fn key_cost(code: &str, numeric_keypad: &NumericKeypad) -> i64 {
    let mut cost = 0;
    let mut previous = 'A';

    for c in code.chars() {
        cost += numeric_keypad.transition_cost.get(&(previous, c)).unwrap();
        previous = c;
    }

    cost as i64
}

fn code_cost(code: &str, numeric_keypad: &NumericKeypad) -> i64 {
    let num = code.trim_end_matches('A').parse::<i64>().unwrap();
    let cost = key_cost(code, numeric_keypad);
    num * cost
}

fn parse_input<R: BufRead>(reader: R) -> Vec<String> {
    reader.lines()
        .flatten()
        .filter(|l| !l.is_empty())
        .collect()
}

fn create_numpad_keyboard(number_of_robot_direction_keypads: usize) -> NumericKeypad {
    let mut current_robot_keypad =
        DirectionKeypad::remote_directional_keypad(&MANUAL_KEYBOARD_COST);

    for _ in 2..=number_of_robot_direction_keypads {
        current_robot_keypad = DirectionKeypad::remote_directional_keypad(&current_robot_keypad.press_cost);
    }

    NumericKeypad::numeric_keypad(&current_robot_keypad.press_cost)
}

fn create_numpad_keyboard_part1() -> NumericKeypad {
    create_numpad_keyboard(2)
}

fn create_numpad_keyboard_part2() -> NumericKeypad {
    create_numpad_keyboard(25)
}


//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let codes = parse_input(reader);
    let numpad = create_numpad_keyboard_part1();

    Ok(codes.iter().map(|code| code_cost(code, &numpad)).sum())
}

//noinspection DuplicatedCode
fn part2<R: BufRead>(reader: R) -> Result<i64> {
    let codes = parse_input(reader);
    let numpad = create_numpad_keyboard_part2();

    Ok(codes.iter().map(|code| code_cost(code, &numpad)).sum())
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

        fn test_key_cost(expect: i64, code: &str) {
            let numpad = create_numpad_keyboard_part1();
            assert_eq!(expect, key_cost(code, &numpad));
        }

        #[test]
        fn test1() {
            test_part1(
                126384,
                indoc! {"
                    029A
                    980A
                    179A
                    456A
                    379A
                "},
            );
        }

        #[test]
        fn test_029a() {
            test_key_cost(68, "029A")
        }
        #[test]
        fn test_980a() {
            test_key_cost(60, "980A")
        }
        #[test]
        fn test_179a() {
            test_key_cost(68, "179A")
        }
        #[test]
        fn test_456a() {
            test_key_cost(64, "456A")
        }
        #[test]
        fn test_379a() {
            test_key_cost(64, "379A")
        }

        #[test]
        fn part1_final() {
            assert_eq!(162740, run_on_day_input(day!(), part1).unwrap());
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
        fn part2_final() {
            assert_eq!(203640915832208, run_on_day_input(day!(), part2).unwrap());
        }
    }
}
