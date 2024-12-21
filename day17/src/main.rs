use std::fs::read_to_string;

#[derive(Debug)]
enum Status {
    Running,
    Halt,
    BadHalt,
}

#[derive(Debug)]
struct MachineState {
    registers: Vec<i64>,
    program: Vec<i32>,
    status: Status,
    output_buffer: Vec<i32>,
}

fn parse_data_file(filename: &str) -> Result<MachineState, Box<dyn std::error::Error>> {
    let contents = read_to_string(filename)?;
    let mut registers = Vec::with_capacity(3);
    let mut program = Vec::new();

    for line in contents.lines() {
        if line.starts_with("Register") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let value = parts[1].trim().parse::<i64>()?;
                registers.push(value);
            }
        } else if line.starts_with("Program:") {
            let program_part = line.split(':').nth(1).unwrap().trim();
            program = program_part
                .split(',')
                .map(|s| s.trim().parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()?;
        }
    }

    Ok(MachineState {
        registers,
        program,
        status: Status::Running,
        output_buffer: Vec::new(),
    })
}

impl MachineState {
    fn get_operand_value(&mut self, operand: i32) -> Option<i64> {
        match operand {
            0..=3 => Some(operand.into()), // Convert i32 to i64
            4 => self.registers.get(0).copied(),
            5 => self.registers.get(1).copied(),
            6 => self.registers.get(2).copied(),
            7 => {
                self.status = Status::BadHalt;
                None
            }
            _ => {
                self.status = Status::BadHalt;
                None
            }
        }
    }
}

fn run(prog: &[i32], rega: i64, regb: i64, regc: i64) -> Vec<i32> {
    let mut state = MachineState {
        registers: vec![rega, regb, regc],
        program: prog.to_vec(),
        status: Status::Running,
        output_buffer: Vec::new(),
    };

    let mut ip: usize = 0;

    while ip < state.program.len() - 1 {
        let opcode = state.program[ip];
        let operand = state.program[ip + 1];

        println!("\nIP: {}", ip);
        println!("Before: Registers: {:?}", state.registers);
        println!("Executing opcode: {}, operand: {}", opcode, operand);

        match opcode {
            0 => {
                if let Some(op_val) = state.get_operand_value(operand) {
                    state.registers[0] = state.registers[0] / (1 << op_val);
                }
            }
            1 => {
                // bxl - directly use operand value without get_operand_value
                state.registers[1] ^= operand as i64; // Convert operand to i64 for bitwise operation
            }
            2 => {
                if let Some(op_val) = state.get_operand_value(operand) {
                    state.registers[1] = op_val % 8;
                }
            }
            3 => {
                if let Some(op_val) = state.get_operand_value(operand) {
                    if state.registers[0] != 0 {
                        ip = op_val as usize;
                        continue;
                    }
                }
            }
            4 => {
                state.registers[1] = state.registers[1] ^ state.registers[2];
            }
            5 => {
                if let Some(op_val) = state.get_operand_value(operand) {
                    state.output_buffer.push((op_val % 8) as i32);
                }
            }
            6 => {
                if let Some(op_val) = state.get_operand_value(operand) {
                    state.registers[1] = state.registers[0] / (1 << op_val);
                }
            }
            7 => {
                if let Some(op_val) = state.get_operand_value(operand) {
                    state.registers[2] = state.registers[0] / (1 << op_val);
                }
            }
            _ => break,
        }
        ip += 2;
        println!("After:  Registers: {:?}", state.registers);
    }

    state.output_buffer
}

fn rev_eng(prog: &[i32], _reg_b: i64, _reg_c: i64) -> i64 {
    let mut reg_a = 0;
    let mut to_match: usize = 1;
    let mut istart = 0;

    while to_match <= prog.len() && to_match >= 1 {
        println!("\n--- Loop Start ---");
        println!(
            "to_match: {}, reg_a: {}, istart: {}",
            to_match, reg_a, istart
        );
        reg_a <<= 3;
        println!("After shift left: reg_a = {}", reg_a);

        let mut found_match = false;
        for i in istart..8 {
            println!("\nTrying i={}", i);
            let output = run(prog, reg_a + i, 0, 0); // Always start with clean registers
            let prog_slice = &prog[prog.len() - to_match..];
            println!(
                "Comparing prog[-{}:] = {:?} with output = {:?}",
                to_match, prog_slice, output
            );

            // Direct comparison with the output, just like Python
            if prog_slice == output {
                println!("Match found with i={}", i);
                reg_a += i;
                to_match += 1;
                istart = 0;
                found_match = true;
                break;
            }
        }

        if !found_match {
            println!("No match found - backtracking");
            to_match -= 1;
            reg_a >>= 3;
            istart = (reg_a % 8) + 1;
            reg_a >>= 3;
            println!(
                "Backtrack: to_match={}, new reg_a={}, new istart={}",
                to_match, reg_a, istart
            );
            continue;
        }

        println!("Success: adding i={} to reg_a", reg_a % 8);
    }

    println!("\nFinal reg_a: {}", reg_a);
    reg_a
}

fn main() {
    let state = match parse_data_file("data2") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let output = run(
        &state.program,
        state.registers[0],
        state.registers[1],
        state.registers[2],
    );

    print!("\nProgram output: ");
    for (i, val) in output.iter().enumerate() {
        if i < output.len() - 1 {
            print!("{},", val);
        } else {
            println!("{}", val);
        }
    }

    // Try to find the input that makes the program output itself
    let result = rev_eng(&state.program, state.registers[1], state.registers[2]);
    println!("\nFound input value: {}", result);
    let output = run(
        &state.program,
        result,
        state.registers[1],
        state.registers[2],
    );
    print!("Verification output: ");
    for (i, val) in output.iter().enumerate() {
        if i < output.len() - 1 {
            print!("{},", val);
        } else {
            println!("{}", val);
        }
    }
    println!("\nReverse engineered register A value: {}", result);
}
