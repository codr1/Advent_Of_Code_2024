use std::fs::read_to_string;

const GRID_X = 11;
const GRID_Y = 7;
// const GRID_X = 101;
// const GRID_Y = 103;

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    x_v: i32,
    y_v: i32
}


fn parse_line(line: &str) -> Option<(i128, i128)> {
    let parts: Vec<&str> = line.split(":").collect();
    if parts.len() != 2 {
        return None;
    }

    let coords = parts[1].trim();
    let x_y: Vec<&str> = coords.split(",").collect();
    if x_y.len() != 2 {
        return None;
    }

    let x = x_y[0]
        .trim()
        .trim_start_matches("X")
        .trim_start_matches("=")
        .trim_start_matches("+");
    let y = x_y[1]
        .trim()
        .trim_start_matches("Y")
        .trim_start_matches("=")
        .trim_start_matches("+");

    Some((x.parse().unwrap_or(0), y.parse().unwrap_or(0)))
}

fn parse_machines(content: &str) -> Vec<Machine> {
    let mut robots = Vec::new();
    let mut lines = content.lines();

    while let Some(robot) = lines.next()
    {
    }

    machines
}

fn solve_machine(machine: &Machine) -> Option<(i128, i128)> {
    // Create the coefficient matrix A and vector b for Ax = b
    let a = Matrix2::new(
        machine.button_a.x as f64,
        machine.button_b.x as f64,
        machine.button_a.y as f64,
        machine.button_b.y as f64,
    );

    let b = Vector2::new(machine.prize.x as f64, machine.prize.y as f64);

    // Solve the system of equations
    match a.try_inverse() {
        Some(a_inv) => {
            let solution = a_inv * b;
            let n2 = solution[0].round();
            let n1 = solution[1].round();

            if n1 >= 0.0 && n2 >= 0.0 {
                let n1_i = n1 as i128;
                let n2_i = n2 as i128;

                // Verify solution with exact arithmetic
                let x_valid =
                    n2_i * machine.button_a.x + n1_i * machine.button_b.x == machine.prize.x;
                let y_valid =
                    n2_i * machine.button_a.y + n1_i * machine.button_b.y == machine.prize.y;

                if x_valid && y_valid {
                    return Some((n2_i, n1_i));
                }
            }
        }
        None => return None,
    }
    None
}

fn main() {
    let content = read_to_string("data").expect("Could not read file");
    let robots = parse_robots(&content);

    let mut total_tokens: i128 = 0;

    println!("Found {} machines:", machines.len());
    for (i, machine) in machines.iter().enumerate() {
        println!("\nMachine {}:", i + 1);
        println!(
            "  Button A: ({}, {})",
            machine.button_a.x, machine.button_a.y
        );
        println!(
            "  Button B: ({}, {})",
            machine.button_b.x, machine.button_b.y
        );
        println!("  Prize: ({}, {})", machine.prize.x, machine.prize.y);

        // Try to solve the machine
        match solve_machine(machine) {
            Some((a, b)) => {
                println!("  Solution found: Press A {} times and B {} times", a, b);
                total_tokens += a * 3 + b;
            }
            None => println!("  No solution found"),
        }
    }

    println!("Total Tokens: {}", total_tokens);
}
