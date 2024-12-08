use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

struct Grid {
    data: Vec<String>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(data: Vec<String>) -> Self {
        let height = data.len();
        let width = data.first().map_or(0, |line| line.len());
        Self {
            data,
            width,
            height,
        }
    }

    fn char_at(&self, p: Point) -> Option<char> {
        if self.is_in_bounds(p) {
            self.data[p.x as usize].chars().nth(p.y as usize)
        } else {
            None
        }
    }

    fn is_in_bounds(&self, p: Point) -> bool {
        p.x >= 0 && p.x < self.height as i32 && p.y >= 0 && p.y < self.width as i32
    }
}

fn get_unique_symbols(grid: &Grid, filter_symbols: &Option<&HashSet<char>>) -> HashSet<char> {
    grid.data
        .iter()
        .flat_map(|line| line.chars())
        .filter(|&c| c != '.' && c != ' ')
        .filter(|c| filter_symbols.map_or(true, |fs| fs.contains(c)))
        .collect()
}

fn visualize_grid(grid: &Grid, points: &[Point], filter_symbols: &Option<&HashSet<char>>) {
    println!("\nGrid visualization (# = extended points, symbols shown as-is):");
    for i in 0..grid.height {
        for j in 0..grid.width {
            let p = Point {
                x: i as i32,
                y: j as i32,
            };
            let current_char = grid.char_at(p).unwrap_or('.');
            if points.contains(&p) {
                print!("#");
            } else if current_char != '.'
                && filter_symbols.map_or(true, |fs| fs.contains(&current_char))
            {
                print!("{}", current_char);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn find_symbol_pairs(grid: &Grid, filter_symbols: &Option<&HashSet<char>>) -> io::Result<()> {
    // Set to store unique valid extended points
    let mut unique_extended_points = HashSet::new();

    println!("Grid size: {}x{}", grid.width, grid.height);

    // Get unique symbols
    let symbols = get_unique_symbols(grid, filter_symbols);

    // Print the symbols being processed
    println!("\nProcessing symbols:");
    let mut symbol_vec: Vec<_> = symbols.iter().collect();
    symbol_vec.sort(); // Sort for consistent output
    for symbol in &symbol_vec {
        print!("{} ", symbol);
    }
    println!("\n");

    // For each symbol, find all positions
    for symbol in symbols {
        let mut positions = Vec::new();

        // Scan the grid for the symbol
        for (row, line) in grid.data.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == symbol {
                    positions.push(Point {
                        x: row as i32,
                        y: col as i32,
                    });
                }
            }
        }

        // Print pairs for this symbol and their extended points
        println!("\nPairs for symbol '{}':", symbol);
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let p1 = positions[i];
                let p2 = positions[j];

                // Calculate vector between points
                let dx = p2.x - p1.x;
                let dy = p2.y - p1.y;

                // Calculate antinode points
                let ext1 = Point {
                    x: p1.x - dx,
                    y: p1.y - dy,
                };
                let ext2 = Point {
                    x: p2.x + dx,
                    y: p2.y + dy,
                };

                // Store valid extended points
                if grid.is_in_bounds(ext1) {
                    unique_extended_points.insert(ext1);
                }
                if grid.is_in_bounds(ext2) {
                    unique_extended_points.insert(ext2);
                }
            }
        }
    }

    println!("\nUnique valid extended points:");
    let mut points: Vec<_> = unique_extended_points.into_iter().collect();
    points.sort(); // Sort for consistent output
    for point in &points {
        println!("({}, {})", point.x, point.y);
    }

    visualize_grid(grid, &points, filter_symbols);

    println!("\nTotal unique valid extended points: {}", points.len());
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Create filter set from command line arguments if provided
    let filter_symbols = if args.len() > 1 {
        let mut symbols = HashSet::new();
        for symbol in args[1..].iter() {
            if let Some(c) = symbol.chars().next() {
                symbols.insert(c);
            }
        }
        Some(symbols)
    } else {
        None
    };

    // Read the grid once
    let path = Path::new("data2");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let data: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let grid = Grid::new(data);

    find_symbol_pairs(&grid, &filter_symbols.as_ref())
}
