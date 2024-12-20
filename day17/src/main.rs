use std::fs::read_to_string;

#[derive(Debug)]
enum Status {
    Running,
    Halt,
    BadHalt,
}

#[derive(Debug)]
struct MachineState {
    registers: Vec<i32>,
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
                let value = parts[1].trim().parse::<i32>()?;
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
    fn get_operand_value(&mut self, operand: i32) -> Option<i32> {
        match operand {
            0..=3 => Some(operand),
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

fn main() {
    let mut state = match parse_data_file("data2") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut ip: usize = 0;

    loop {
        if ip >= state.program.len() - 1 {
            // Need space for opcode + operand
            state.status = Status::Halt;
            break;
        }

        let opcode = state.program[ip];
        let operand = state.program[ip + 1];

        println!("\nIP: {}", ip);
        println!("Before: Registers: {:?}", state.registers);
        println!("Executing opcode: {}, operand: {}", opcode, operand);

        match opcode {
            0 => {
                // adv - division - reg A / 2^operand
                if let Some(op_val) = state.get_operand_value(operand) {
                    state.registers[0] = state.registers[0] / (1 << op_val);
                }
            }
            1 => {
                // bxl - bitwise xor of reg B and the literal operand
                println!(
                    "Reg {:?} Open {}  Xor {}",
                    state.registers[1],
                    operand,
                    state.registers[1] ^ operand,
                );
                state.registers[1] ^= operand;
            }
            2 => {
                // bst - modulo 8 then writes to reg B
                if let Some(op_val) = state.get_operand_value(operand) {
                    state.registers[1] = op_val % 8;
                }
            }
            3 => {
                // jnz - if a is 0, noop, else ip = literal operand and continue
                if state.registers[0] != 0 {
                    ip = operand as usize;
                    continue; // Skip the ip += 2 at the end
                }
            }
            4 => {
                // bxc - bitwise xor of B and C then stores in B
                state.registers[1] = state.registers[1] ^ state.registers[2];
            }
            5 => {
                // out - operand mod 8 then store it in output buffer
                if let Some(op_val) = state.get_operand_value(operand) {
                    state.output_buffer.push(op_val % 8);
                }
            }
            6 => {
                // bdv - same as adv but store result in B
                if let Some(op_val) = state.get_operand_value(operand) {
                    state.registers[1] = state.registers[0] / (1 << op_val);
                }
            }
            7 => {
                // cdv - same as adv but store result in C
                if let Some(op_val) = state.get_operand_value(operand) {
                    state.registers[2] = state.registers[0] / (1 << op_val);
                }
            }
            _ => {
                state.status = Status::BadHalt;
                break;
            }
        }

        ip += 2; // Move to next instruction (each instruction is 2 words)
        println!("After:  Registers: {:?}", state.registers);
    }

    println!("\nFinal state: {:?}", state);
    print!("\nProgram output: ");
    for (i, val) in state.output_buffer.iter().enumerate() {
        if i < state.output_buffer.len() - 1 {
            print!("{},", val);
        } else {
            println!("{}", val);
        }
    }
}
