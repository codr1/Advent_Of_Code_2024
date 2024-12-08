use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Entry {
    answer: i128,
    numbers: Vec<i128>,
}

fn parse_line(line: &str) -> Option<Entry> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    let answer = parts[0].trim().parse().ok()?;
    let numbers: Vec<i128> = parts[1]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    Some(Entry { answer, numbers })
}


fn try_combinations(numbers: &[i128], answer: i128) -> bool {
    let n = numbers.len() - 1; // number of spaces between numbers
    let max_combinations = 3_i128.pow(n as u32); // 3^n combinations for 3 operators

    // Try each possible combination of operators
    for i in 0..max_combinations {
        let mut result = numbers[0];
        
        // Use modulo to determine operators
        let mut combo = i;
        for j in 0..n {
            let next_num = numbers[j + 1];
            match combo % 3 {
                0 => result += next_num,      // Addition
                1 => result *= next_num,      // Multiplication
                2 => result = (result * 10_i128.pow(next_num.abs().to_string().len() as u32)) + next_num, // Concat
                _ => unreachable!()
            }
            combo /= 3;
        }

        if result == answer {
            return true;
        }
    }
    false
}

fn main() {
    let path = Path::new("data");
    if let Ok(file) = File::open(path) {
        let reader = io::BufReader::new(file);

        let mut sum = 0;
        for line in reader.lines() {
            if let Ok(line) = line {
                if let Some(entry) = parse_line(&line) {
                    print!("Testing {}: ", entry.answer);
                    if try_combinations(&entry.numbers, entry.answer) {
                        println!("Victory!");
                        sum += entry.answer
                    } else {
                        println!("Failure!");
                    }
                }
            }
        }
        println!("Done with {}", sum);
    } else {
        println!("Could not open file 'data'");
    }

}
