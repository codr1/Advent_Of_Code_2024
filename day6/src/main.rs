use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

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

fn walk_grid(grid: &[Vec<char>], start: (usize, usize)) -> usize {
    let mut visited = HashSet::new();
    let mut current = (start.0 as i32, start.1 as i32);
    let mut direction = Direction::North;
    
    // Add starting position to visited set
    visited.insert(current);
    
    loop {
        let next = get_next_position(current, &direction);
        
        // Check if next position is outside grid
        if !is_valid_move(grid, next.0, next.1) {
            break;
        }
        
        // Check if we hit a wall (#)
        if grid[next.0 as usize][next.1 as usize] == '#' {
            direction = direction.next();
            continue;
        }
        
        // Move to next position
        current = next;
        visited.insert(current);
    }
    
    visited.len()
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
            let visited_count = walk_grid(&grid, (row, col));
            println!("Number of coordinates visited: {}", visited_count);
            } else {
                println!("\nNo cursor (^) found in the grid!");
            }
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}
