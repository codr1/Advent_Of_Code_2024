use std::fs::File;
use std::io::Read;


fn parse_line(line_start: usize, chars: &[u8]) -> (Vec<i32>, usize) {
    let mut numbers = Vec::new();
    let mut current_start = line_start;
    let mut index = line_start;
    
    while index < chars.len() && chars[index] != b'\n' {
        if chars[index] == b' ' || chars[index] == b'\n' {
            let number = chars[current_start..index]
                .iter()
                .fold(0, |acc, &c| acc * 10 + (c as u8 - b'0') as i32);
            numbers.push(number);
            current_start = index + 1;
        }
        index += 1;
    }
    
    // Don't forget the last number in the line
    if current_start < index {
        let number = chars[current_start..index]
            .iter()
            .fold(0, |acc, &c| acc * 10 + (c as u8 - b'0') as i32);
        numbers.push(number);
    }
    
    (numbers, index)
}

    fn check_sequence(numbers: &[i32]) -> bool {
        let mut prev = numbers[0];
        let mut ascending: Option<bool> = None;
        
        
        for &number in numbers.iter().skip(1) {
            let delta = prev - number;
            
            // check if we are way off...
            if delta.abs() > 3 || delta == 0 {
                return false;
            }
            
            if ascending.is_none() {
                ascending = Some(delta < 0);
            } else if (ascending == Some(true) && delta > 0) || (ascending == Some(false) && delta < 0) {
                return false;
            }
            
            prev = number;
        }
        
        true
    }
    
        fn analyze_line(numbers: &[i32]) -> bool {
            // First check if sequence is valid without removing anything
            if check_sequence(numbers) {
                return true;
            }
            
            // Try removing each number in turn
            for skip_index in 0..numbers.len() {
                let mut test_sequence: Vec<i32> = Vec::new();
                for (i, &num) in numbers.iter().enumerate() {
                    if i != skip_index {
                        test_sequence.push(num);
                    }
                }
                
                if check_sequence(&test_sequence) {
                    println!("  Found valid sequence by removing {}", numbers[skip_index]);
                    return true;
                }
            }
            
            false
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut file = File::open("data")?;
    let mut characters = Vec::new();
    file.read_to_end(&mut characters)?;

    let mut start_index = 0;
    let mut danger_reps = 0;
    let mut total_reps = 0;
    
    while start_index < characters.len() {
        let (numbers, new_index) = parse_line(start_index, &characters);
        if !numbers.is_empty() {
            total_reps += 1;
            println!("Line {}: {:?}", total_reps, numbers);
            
            if !analyze_line(&numbers) {
                danger_reps += 1;
                println!("Rep {} is dangerous", total_reps);
            }
        }
        start_index = new_index + 1;
    }

    println!("Total {} Dangerous {}", total_reps, danger_reps);
    Ok(())
}
