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

    fn execute(&mut self, program: &Vec<u8>) -> Vec<u8> {
        while self.instruction_pointer < program.len() {
            let opcode = program[self.instruction_pointer];
            let operand = if self.instruction_pointer + 1 < program.len() {
                program[self.instruction_pointer + 1]
            } else {
                // If the computer tries to read an opcode past the end of the program, it instead halts.
                break
            };

            // println!("{:03} {} {}", self.instruction_pointer, opcode, operand);


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

            // println!("State:\na={:b}\nb={:b}\nc={:b}", self.register_a, self.register_b, self.register_c);
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

fn execute_program(computer: &mut ThreeBitComputer, program: &Vec<u8>) -> String {
    let output = computer.execute(program);

    output.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
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
    Ok(execute_program(&mut computer, &program))
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
            computer.execute(&program);
            assert_eq!(computer.register_b, 1);
        }

        #[test]
        fn test_example_two() {
            // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
            let mut computer = ThreeBitComputer::new(10, 0, 0);
            let program = vec![5, 0, 5, 1, 5, 4];
            let output = computer.execute(&program);
            assert_eq!(output, vec![0, 1, 2]);
        }

        #[test]
        fn test_example_three() {
            // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0
            // and leave 0 in register A.
            let mut computer = ThreeBitComputer::new(2024, 0, 0);
            let program = vec![0, 1, 5, 4, 3, 0];
            let output = computer.execute(&program);
            assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
            assert_eq!(computer.register_a, 0);
        }

        #[test]
        fn test_example_four() {
            // If register B contains 29, the program 1,7 would set register B to 26.
            let mut computer = ThreeBitComputer::new(0, 29, 0);
            let program = vec![1, 7];
            computer.execute(&program);
            assert_eq!(computer.register_b, 26);
        }

        #[test]
        fn test_example_five() {
            // If register B contains 2024 and register C contains 43690,
            // the program 4,0 would set register B to 44354.
            let mut computer = ThreeBitComputer::new(0, 2024, 43690);
            let program = vec![4, 0];
            computer.execute(&program);
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
            let output = computer.execute(&program);
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
        fn test2() {
            test_part1(
                "0,3,5,4,3,0",
                indoc! {"
                    Register A: 117440
                    Register B: 0
                    Register C: 0

                    Program: 0,3,5,4,3,0
                "},
            );
        }

        #[test]
        fn part1_final() {
            assert_eq!("6,1,6,4,2,4,7,3,5", run_on_day_input(day!(), part1).unwrap());
        }
    }

    //noinspection SpellCheckingInspection
    #[cfg(test)]
    mod part2_tests {
        use super::*;

        fn execute_experimental_program(a: i64) -> String {
            let mut computer = ThreeBitComputer::new(a, 0, 0);
            let program = vec![2,4,1,1,7,5,0,3,1,4,4,4,5,5,3,0];
            execute_program(&mut computer, &program)
        }

        fn experiment(expect: &str, a: i64) {
            assert_eq!(expect, execute_experimental_program(a));
        }

        #[test] fn test_bits() {
            let program = vec![2, 4, 1, 1, 7, 5, 0, 3, 1, 4, 4, 4, 5, 5, 3, 0];
            let mut result: Vec<i64> = Default::default();
            result.push(0);

            for i in (0..program.len()).rev() {
                let mut next: Vec<i64> = Default::default();
                let target = program[i];

                for var in result.iter() {
                    println!("i={}, var={}, target={}", i, var, target);

                    for k in 0..=0b111 {
                        let a = (var << 3) | k as i64;
                        let mut computer = ThreeBitComputer::new(a, 0, 0);
                        let output = computer.execute(&program);
                        if output.first().unwrap() == &target {
                            next.push(a);
                        }
                    }
                }

                result = next;
            }

            let a = *(result.first().unwrap());
            experiment("2,4,1,1,7,5,0,3,1,4,4,4,5,5,3,0", a);
            println!("{}", a);
        }

        #[test] fn test0() { experiment("5", 0); }
        #[test] fn test1() { experiment("5", 1); }
        #[test] fn test2() { experiment("7", 2); }
        #[test] fn test3() { experiment("6", 3); }
        #[test] fn test4() { experiment("1", 4); }
        #[test] fn test5() { experiment("0", 5); }
        #[test] fn test6() { experiment("3", 6); }
        #[test] fn test7() { experiment("2", 7); }
        #[test] fn test8() { experiment("1,5", 8); }
        #[test] fn test9() { experiment("5,5", 9); }
        #[test] fn test10() { experiment("6,5", 10); }
        #[test] fn test11() { experiment("4,5", 11); }
        #[test] fn test12() { experiment("1,5", 12); }
        #[test] fn test13() { experiment("0,5", 13); }
        #[test] fn test14() { experiment("3,5", 14); }
        #[test] fn test15() { experiment("2,5", 15); }
        #[test] fn test16() { experiment("5,7", 16); }
        #[test] fn test281474976710655() {
            experiment("5,5,5,5,5,5,5,5,5,5,5,5,5,5,2,2", 281474976710655); // 8^16 - 1
        }

        #[test] fn test35184372088832() {
            experiment("5,5,5,5,5,5,5,5,5,5,5,5,5,5,1,5", 35184372088832); // 8^15
        }

        #[test] fn test35184372088831() {
            experiment("5,5,5,5,5,5,5,5,5,5,5,5,5,2,2", 35184372088831); // 8^15 - 1
        }

        #[test] fn test4398046511103() {
            experiment("5,5,5,5,5,5,5,5,5,5,5,5,2,2", 4398046511103); // 8^14 - 1
        }
    }
}
