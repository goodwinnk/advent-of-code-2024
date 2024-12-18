use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};

#[derive(Debug)]
struct ThreeBitComputer {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    instruction_pointer: usize,
    output: Vec<u8>,
}

impl ThreeBitComputer {
    fn new(a: i64, b: i64, c: i64) -> Self {
        Self {
            register_a: a,
            register_b: b,
            register_c: c,
            instruction_pointer: 0,
            output: Vec::new(),
        }
    }

    fn execute(&mut self, program: Vec<u8>) -> Vec<u8> {
        while self.instruction_pointer < program.len() {
            let opcode = program[self.instruction_pointer];
            let operand = if self.instruction_pointer + 1 < program.len() {
                program[self.instruction_pointer + 1]
            } else {
                // If the computer tries to read an opcode past the end of the program, it instead halts.
                break
            };

            match opcode {
                0 => self.adv(operand),  // Division to A register
                1 => self.bxl(operand),  // Bitwise XOR to B register with literal
                2 => self.bst(operand),  // Set B register
                3 => self.jnz(operand),  // Jump if not zero
                4 => self.bxc(operand),  // XOR B with C
                5 => self.out(operand),  // Output
                6 => self.bdv(operand),  // Division to B register
                7 => self.cdv(operand),  // Division to C register
                _ => panic!("Unknown opcode"),
            }
        }

        self.output.clone()
    }

    fn adv(&mut self, operand: u8) {
        let denominator = 2_i64.pow(self.get_combo_value(operand) as u32);
        self.register_a /= denominator;
        self.instruction_pointer += 2;
    }

    fn bxl(&mut self, operand: u8) {
        self.register_b ^= operand as i64;
        self.instruction_pointer += 2;
    }

    fn bst(&mut self, operand: u8) {
        self.register_b = self.get_combo_value(operand) % 8;
        self.instruction_pointer += 2;
    }

    fn jnz(&mut self, operand: u8) {
        if self.register_a != 0 {
            self.instruction_pointer = operand as usize * 2;
        } else {
            self.instruction_pointer += 2;
        }
    }

    fn bxc(&mut self, _operand: u8) {
        self.register_b ^= self.register_c;
        self.instruction_pointer += 2;
    }

    fn out(&mut self, operand: u8) {
        self.output.push((self.get_combo_value(operand) % 8) as u8);
        self.instruction_pointer += 2;
    }

    fn bdv(&mut self, operand: u8) {
        let denominator = 2_i64.pow(self.get_combo_value(operand) as u32);
        self.register_b = self.register_a / denominator;
        self.instruction_pointer += 2;
    }

    fn cdv(&mut self, operand: u8) {
        let denominator = 2_i64.pow(self.get_combo_value(operand) as u32);
        self.register_c = self.register_a / denominator;
        self.instruction_pointer += 2;
    }

    fn get_combo_value(&self, operand: u8) -> i64 {
        match operand {
            0..=3 => operand as i64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid program"), // Reserved or invalid
        }
    }
}

//noinspection DuplicatedCode
fn parse_register_value(line: &str) -> Result<i64> {
    line.split(": ")
        .nth(1)
        .ok_or_else(|| Error::msg("Invalid register line"))
        .and_then(|value|
            value.trim().parse().map_err(|_| Error::msg("Cannot parse register value"))
        )
}

fn parse_program(line: &str) -> Result<Vec<u8>> {
    line.split(": ")
        .nth(1)
        .ok_or_else(|| Error::msg("Invalid program line"))
        .and_then(|program_str| {
            program_str
                .split(',')
                .map(|s| s.trim().parse().map_err(|_| Error::msg("Cannot parse program value")))
                .collect()
        })
}

fn part1<R: BufRead>(mut reader: R) -> Result<String> {
    let mut register_a = 0;
    let mut register_b = 0;
    let mut register_c = 0;
    let mut program = Vec::new();

    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        if line.starts_with("Register A:") {
            register_a = parse_register_value(&line)?;
        } else if line.starts_with("Register B:") {
            register_b = parse_register_value(&line)?;
        } else if line.starts_with("Register C:") {
            register_c = parse_register_value(&line)?;
        } else if line.starts_with("Program:") {
            program = parse_program(&line)?;
        }
        line.clear();
    }

    let mut computer = ThreeBitComputer::new(register_a, register_b, register_c);
    let output = computer.execute(program);

    // Convert output to comma-separated string
    Ok(output.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(","))
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

        fn test_part1(expect: &str, input: &str) {
            assert_eq!(expect, part1(BufReader::new(input.as_bytes())).unwrap());
        }

        #[test]
        fn test_example_one() {
            // If register C contains 9, the program 2,6 would set register B to 1.
            let mut computer = ThreeBitComputer::new(0, 0, 9);
            let program = vec![2, 6];
            computer.execute(program);
            assert_eq!(computer.register_b, 1);
        }

        #[test]
        fn test_example_two() {
            // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
            let mut computer = ThreeBitComputer::new(10, 0, 0);
            let program = vec![5, 0, 5, 1, 5, 4];
            let output = computer.execute(program);
            assert_eq!(output, vec![0, 1, 2]);
        }

        #[test]
        fn test_example_three() {
            // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0
            // and leave 0 in register A.
            let mut computer = ThreeBitComputer::new(2024, 0, 0);
            let program = vec![0, 1, 5, 4, 3, 0];
            let output = computer.execute(program);
            assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
            assert_eq!(computer.register_a, 0);
        }

        #[test]
        fn test_example_four() {
            // If register B contains 29, the program 1,7 would set register B to 26.
            let mut computer = ThreeBitComputer::new(0, 29, 0);
            let program = vec![1, 7];
            computer.execute(program);
            assert_eq!(computer.register_b, 26);
        }

        #[test]
        fn test_example_five() {
            // If register B contains 2024 and register C contains 43690,
            // the program 4,0 would set register B to 44354.
            let mut computer = ThreeBitComputer::new(0, 2024, 43690);
            let program = vec![4, 0];
            computer.execute(program);
            assert_eq!(computer.register_b, 44354);
        }

        #[test]
        fn test_specific_problem_example() {
            // Example from the problem description:
            // Register A: 729
            // Register B: 0
            // Register C: 0
            // Program: 0,1,5,4,3,0
            // Expected output: 4,6,3,5,6,3,5,2,1,0
            let mut computer = ThreeBitComputer::new(729, 0, 0);
            let program = vec![0, 1, 5, 4, 3, 0];
            let output = computer.execute(program);
            assert_eq!(output, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
        }

        #[test]
        fn test1() {
            test_part1(
                "4,6,3,5,6,3,5,2,1,0",
                indoc! {"
                    Register A: 729
                    Register B: 0
                    Register C: 0

                    Program: 0,1,5,4,3,0
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
