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
    
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    
    // First check if current position is 'A'
    if grid[row as usize][col as usize] != 'A' {
        return false;
    }
    
    // Check for 'M' in one direction
    let m_row = row - dy;
    let m_col = col - dx;
    if m_row < 0 || m_row >= rows || m_col < 0 || m_col >= cols {
        return false;
    }
    if grid[m_row as usize][m_col as usize] != 'M' {
        return false;
    }
    
    // Check for 'S' in the opposite direction
    let s_row = row + dy;
    let s_col = col + dx;
    if s_row < 0 || s_row >= rows || s_col < 0 || s_col >= cols {
        return false;
    }
    if grid[s_row as usize][s_col as usize] != 'S' {
        return false;
    }
    
    true
}

fn find_xmas(grid: &[Vec<char>]) -> Vec<(usize, usize, &'static str)> {
    let mut findings = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();
    
    // Only diagonal directions
    let directions = [
        
        (1, 1, "down-right"),
        (-1, 1, "down-left"),
        (1, -1, "up-right"),
        (-1, -1, "up-left"),
    ];

    for row in 0..rows {
        for col in 0..cols {
            
            let mut found = false;
            for &(dx, dy, direction) in &directions {
                if check_direction(grid, row as i32, col as i32, dx, dy) {
                    if !found {
                        found = true;
                        println!("++ x: {} y: {}, dir: {}", row, col, direction);
                    } else { 
                        println!("!! x: {} y: {}, dir: {}", row, col, direction);
                        findings.push((row, col, direction));
                        break;
                    }
                } else {
                        println!("-- x: {} y: {}, dir: {}", row, col, direction);
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
