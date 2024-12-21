use std::collections::{BinaryHeap, HashMap};
use std::fs::read_to_string;
use std::process;

#[derive(Debug, PartialEq, Clone)]
enum Thing {
    Empty,
    Wall,
    Start,
    End,
}

#[derive(Debug, Clone)]
struct Item {
    thing: Thing,
}

type Map = Vec<Vec<Item>>;

#[derive(Debug)]
struct JumpResult {
    from: (i32, i32),
    to: (i32, i32),
    total_length: i32,
    start_to_jump: i32,
    jump_to_end: i32,
}

fn read_map() -> Result<Map, std::io::Error> {
    let contents = read_to_string("data2")?;
    let map: Map = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Item {
                    thing: match c {
                        '.' => Thing::Empty,
                        '#' => Thing::Wall,
                        'S' => Thing::Start,
                        'E' => Thing::End,
                        _ => panic!("Invalid character in map: {}", c),
                    },
                })
                .collect()
        })
        .collect();
    Ok(map)
}

fn calculate_distances(map: &Map, start: (i32, i32)) -> HashMap<(i32, i32), i32> {
    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();

    // Start with distance 0 to start position
    distances.insert(start, 0);
    heap.push((0, start));

    while let Some((cost, pos)) = heap.pop() {
        let cost = -cost; // Convert back from max-heap to min-heap

        // Skip if we've found a better path
        if let Some(&best) = distances.get(&pos) {
            if best < cost {
                continue;
            }
        }

        // Check each neighbor
        let (x, y) = pos;
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let new_x = x + dx;
            let new_y = y + dy;

            // Skip if out of bounds
            if new_y < 0
                || (new_y as usize) >= map.len()
                || new_x < 0
                || (new_x as usize) >= map[0].len()
            {
                continue;
            }

            // Skip walls
            if matches!(map[new_y as usize][new_x as usize].thing, Thing::Wall) {
                continue;
            }

            let new_pos = (new_x, new_y);
            let new_cost = cost + 1;

            if !distances.contains_key(&new_pos) || new_cost < distances[&new_pos] {
                distances.insert(new_pos, new_cost);
                heap.push((-new_cost, new_pos));
            }
        }
    }

    distances
}

fn analyze_jumps(
    map: &Map,
    start_distances: &HashMap<(i32, i32), i32>,
    end_distances: &HashMap<(i32, i32), i32>,
    reference_cost: i32,
) -> Vec<JumpResult> {
    let mut shortcuts = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if matches!(map[y][x].thing, Thing::Empty) {
                let current_pos = (x as i32, y as i32);

                // Skip if we can't reach this position from start
                if !start_distances.contains_key(&current_pos) {
                    continue;
                }

                // Check 2 spaces in each direction
                let directions = [(0, -2), (0, 2), (2, 0), (-2, 0)]; // up, down, right, left
                for (dx, dy) in directions.iter() {
                    let jump_x = x as i32 + dx;
                    let jump_y = y as i32 + dy;
                    let jump_pos = (jump_x, jump_y);

                    // Check if jump position is valid and reachable
                    if jump_y >= 0
                        && (jump_y as usize) < map.len()
                        && jump_x >= 0
                        && (jump_x as usize) < map[0].len()
                        && !matches!(map[jump_y as usize][jump_x as usize].thing, Thing::Wall)
                        && end_distances.contains_key(&jump_pos)
                    {
                        // Calculate total path length with jump
                        let path_length = start_distances[&current_pos] // Start to current
                            + 2  // Cost of jump
                            + end_distances[&jump_pos]; // Jump point to end

                        if path_length < reference_cost {
                            shortcuts.push(JumpResult {
                                from: current_pos,
                                to: jump_pos,
                                total_length: path_length,
                                start_to_jump: start_distances[&current_pos],
                                jump_to_end: end_distances[&jump_pos],
                            });
                        }
                    }
                }
            }
        }
    }
    shortcuts
}

fn main() {
    let map = match read_map() {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Error reading map: {}", e);
            process::exit(1);
        }
    };

    // Find start and end positions
    let mut start_pos = None;
    let mut end_pos = None;

    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            match cell.thing {
                Thing::Start => start_pos = Some((x as i32, y as i32)),
                Thing::End => end_pos = Some((x as i32, y as i32)),
                _ => (),
            }
        }
    }

    let start_pos = start_pos.expect("No start position found");
    let end_pos = end_pos.expect("No end position found");

    // Calculate distances from start and end
    let start_distances = calculate_distances(&map, start_pos);
    let end_distances = calculate_distances(&map, end_pos);

    // Calculate reference path length (without jumps)
    let reference_cost = match start_distances.get(&end_pos) {
        Some(cost) => *cost,
        None => {
            println!("No path found from start to end!");
            process::exit(1);
        }
    };

    println!("Reference path length: {}", reference_cost);

    // Analyze possible jumps
    let shortcuts = analyze_jumps(&map, &start_distances, &end_distances, reference_cost);

    let count = shortcuts
        .iter()
        .filter_map(
            |shortcut| match reference_cost - shortcut.total_length >= 100 {
                true => {
                    println!("\nFound shorter path:");
                    println!("  Jump from {:?} to {:?}", shortcut.from, shortcut.to);
                    println!("  New path length: {}", shortcut.total_length);
                    println!("  Saves {} steps", reference_cost - shortcut.total_length);
                    println!("  Path segments:");
                    println!("    Start to jump point: {} steps", shortcut.start_to_jump);
                    println!("    Jump cost: 2 steps");
                    println!("    After jump to end: {} steps", shortcut.jump_to_end);
                    Some(())
                }
                false => None,
            },
        )
        .count();

    println!("\nFound {} good shortcuts", count);
}
