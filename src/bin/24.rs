use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};
use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Gate {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Connection {
    gate: Gate,
    input1: String,
    input2: String,
    output: String,
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

fn simulate_circuit(circuit: &Circuit) -> Result<i64> {
    let mut wire_values = circuit.initial_values.clone();

    // Keep simulating until no new values are computed
    loop {
        let mut changes = false;

        for conn in &circuit.connections {
            if wire_values.contains_key(&conn.output) {
                continue;
            }

            if let (Some(&input1), Some(&input2)) = (
                wire_values.get(&conn.input1),
                wire_values.get(&conn.input2),
            ) {
                let result = match conn.gate {
                    Gate::And => input1 && input2,
                    Gate::Or => input1 || input2,
                    Gate::Xor => input1 != input2,
                };
                wire_values.insert(conn.output.clone(), result);
                changes = true;
            }
        }

        if !changes {
            break;
        }
    }

    // Collect all z-wires and convert to decimal
    let binary_string: String = wire_values
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .collect::<Vec<_>>()  // collect to sort
        .iter()
        .sorted_by(|(k1, _), (k2, _)| k1.cmp(k2))
        .map(|(_, &value)| if value { '1' } else { '0' })
        .collect();

    let result = i64::from_str_radix(&binary_string, 2)?;

    Ok(result)
}

fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let circuit = parse_input(reader)?;
    simulate_circuit(&circuit)
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
            part1_result().unwrap();
        }
    }

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