use std::i8;
use std::io;

#[derive(Debug, Clone)]
struct Cell {
    symbol: char,
    color: i8, // -1 for uncolored, 1-16 for colors
}

impl Cell {
    fn new(symbol: char) -> Self {
        Cell {
            symbol,
            color: -1, // Initially uncolored
        }
    }

    fn set_color(&mut self, color: i8) {
        assert!(color >= 1 && color <= 16, "Color must be between 1 and 16");
        self.color = color;
    }
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(cells: Vec<Vec<Cell>>) -> Self {
        let rows = cells.len();
        let cols = cells[0].len();
        Grid { cells, rows, cols }
    }

    fn count_sides(&self, start_row: usize, start_col: usize) -> i32 {
        println!(
            "\nProcessing region '{}' at ({}, {})",
            self.cells[start_row][start_col].symbol, start_row, start_col
        );
        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)]; // Up, Right, Down, Left
        let symbol = self.cells[start_row][start_col].symbol;
        let color = self.cells[start_row][start_col].color;

        // Check if this is a single cell region
        let mut has_neighbor = false;
        for (dx, dy) in &directions {
            let next_row = start_row as i32 + dy;
            let next_col = start_col as i32 + dx;
            if self.can_move(next_row, next_col, symbol, color) {
                has_neighbor = true;
                break;
            }
        }
        if !has_neighbor {
            return 4; // Single cell is a square
        }

        // Find initial direction
        let mut dir = 0;
        let mut curr_row = start_row as i32;
        let mut curr_col = start_col as i32;

        // Find first valid direction
        let mut found_direction = false;
        while dir < 4 {
            let next_row = curr_row + directions[dir].1;
            let next_col = curr_col + directions[dir].0;
            if self.can_move(next_row, next_col, symbol, color) {
                found_direction = true;
                break;
            }
            dir = (dir + 1) % 4;
        }

        if !found_direction {
            return 4; // Fallback if we can't find a valid direction
        }

        let mut sides = 0;

        // Make the first move
        let mut curr_row = curr_row + directions[dir].1;
        let mut curr_col = curr_col + directions[dir].0;
        let start_row = curr_row;
        let start_col = curr_col;
        let start_dir = dir;

        let mut steps = 0;
        loop {
            steps += 1;

            println!(
                "  Step {}: At ({}, {}), facing dir {}, sides={}",
                steps, curr_row, curr_col, dir, sides
            );

            // Try to turn left first
            let left_dir = (dir + 3) % 4;
            let next_row = curr_row + directions[left_dir].1;
            let next_col = curr_col + directions[left_dir].0;

            if self.can_move(next_row, next_col, symbol, color) {
                // Can turn left
                sides += 1;
                dir = left_dir;
                curr_row = next_row;
                curr_col = next_col;
            } else {
                // Try going straight
                let next_row = curr_row + directions[dir].1;
                let next_col = curr_col + directions[dir].0;

                if self.can_move(next_row, next_col, symbol, color) {
                    // Can go straight
                    curr_row = next_row;
                    curr_col = next_col;
                } else {
                    // Must turn right
                    loop {
                        // we increment every time we turn right, even if we don't move
                        sides += 1;
                        dir = (dir + 1) % 4;
                        let next_row = curr_row + directions[dir].1;
                        let next_col = curr_col + directions[dir].0;
                        if self.can_move(next_row, next_col, symbol, color) {
                            curr_row = next_row;
                            curr_col = next_col;
                            break;
                        }
                    }
                }
            }

            if curr_row == start_row && curr_col == start_col && dir == start_dir {
                break;
            }
        }

        sides
    }

    fn can_move(&self, row: i32, col: i32, symbol: char, color: i8) -> bool {
        if row >= 0 && row < self.rows as i32 && col >= 0 && col < self.cols as i32 {
            let cell = &self.cells[row as usize][col as usize];
            return cell.symbol == symbol && cell.color == color;
        }
        false
    }

    fn get_next_color(last_color: i8) -> i8 {
        // Use a prime number to get good distribution
        ((last_color + 7) % 16) + 1
    }

    // Return (perimeter, area)
    fn depth_first_search(&mut self, row: usize, col: usize, color: i8) -> (i32, i32) {
        let current_symbol = self.cells[row][col].symbol;
        let mut perimeter = 4; // Start with 4 for new cell
        let mut area = 1; // Start with area of 1 for current cell
        self.cells[row][col].set_color(color);

        // Check all four directions: up, right, down, left
        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        for (dx, dy) in directions.iter() {
            let new_row = row as i32 + dy;
            let new_col = col as i32 + dx;

            // Check bounds
            if new_row >= 0
                && new_row < self.rows as i32
                && new_col >= 0
                && new_col < self.cols as i32
            {
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                let neighbor = &self.cells[new_row][new_col];

                if neighbor.symbol == current_symbol {
                    perimeter -= 1; // Decrease perimeter for matching symbol

                    if neighbor.color == -1 {
                        // Recursively search this direction
                        let (sub_perimeter, sub_area) =
                            self.depth_first_search(new_row, new_col, color);
                        perimeter += sub_perimeter;
                        area += sub_area;
                    }
                }
            }
        }
        (perimeter, area)
    }

    fn color_all_regions(&mut self) {
        let mut last_color = 0;
        let mut total_score = 0;
        let mut total_score2 = 0;
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.cells[row][col].color == -1 {
                    last_color = Self::get_next_color(last_color);
                    let (perimeter, area) = self.depth_first_search(row, col, last_color);
                    let sides = self.count_sides(row, col);
                    let region_score = perimeter * area;
                    let region_score2 = area * sides;
                    total_score += region_score;
                    total_score2 += region_score2;
                    println!(
                        "Region at ({}, {}) with color {} has perimeter {}, area {}, sides {}, score {}, score2 {}",
                        row, col, last_color, perimeter, area, sides, region_score, region_score2
                    );
                }
            }
        }
        println!("\nTotal score (sum of perimeter * area): {}", total_score);
        println!("Total score2 (sum of area * sides): {}", total_score2);
    }

    fn print_colored(&self) {
        for row in &self.cells {
            for cell in row {
                // ANSI color codes from 31-36, 91-96 for bright colors
                let color_code = if cell.color <= 6 {
                    30 + cell.color
                } else {
                    90 + (cell.color - 6)
                };
                print!("\x1b[{}m{}\x1b[0m", color_code, cell.symbol);
            }
            println!();
        }
    }
}

fn main() -> io::Result<()> {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    println!("Hello, world!");
    // Read the grid once
    let path = Path::new("data2");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let data: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    // Convert the raw data into a grid of Cells
    let grid_data: Vec<Vec<Cell>> = data
        .iter()
        .map(|line| line.chars().map(|c| Cell::new(c)).collect())
        .collect();

    let mut grid = Grid::new(grid_data);
    grid.color_all_regions();
    println!("\nFinal colored grid:");
    grid.print_colored();
    Ok(())
}
