use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, BufRead};

const X_DIM: usize = 71;
const Y_DIM: usize = 71;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    cost: usize,
}

// Implementation for priority queue
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for (dx, dy) in directions {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;

        if new_x >= 0 && new_x < X_DIM as i32 && new_y >= 0 && new_y < Y_DIM as i32 {
            neighbors.push((new_x as usize, new_y as usize));
        }
    }
    neighbors
}

fn find_path(grid: &Vec<Vec<char>>) -> Option<Vec<(usize, usize)>> {
    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut came_from = HashMap::new();

    // Start point
    heap.push(Point {
        x: 0,
        y: 0,
        cost: 0,
    });
    distances.insert((0, 0), 0);

    while let Some(Point { x, y, cost }) = heap.pop() {
        if x == X_DIM - 1 && y == Y_DIM - 1 {
            // Reconstruct path
            let mut path = Vec::new();
            let mut current = (x, y);
            while let Some(&prev) = came_from.get(&current) {
                path.push(current);
                current = prev;
            }
            path.push((0, 0));
            path.reverse();
            return Some(path);
        }

        if cost > *distances.get(&(x, y)).unwrap_or(&std::usize::MAX) {
            continue;
        }

        for (new_x, new_y) in get_neighbors(x, y) {
            if grid[new_y][new_x] == '#' {
                continue;
            }

            let new_cost = cost + 1;
            if new_cost < *distances.get(&(new_x, new_y)).unwrap_or(&std::usize::MAX) {
                distances.insert((new_x, new_y), new_cost);
                came_from.insert((new_x, new_y), (x, y));
                heap.push(Point {
                    x: new_x,
                    y: new_y,
                    cost: new_cost,
                });
            }
        }
    }
    None
}

fn print_grid(grid: &Vec<Vec<char>>, path: &[(usize, usize)]) {
    let mut display_grid = grid.clone();
    for &(x, y) in path {
        if display_grid[y][x] == '.' {
            display_grid[y][x] = 'o';
        }
    }

    for row in &display_grid {
        for &cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

fn main() -> io::Result<()> {
    let file = File::open("data2")?;
    let reader = io::BufReader::new(file);
    let mut grid = vec![vec!['.'; X_DIM]; Y_DIM];

    let mut current_path: Option<Vec<(usize, usize)>> = None;

    'outer: for line in reader.lines() {
        let line = line?;
        let mut parts = line.split(',');
        if let (Some(x), Some(y)) = (parts.next(), parts.next()) {
            let x: usize = x.trim().parse().unwrap();
            let y: usize = y.trim().parse().unwrap();

            // Add the new coordinate to the grid
            if x < X_DIM && y < Y_DIM {
                grid[y][x] = '#';
            }

            // If we have a current path, check if this coordinate blocks it
            if let Some(ref path) = current_path {
                if path.contains(&(x, y)) {
                    // Path is blocked, try to find a new path
                    match find_path(&grid) {
                        Some(new_path) => {
                            current_path = Some(new_path);
                        }
                        None => {
                            println!("No more paths available!");
                            println!("Last coordinate read: ({}, {})", x, y);
                            break 'outer;
                        }
                    }
                }
            } else {
                // First time through, try to find initial path
                current_path = find_path(&grid);
                if current_path.is_none() {
                    println!("No initial path available!");
                    println!("Last coordinate read: ({}, {})", x, y);
                    break;
                }
            }
        }
    }

    // Print final state
    if let Some(path) = current_path {
        println!("Final grid with last valid path:");
        print_grid(&grid, &path);
        println!("Path length: {}", path.len() - 1);
    }

    Ok(())
}
