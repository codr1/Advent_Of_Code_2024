use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashSet, HashMap};

#[derive(Debug)]
enum Direction {
    North, // -Y
    East,  // +X
    South, // +Y
    West,  // -X
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

fn read_grid_from_file(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    
    let mut grid = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        if !chars.is_empty() {
            grid.push(chars);
        }
    }
    
    Ok(grid)
}

fn find_cursor(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch == '^' {
                return Some((row_idx, col_idx));
            }
        }
    }
    None
}

fn is_valid_move(grid: &[Vec<char>], row: i32, col: i32) -> bool {
    row >= 0 && 
    col >= 0 && 
    row < grid.len() as i32 && 
    col < grid[0].len() as i32
}

fn get_next_position(current: (i32, i32), direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::North => (current.0 - 1, current.1),
        Direction::East => (current.0, current.1 + 1),
        Direction::South => (current.0 + 1, current.1),
        Direction::West => (current.0, current.1 - 1),
    }
}

#[derive(Debug)]
enum WalkResult {
    ExitGrid,
    Loop,
}

fn walk_grid_detect_loop(grid: &[Vec<char>], start: (usize, usize)) -> WalkResult {
    let mut visit_count: HashMap<(i32, i32), usize> = HashMap::new();
    let mut current = (start.0 as i32, start.1 as i32);
    let mut direction = Direction::North;
    
    visit_count.insert(current, 1);
    
    loop {
        let next = get_next_position(current, &direction);
        
        if !is_valid_move(grid, next.0, next.1) {
            return WalkResult::ExitGrid;
        }
        
        if grid[next.0 as usize][next.1 as usize] == '#' {
            direction = direction.next();
            continue;
        }
        
        current = next;
        
        let count = visit_count.entry(current).or_insert(0);
        *count += 1;
        
        if *count > 3 {
            return WalkResult::Loop;
        }
    }
}

fn find_loops(grid: &mut Vec<Vec<char>>, start: (usize, usize)) -> usize {
    let mut loop_count = 0;
    let height = grid.len();
    let width = grid[0].len();
    
    for row in 0..height {
        for col in 0..width {
            // Skip if not a dot or if it's the starting position
            if grid[row][col] != '.' || (row == start.0 && col == start.1) {
                continue;
            }
            
            // Try placing a wall here
            grid[row][col] = '#';
            
            // Check if this creates a loop
            match walk_grid_detect_loop(grid, start) {
                WalkResult::Loop => {
                    loop_count += 1;
                    println!("Found loop with wall at ({}, {})", row, col);
                }
                WalkResult::ExitGrid => {}
            }
            
            // Restore the dot
            grid[row][col] = '.';
        }
    }
    
    loop_count
}

fn main() {
    match read_grid_from_file("data") {
        Ok(grid) => {
            println!("Successfully read the grid:");
            for row in &grid {
                for &ch in row {
                    print!("{}", ch);
                }
                println!();
            }
            
            
            if let Some((row, col)) = find_cursor(&grid) {
                println!("\nFound cursor (^) at position: row {}, column {}", row + 1, col + 1);
            let mut grid_copy = grid.clone();
            let loop_count = find_loops(&mut grid_copy, (row, col));
            println!("Number of possible loops found: {}", loop_count);
            } else {
                println!("\nNo cursor (^) found in the grid!");
            }
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}
