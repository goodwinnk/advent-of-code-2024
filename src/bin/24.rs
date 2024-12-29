use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Gate {
    And,
    Or,
    Xor,
}

impl Display for Gate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Gate::And => write!(f, "AND"),
            Gate::Or => write!(f, "OR"),
            Gate::Xor => write!(f, "XOR"),
        }
    }
}

#[derive(Debug, Clone)]
struct Connection {
    gate: Gate,
    input1: String,
    input2: String,
    output: String,
}

impl Display for Connection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} <- {} {} {} ",
            self.output, self.input1, self.gate, self.input2
        )
    }
}

#[derive(Debug)]
struct Circuit {
    initial_values: HashMap<String, bool>,
    connections: Vec<Connection>,
}

fn parse_input<R: BufRead>(reader: R) -> Result<Circuit> {
    let initial_value_re = Regex::new(r"^([a-z0-9]+):\s*(\d+)$")?;
    let connection_re = Regex::new(r"^([a-z0-9]+)\s+(AND|OR|XOR)\s+([a-z0-9]+)\s+->\s+([a-z0-9]+)$")?;

    let mut initial_values = HashMap::new();
    let mut connections = Vec::new();
    let mut reading_connections = false;

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            reading_connections = true;
            continue;
        }

        if !reading_connections {
            if let Some(captures) = initial_value_re.captures(&line) {
                let wire = captures[1].to_string();
                let value = captures[2].parse::<i32>()? != 0;
                initial_values.insert(wire, value);
            }
        } else {
            if let Some(captures) = connection_re.captures(&line) {
                let input1 = captures[1].to_string();
                let gate = match &captures[2] {
                    "AND" => Gate::And,
                    "OR" => Gate::Or,
                    "XOR" => Gate::Xor,
                    _ => continue,
                };
                let input2 = captures[3].to_string();
                let output = captures[4].to_string();

                connections.push(Connection {
                    gate,
                    input1,
                    input2,
                    output,
                });
            }
        }
    }

    Ok(Circuit {
        initial_values,
        connections,
    })
}

fn generate_dependency_map(connections: &Vec<Connection>) -> Result<HashMap<String, HashSet<String>>> {
    let connection_map: HashMap<String, &Connection> = connections.iter()
        .map(|conn| (conn.output.clone(), conn))
        .collect();

    let mut dependency_map: HashMap<String, HashSet<String>> = HashMap::new();

    fn trace_dependencies(
        wire: &str,
        connection_map: &HashMap<String, &Connection>,
        visited: &mut HashSet<String>,
    ) -> HashSet<String> {
        if visited.contains(wire) {
            return HashSet::default();
        }

        visited.insert(wire.to_string());

        match connection_map.get(wire) {
            Some(connection) => {
                let mut dependencies = HashSet::new();
                dependencies.insert(connection.output.clone());
                dependencies.extend(trace_dependencies(&connection.input1, connection_map, visited));
                dependencies.extend(trace_dependencies(&connection.input2, connection_map, visited));

                dependencies
            }
            None => {
                HashSet::from([wire.to_string()])
            }
        }
    }

    for connection in connections.iter() {
        let mut visited = HashSet::new();
        let dependencies = trace_dependencies(&connection.output, &connection_map, &mut visited);
        dependency_map.insert(connection.output.clone(), dependencies);
    }

    Ok(dependency_map)
}


fn simulate_circuit_recursive(circuit: &Circuit, i: usize) -> Result<(i64, HashMap<String, bool>)> {
    let mut wire_values = circuit.initial_values.clone();

    let connection_map: HashMap<String, &Connection> = circuit.connections.iter()
        .map(|conn| (conn.output.clone(), conn))
        .collect();

    // Recursive function to compute the value of a wire
    fn resolve_wire(
        wire: &str,
        wire_values: &mut HashMap<String, bool>,
        connection_map: &HashMap<String, &Connection>,
        visited: &mut HashSet<String>,
    ) -> Result<bool> {
        // If the value of the wire is already known, return it
        if let Some(&value) = wire_values.get(wire) {
            return Ok(value);
        }

        if visited.contains(wire) {
            bail!("Recursion in {}", wire);
        }

        visited.insert(wire.to_string());

        // Otherwise, find the connection that produces this wire
        let connection = connection_map.get(wire);
        if let Some(connection) = connection {
            let input1 = resolve_wire(&connection.input1, wire_values, connection_map, visited)?;
            let input2 = resolve_wire(&connection.input2, wire_values, connection_map, visited)?;
            let result = match connection.gate {
                Gate::And => input1 && input2,
                Gate::Or => input1 || input2,
                Gate::Xor => input1 != input2,
            };
            wire_values.insert(wire.to_string(), result); // Cache the computed value
            Ok(result)
        } else {
            bail!("Wire \"{}\" has no input or initial value!", wire);
        }
    }

    let mut number: i64 = 0;
    for i in 0..=i {
        let key = format!("z{:02}", i);
        let mut visited = HashSet::new();
        if connection_map.contains_key(&key) {
            let value = resolve_wire(&key, &mut wire_values, &connection_map, &mut visited)?;
            number |= if value { 1 << i } else { 0 };
        } else {
            break;
        }
    }

    Ok((number, wire_values))
}

fn simulate_circuit(circuit: &Circuit) -> Result<(i64, HashMap<String, bool>)> {
    simulate_circuit_recursive(circuit, 64)
}

fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let circuit = parse_input(reader)?;
    simulate_circuit(&circuit).map(|(result, _)| result)
}

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

    #[cfg(test)]
    mod part1_tests {
        use super::*;

        fn test_part1(expect: i64, input: &str) {
            assert_eq!(expect, part1(BufReader::new(input.as_bytes())).unwrap());
        }

        #[test]
        fn test_small() {
            test_part1(
                4,
                indoc! {"
                    x00: 1
                    x01: 1
                    x02: 1
                    y00: 0
                    y01: 1
                    y02: 0

                    x00 AND y00 -> z00
                    x01 XOR y01 -> z01
                    x02 OR y02 -> z02
                "},
            );
        }

        #[test]
        fn test_large() {
            test_part1(
                2024,
                indoc! {"
                    x00: 1
                    x01: 0
                    x02: 1
                    x03: 1
                    x04: 0
                    y00: 1
                    y01: 1
                    y02: 1
                    y03: 1
                    y04: 1

                    ntg XOR fgs -> mjb
                    y02 OR x01 -> tnw
                    kwq OR kpj -> z05
                    x00 OR x03 -> fst
                    tgd XOR rvg -> z01
                    vdt OR tnw -> bfw
                    bfw AND frj -> z10
                    ffh OR nrd -> bqk
                    y00 AND y03 -> djm
                    y03 OR y00 -> psh
                    bqk OR frj -> z08
                    tnw OR fst -> frj
                    gnj AND tgd -> z11
                    bfw XOR mjb -> z00
                    x03 OR x00 -> vdt
                    gnj AND wpb -> z02
                    x04 AND y00 -> kjc
                    djm OR pbm -> qhw
                    nrd AND vdt -> hwm
                    kjc AND fst -> rvg
                    y04 OR y02 -> fgs
                    y01 AND x02 -> pbm
                    ntg OR kjc -> kwq
                    psh XOR fgs -> tgd
                    qhw XOR tgd -> z09
                    pbm OR djm -> kpj
                    x03 XOR y03 -> ffh
                    x00 XOR y04 -> ntg
                    bfw OR bqk -> z06
                    nrd XOR fgs -> wpb
                    frj XOR qhw -> z04
                    bqk OR frj -> z07
                    y03 OR x01 -> nrd
                    hwm AND bqk -> z03
                    tgd XOR rvg -> z12
                    tnw OR pbm -> gnj
                "},
            );
        }

        #[test]
        fn part1_final() {
            assert_eq!(51657025112326, run_on_day_input(day!(), part1).unwrap());
        }
    }

    #[cfg(test)]
    mod part2_tests {
        use core::result::Result::Ok;
        use std::collections::VecDeque;
        use advent_of_code2024_rust::day_input;
        use super::*;

        fn binary_to_map(number: u64, prefix: char) -> HashMap<String, bool> {
            let mut result = HashMap::new();

            for bit_position in 0..45 {
                let key = format!("{}{:02}", prefix, bit_position);
                let bit_value = (number & (1 << bit_position)) != 0;
                result.insert(key, bit_value);
            }

            result
        }

        fn run(x: u64, y: u64, connections: &Vec<Connection>) -> ((u64, u64), HashMap<String, bool>) {
            let x_map = binary_to_map(x, 'x');
            let y_map = binary_to_map(y, 'y');

            let mut initial_values = HashMap::new();
            initial_values.extend(x_map);
            initial_values.extend(y_map);

            let connections = connections.clone();

            let (result, wire_values) = simulate_circuit(&Circuit {
                initial_values,
                connections
            }).unwrap();

            ((x + y, result as u64), wire_values)
        }

        fn test_run(x: u64, y: u64, connections: &Vec<Connection>) -> Result<u64> {
            let x_map = binary_to_map(x, 'x');
            let y_map = binary_to_map(y, 'y');

            let mut initial_values = HashMap::new();
            initial_values.extend(x_map);
            initial_values.extend(y_map);

            let connections = connections.clone();

            let (result, _) = simulate_circuit(&Circuit {
                initial_values,
                connections
            })?;

            Ok(result as u64)
        }

        fn test_run_full(x: u64, y: u64, connections: &Vec<Connection>) -> bool {
            match test_run(x, y, connections) {
                Ok(result) => result == x + y,
                Err(_) => {
                    false
                }
            }
        }

        fn test_bit_part(expect: u64, i: usize, x: u64, y: u64, connections: &Vec<Connection>) -> bool {
            match test_run(x, y, connections) {
                Ok(result) => {
                    if result >> i & 1 != expect {
                        return false;
                    }
                }
                Err(_) => {
                    return false;
                }
            }

            true
        }

        fn test_bit_heuristic(i: usize, connections: &Vec<Connection>) -> bool {
            test_bit_part(1, i, 1 << i, 0, connections) &&
            test_bit_part(1, i, 0, 1 << i, connections) &&
            test_bit_part(0, i, 1 << i, 1 << i, connections) &&
            (i == 0 ||
                (test_bit_part(1, i, 11 << (i - 1), 11 << (i - 1), connections)) &&
                (test_bit_part(0, i, (1 << i) - 1, (1 << (i + 1)) - 1, connections)) &&
                (test_bit_part(0, i, (1 << (i + 1)) - 1, (1 << i) - 1, connections))
            )
        }

        fn trace(i: usize, output_to_connections: &HashMap<String, &Connection>, wire_values: &HashMap<String, bool>) {
            let key = format!("z{:02}", i);
            println!("Trace {key}");
            let mut queue = VecDeque::new();
            queue.push_back((key, 0));
            while let Some((key, level)) = queue.pop_front() {
                if let Some(gate) = output_to_connections.get(&key) {
                    let indent = "  ".repeat(level);
                    let value = wire_values.get(&key).map(|v| if *v { 1 } else { 0 }).unwrap_or(-1);
                    println!("{indent}{gate}({})", value);
                    queue.push_front((gate.input2.clone(), level + 1));
                    queue.push_front((gate.input1.clone(), level + 1));
                }
            }
        }

        fn experiment(x: u64, y: u64) {
            let result = run(x, y, &parse_input(day_input(day!())).unwrap().connections).0;
            assert_eq!(
                format!("{:045b}", result.0),
                format!("{:045b}", result.1),
            );
        }

        #[test]
        fn test_bit_0() {
            let connections = parse_input(day_input(day!())).unwrap().connections;
            assert!(test_bit_heuristic(0, &connections));
        }

        #[test]
        fn test_bit_1() {
            let connections = parse_input(day_input(day!())).unwrap().connections;
            assert!(test_bit_heuristic(1, &connections));
        }

        #[test]
        fn test_bit_2() {
            let connections = parse_input(day_input(day!())).unwrap().connections;
            assert!(test_bit_heuristic(2, &connections));
        }

        #[test]
        fn test_bit_3() {
            let connections = parse_input(day_input(day!())).unwrap().connections;
            assert!(test_bit_heuristic(3, &connections));
        }

        #[test]
        fn test_bit_4() {
            let connections = parse_input(day_input(day!())).unwrap().connections;
            assert!(test_bit_heuristic(4, &connections));
        }

        #[test]
        fn test_bit_5() {
            let connections = parse_input(day_input(day!())).unwrap().connections;
            assert!(!test_bit_heuristic(5, &connections));
        }

        #[test]
        fn test1() {
            experiment(0, 0);
        }

        #[test]
        fn test2() {
            experiment(1 << 45 - 1, 0);
        }

        #[test]
        fn test3() {
            experiment(0, 1 << 45 - 1);
        }

        #[test]
        fn test4() {
            experiment(1 << 45 - 1, 1 << 45 - 1);
        }

        fn trace_different_bits(x: u64, y: u64) -> Vec<i32> {
            let xor = x ^ y;
            let mut positions = Vec::new();

            for i in 0..64 { // Loop over 64 bits (assuming u64)
                if (xor & (1 << i)) != 0 { // Check if the i-th bit is different
                    positions.push(i);
                }
            }

            positions
        }

        fn test_with_trace(
            i: i32, x: u64, y: u64,
            connections: &Vec<Connection>,
            output_to_connections: &HashMap<String, &Connection>
        ) {
            let ((expect, actual), wires_values) = run(x, y, connections);
            if expect != actual {
                println!("====================================");
                println!("Test: {i}");
                println!("x:{x:046b}\ny:{y:046b}\ne:{:046b}\na:{:046b}", expect, actual);
                trace_different_bits(expect, actual).iter().for_each(|w|
                    trace(*w as usize, output_to_connections, &wires_values)
                );
            }
        }

        fn swap_outputs(connection: &Vec<Connection>, outputs: (String, String)) -> Vec<Connection> {
            connection.iter().map(|conn| {
                if conn.output == outputs.0 {
                    Connection {
                        output: outputs.1.clone(),
                        ..conn.clone()
                    }
                } else if conn.output == outputs.1 {
                    Connection {
                        output: outputs.0.clone(),
                        ..conn.clone()
                    }
                } else {
                    conn.clone()
                }
            }).collect()
        }

        fn test_connections(i: usize, connection: &Vec<Connection>) -> bool {
            test_run_full(0, 0, connection) && test_bit_heuristic(i, connection)
        }

        fn auto_fix(i: u32, connection: &Vec<Connection>) -> (String, String) {
            let outputs = connection.iter().map(|conn| conn.output.clone()).collect::<HashSet<String>>();
            let dependency_map = generate_dependency_map(connection).unwrap();

            let previous_dependencies: HashSet<&String> =
                (0..i)
                    .flat_map(|z| dependency_map.get(&format!("z{:02}", z)).unwrap())
                    .collect();

            let possible_replacements = dependency_map.iter()
                .filter(|(output, _)| !previous_dependencies.contains(output))
                .filter(|(_, dependencies)| {
                    dependencies.iter()
                        .filter(|d| d.starts_with("x") || d.starts_with("y"))
                        .all(|d| d[1..].parse::<u32>().unwrap() <= i)
                })
                .map(|(output, _)| output.clone())
                .collect::<Vec<String>>();

            let allowed_affected_outputs = dependency_map.get(&format!("z{:02}", i)).unwrap()
                .iter()
                .filter(|&z_i_dependencies| outputs.contains(z_i_dependencies))
                .filter(|&z_i_dependencies| !previous_dependencies.contains(z_i_dependencies))
                .collect::<Vec<&String>>();

            println!("Auto fix {} bit, need to check Allowed = {}, Possible = {}, Total = {}",
                     i,
                     allowed_affected_outputs.len(),
                     possible_replacements.len(),
                     allowed_affected_outputs.len() * possible_replacements.len());

            let mut fixes = Vec::new();
            for z_i_dependencies in allowed_affected_outputs {
                for other in possible_replacements.iter() {
                    if other == z_i_dependencies {
                        continue;
                    }

                    let updated = swap_outputs(connection, (z_i_dependencies.clone(), other.clone()));

                    if test_connections(i as usize, &updated) {
                        fixes.push((z_i_dependencies.clone(), other.clone()));
                        println!("FOUND SWAP: {} -> {}", z_i_dependencies, other);
                    }
                }
            }

            if fixes.len() != 1 {
                panic!("Found {} fixes", fixes.len());
            }

            fixes[0].clone()
        }

        #[test]
        fn test_auto_fix_5() {
            auto_fix(5, &parse_input(day_input(day!())).unwrap().connections);
        }

        #[test]
        fn test_auto_fix_9() {
            auto_fix(9, &read_connection_with_fixes(1));
        }

        #[test]
        fn test_auto_fix_15() {
            auto_fix(15, &read_connection_with_fixes(2));
        }

        #[test]
        fn test_auto_fix_30() {
            auto_fix(30, &read_connection_with_fixes(3));
        }

        fn fixes() -> [(&'static str, &'static str); 4] {
            [("z05", "hdt"), ("z09", "gbf"), ("mht", "jgt"), ("z30", "nbf")]
        }

        #[test]
        fn sorted_fixes() {
            println!("{}", fixes().iter()
                .flat_map(|(a, b)| [*a, *b]).sorted().collect::<Vec<&str>>().join(","));
        }

        fn read_connection_with_fixes(i: usize) -> Vec<Connection> {
            fixes()[..i].iter().fold(parse_input(day_input(day!())).unwrap().connections, |acc, fix| {
                swap_outputs(&acc, (fix.0.to_string(), fix.1.to_string()))
            })
        }

        #[test]
        fn test_bits() {
            let connections = read_connection_with_fixes(4);

            let output_to_connections: HashMap<String, &Connection> =
                connections.iter()
                    .map(|conn| (conn.output.clone(), conn))
                    .collect();

            for i in 0..=44 {
                test_with_trace(i, 1 << i, 0, &connections, &output_to_connections);
                test_with_trace(i, 0, 1 << i, &connections, &output_to_connections);
                test_with_trace(i, 1 << i, 1 << i, &connections, &output_to_connections);
            }
        }
    }
}