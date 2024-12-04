use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_to_array(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    let mut array_2d: Vec<Vec<char>> = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        // Convert each line into a vector of characters
        let row: Vec<char> = line.chars().collect();
        array_2d.push(row);
    }
    
    Ok(array_2d)
}

fn check_direction(grid: &[Vec<char>], row: i32, col: i32, dx: i32, dy: i32) -> bool {
    let target = ['X', 'M', 'A', 'S'];
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    
    // Check if we can fit "XMAS" in this direction from this position
    for i in 0..4 {
        let new_row = row + dy * i;
        let new_col = col + dx * i;
        
        if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
            return false;
        }
        
        if grid[new_row as usize][new_col as usize] != target[i as usize] {
            return false;
        }
    }
    true
}

fn find_xmas(grid: &[Vec<char>]) -> Vec<(usize, usize, &'static str)> {
    let mut findings = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();
    
    // All eight directions: right, left, up, down, and all diagonals
    let directions = [
        (1, 0, "right"),
        (-1, 0, "left"),
        (0, 1, "down"),
        (0, -1, "up"),
        (1, 1, "down-right"),
        (-1, 1, "down-left"),
        (1, -1, "up-right"),
        (-1, -1, "up-left"),
    ];

    for row in 0..rows {
        for col in 0..cols {
            for &(dx, dy, direction) in &directions {
                if check_direction(grid, row as i32, col as i32, dx, dy) {
                    findings.push((row, col, direction));
                }
            }
        }
    }
    
    findings
}

fn main() -> io::Result<()> {
    let filename = "data";
    match read_to_array(filename) {
        Ok(grid) => {
            // Find all occurrences of "XMAS"
            let findings = find_xmas(&grid);
            
            if findings.is_empty() {
                println!("No 'XMAS' patterns found!");
            } else {
                println!("Found 'XMAS' at the following positions:");
                for (row, col, direction) in &findings {
                    println!("Position ({}, {}) going {}", row, col, direction);
                }
            }
            println!("Number found: {}", findings.len());
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
    
    Ok(())
}
