use std::fs::read_to_string;
use regex::Regex;

#[derive(Debug)]
enum Command {
    Multiply(u32, u32, u32), // (n, m, result)
    Do,
    Dont,
}

fn main() {
    // Read the file
    let contents = match read_to_string("data") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    // Create regex patterns
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don['']t\(\)").unwrap();  // handles both types of apostrophes

    // Find all commands in order
    let mut commands = Vec::new();
    let mut last_end = 0;
    
    // Scan through the string looking for any of our patterns
    while last_end < contents.len() {
        let remainder = &contents[last_end..];
        
        // Find all possible matches at current position
        let mul_match = mul_re.find_at(remainder, 0);
        let do_match = do_re.find_at(remainder, 0);
        let dont_match = dont_re.find_at(remainder, 0);

        // Find the earliest match (if any)
        let next_match = [
            mul_match.map(|m| (0_u8, m)),  // Using u8 instead of implicit i32
            do_match.map(|m| (1_u8, m)),
            dont_match.map(|m| (2_u8, m))
        ].into_iter()
        .flatten()
        .min_by_key(|(_, m)| m.start());

        match next_match {
            Some((0, cap)) => {
                let caps = mul_re.captures(&remainder[cap.start()..cap.end()]).unwrap();
                let n: u32 = caps[1].parse().unwrap();
                let m: u32 = caps[2].parse().unwrap();
                commands.push(Command::Multiply(n, m, n * m));
                last_end += cap.end();
            },
            Some((1, cap)) => {
                commands.push(Command::Do);
                last_end += cap.end();
            },
            Some((2, cap)) => {
                commands.push(Command::Dont);
                last_end += cap.end();
            },
            None => {
                last_end += 1;
            }
            Some(_) => {
                // This shouldn't happen since we only create matches with 0, 1, or 2
                last_end += 1;
            }
        }
    }
    
        // Print all commands in order with command number
        let mut sum = 0;
        let mut ignore = false;
        for (i, cmd) in commands.iter().enumerate() {
            match cmd {
                Command::Multiply(n, m, result) => {
                    println!("Command {}: MUL    {} * {} = {}", i + 1, n, m, result);
                    if !ignore {
                        sum += result;
                    }
                }
                Command::Do => {
                    println!("Command {}: DO     do()", i + 1);
                    ignore = false;
                }
                Command::Dont => {
                    println!("Command {}: DON'T  don't()", i + 1);
                    ignore = true;
                }
            }
        }
        println!("\nTotal commands found: {}", commands.len());
        println!("Sum of multiplications: {}", sum);
}
