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
    const MAX_JUMP: i32 = 20;

    for start_y in 0..map.len() {
        for start_x in 0..map[0].len() {
            if matches!(map[start_y][start_x].thing, Thing::Empty | Thing::Start) {
                let start_jump = (start_x as i32, start_y as i32);

                // Skip if we can't reach this position from start
                if !start_distances.contains_key(&start_jump) {
                    continue;
                }

                // Check all positions within Manhattan distance of MAX_JUMP
                for end_y in 0..map.len() {
                    for end_x in 0..map[0].len() {
                        let end_jump = (end_x as i32, end_y as i32);

                        // Calculate Manhattan distance
                        let manhattan_dist =
                            (end_jump.0 - start_jump.0).abs() + (end_jump.1 - start_jump.1).abs();

                        // Skip if jump is too long or to the same position
                        if manhattan_dist == 0 || manhattan_dist > MAX_JUMP {
                            continue;
                        }

                        // Skip walls and unreachable positions
                        if matches!(map[end_y][end_x].thing, Thing::Wall)
                            || !end_distances.contains_key(&end_jump)
                        {
                            continue;
                        }

                        // Calculate total path length with jump
                        let path_length = start_distances[&start_jump] // Start to jump point
                            + manhattan_dist  // Cost of jump
                            + end_distances[&end_jump]; // Jump endpoint to end

                        if path_length < reference_cost {
                            shortcuts.push(JumpResult {
                                from: start_jump,
                                to: end_jump,
                                total_length: path_length,
                                start_to_jump: start_distances[&start_jump],
                                jump_to_end: end_distances[&end_jump],
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
                    /*
                    println!("\nFound shorter path:");
                    println!("  Jump from {:?} to {:?}", shortcut.from, shortcut.to);
                    println!("  New path length: {}", shortcut.total_length);
                    println!("  Saves {} steps", reference_cost - shortcut.total_length);
                    println!("  Path segments:");
                    println!("    Start to jump point: {} steps", shortcut.start_to_jump);
                    let manhattan_dist = (shortcut.to.0 - shortcut.from.0).abs()
                        + (shortcut.to.1 - shortcut.from.1).abs();
                    println!("    Jump cost: {} steps", manhattan_dist);
                    println!("    After jump to end: {} steps", shortcut.jump_to_end);
                    */
                    Some(())
                }
                false => None,
            },
        )
        .count();

    println!("\nFound {} good shortcuts", count);
}
