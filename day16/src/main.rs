use std::fs::read_to_string;

#[derive(Debug, Copy, Clone)]
enum Thing {
    Wall,
    Robot,
    End,
    Empty,
}

#[derive(Debug, Copy, Clone)]
struct Item {
    x: i32,
    y: i32,
    thing: Thing,
}
type Map = Vec<Vec<Item>>;
fn parse_map(content: &str) -> (Map, Item, Item) {
    let lines: Vec<&str> = content.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut robot: Item = Item {
        x: 0,
        y: 0,
        thing: Thing::Robot,
    };
    let mut end = Item {
        x: 0,
        y: 0,
        thing: Thing::Empty,
    };
    let mut map: Vec<Vec<Item>> = Vec::with_capacity(height);

    for (y, line) in lines.iter().enumerate() {
        map.push(Vec::with_capacity(width));
        for (x, ch) in line.chars().enumerate() {
            if ch == 'S' {
                robot = Item {
                    x: x as i32,
                    y: y as i32,
                    thing: Thing::Robot,
                };
            }
            map[y].push(Item {
                x: x as i32,
                y: y as i32,
                thing: match ch {
                    '#' => Thing::Wall,
                    'S' => Thing::Robot,
                    'E' => Thing::End,
                    _ => Thing::Empty,
                },
            });
            // If we just pushed E in there, I want to return that item.
            if ch == 'E' {
                end = map[y][map[y].len() - 1];
            }
        }
    }

    (map, robot, end)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    cost: i32,
    position: (i32, i32),
    direction: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse ordering so priority queue becomes a min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_valid_turns(current_dir: Direction, is_first_move: bool) -> Vec<Direction> {
    match (current_dir, is_first_move) {
        (dir, true) => {
            // At first position: straight, left, right, or turn left twice
            let mut turns = vec![
                dir,                       // straight
                turn_left(dir),            // left
                turn_right(dir),           // right
                turn_left(turn_left(dir)), // turn left twice
            ];
            turns.dedup(); // Remove any duplicates
            turns
        }
        (dir, false) => {
            // After first position: straight, left, or right only
            vec![
                dir,             // straight
                turn_left(dir),  // left
                turn_right(dir), // right
            ]
        }
    }
}

fn turn_left(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::West,
        Direction::West => Direction::South,
        Direction::South => Direction::East,
        Direction::East => Direction::North,
    }
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn get_neighbors(
    pos: (i32, i32),
    current_dir: Direction,
    is_first_move: bool,
    map: &Map,
) -> Vec<((i32, i32), Direction)> {
    println!(
        "Getting neighbors for position {:?}, current_dir: {:?}",
        pos, current_dir
    );
    let mut neighbors = Vec::new();

    let valid_turns = get_valid_turns(current_dir, is_first_move);

    for new_dir in valid_turns {
        let (dx, dy) = match new_dir {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        };
        let new_x = pos.0 + dx;
        let new_y = pos.1 + dy;

        if new_y >= 0
            && (new_y as usize) < map.len()
            && new_x >= 0
            && (new_x as usize) < map[0].len()
        {
            if !matches!(map[new_y as usize][new_x as usize].thing, Thing::Wall) {
                neighbors.push(((new_x, new_y), new_dir));
            }
        }
    }
    neighbors
}

fn find_all_paths(map: &Map, start: Item, end: Item, target_cost: i32) -> Vec<Vec<(i32, i32)>> {
    use std::collections::{HashMap, VecDeque};

    #[derive(Clone)]
    struct PathState {
        cost: i32,
        position: (i32, i32),
        direction: Direction,
        path: Vec<(i32, i32)>,
    }

    let mut paths = Vec::new();
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    // Start facing East
    let initial_state = PathState {
        cost: 0,
        position: (start.x, start.y),
        direction: Direction::East,
        path: vec![(start.x, start.y)],
    };
    queue.push_back(initial_state);

    while let Some(state) = queue.pop_front() {
        // Prune if cost exceeds target
        if state.cost > target_cost {
            continue;
        }

        // Found a valid path
        if state.position == (end.x, end.y) && state.cost == target_cost {
            paths.push(state.path);
            continue;
        }

        let is_first_move = state.position == (start.x, start.y);
        for (next_pos, next_dir) in
            get_neighbors(state.position, state.direction, is_first_move, map)
        {
            let move_cost = if state.direction == next_dir { 1 } else { 1001 };
            let next_cost = state.cost + move_cost;

            // Skip if exceeds target cost
            if next_cost > target_cost {
                continue;
            }

            // Create new path state
            let mut next_path = state.path.clone();
            next_path.push(next_pos);

            // Visit state if it's better or equal cost
            let state_key = (next_pos, next_dir);
            if !visited.contains_key(&state_key) || visited[&state_key] >= next_cost {
                visited.insert(state_key, next_cost);
                queue.push_back(PathState {
                    cost: next_cost,
                    position: next_pos,
                    direction: next_dir,
                    path: next_path,
                });
            }
        }
    }
    paths
}

fn find_path(map: &Map, start: Item, end: Item) -> Option<(i32, Vec<Vec<(i32, i32)>>)> {
    use std::collections::{BinaryHeap, HashMap};

    // First find shortest path cost using Dijkstra's
    let mut min_cost = i32::MAX;
    let mut best_paths: Vec<Vec<(i32, i32)>> = Vec::new();

    let mut heap = BinaryHeap::new();
    let mut costs = HashMap::new();

    // Start facing East with allowed initial moves
    let initial_dir = Direction::East;
    let initial_state = State {
        cost: 0,
        position: (start.x, start.y),
        direction: initial_dir,
    };
    heap.push(initial_state);
    costs.insert(((start.x, start.y), initial_dir), 0);

    let mut came_from: HashMap<((i32, i32), Direction), ((i32, i32), Direction)> = HashMap::new();

    let mut iteration = 0;
    while let Some(State {
        cost,
        position,
        direction,
    }) = heap.pop()
    {
        iteration += 1;
        println!(
            "Iteration {}: At {:?} facing {:?} cost={} queue_size={}",
            iteration,
            position,
            direction,
            cost,
            heap.len()
        );

        // Add safety limit
        if iteration > 1000000 {
            println!("Exceeded iteration limit - possible infinite loop");
            return None;
        }
        // Skip if we've found a better path to this position+direction
        if let Some(&best_cost) = costs.get(&(position, direction)) {
            if cost > best_cost {
                continue;
            }
        }

        println!(
            "  Checking if at end: current={:?}, end=({},{})",
            position, end.x, end.y
        );
        if position == (end.x, end.y) {
            println!("Found end position with cost {}!", cost);
            // Reconstruct path
            let mut path = Vec::new();
            let mut current_state = (position, direction);
            path.push(current_state.0);

            println!("Reconstructing path:");
            println!("  End: {:?} facing {:?}", current_state.0, current_state.1);

            while let Some(&prev_state) = came_from.get(&current_state) {
                println!("  Previous: {:?} facing {:?}", prev_state.0, prev_state.1);
                path.push(prev_state.0);
                current_state = prev_state;

                if current_state.0 == (start.x, start.y) {
                    println!("  Reached start!");
                    break;
                }
            }

            path.reverse();
            println!(
                "Found path with {} steps and cost {}: {:?}",
                path.len(),
                cost,
                path
            );

            // Update best paths collection
            if cost < min_cost {
                // Found a better cost, clear all previous paths
                min_cost = cost;
                best_paths.clear();
                best_paths.push(path);
                println!("New minimum cost found: {}! Cleared previous paths.", cost);
            } else if cost == min_cost {
                // Found another path with the same minimum cost
                best_paths.push(path);
                println!("Found another path with minimum cost {}!", cost);
            }

            // Continue searching for other possible paths
            continue;
        }

        let is_first_move = position == (start.x, start.y);
        for (next_pos, next_dir) in get_neighbors(position, direction, is_first_move, map) {
            let move_cost = if direction == next_dir { 1 } else { 1001 };
            let next_cost = cost + move_cost;

            let current_best = costs.get(&(next_pos, next_dir));
            let is_better = current_best.map_or(true, |&c| next_cost < c);

            println!("  Considering move to {:?} facing {:?}", next_pos, next_dir);
            println!(
                "    New cost would be: {} (turn cost: {})",
                next_cost, move_cost
            );
            println!("    Current best cost: {:?}", current_best);
            println!("    Better? {}", is_better);

            if is_better {
                costs.insert((next_pos, next_dir), next_cost);
                came_from.insert((next_pos, next_dir), (position, direction));
                heap.push(State {
                    cost: next_cost,
                    position: next_pos,
                    direction: next_dir,
                });
            }
        }
    }

    if !best_paths.is_empty() {
        // Now that we have the minimum cost, do BFS to find all paths with that cost
        println!("\nDoing BFS to find all paths with cost {}", min_cost);
        let all_paths = find_all_paths(map, start, end, min_cost);
        println!("BFS found {} paths", all_paths.len());
        Some((min_cost, all_paths))
    } else {
        None
    }
}

fn main() {
    let content = read_to_string("data2").expect("Failed to read input file");
    let (map, robot, end) = parse_map(&content);

    println!(
        "Starting search from {:?} to {:?}",
        (robot.x, robot.y),
        (end.x, end.y)
    );
    println!("Map dimensions: {}x{}", map.len(), map[0].len());
    match find_path(&map, robot, end) {
        Some((cost, paths)) => {
            println!("\nFINAL RESULTS:");
            println!("Shortest path cost: {}", cost);
            println!("Number of different shortest paths: {}", paths.len());

            // Print individual paths
            for (i, path) in paths.iter().enumerate() {
                println!("Path {}: {:?}", i + 1, path);
            }

            // Find unique tiles across all paths
            use std::collections::HashSet;
            let mut unique_tiles = HashSet::new();

            // Add all tiles from all paths
            for path in paths.iter() {
                unique_tiles.extend(path.iter().cloned());
            }

            println!("\nPath Analysis:");
            println!("Total optimal paths found: {}", paths.len());
            println!(
                "Number of unique tiles used across all paths: {}",
                unique_tiles.len()
            );
            println!("Unique tiles: {:?}", unique_tiles);

            // Print map with paths
            println!("\nMap with all shortest paths:");

            // Create a set of all path positions
            let mut path_positions: HashSet<(i32, i32)> = HashSet::new();
            for path in paths.iter() {
                path_positions.extend(path.iter().cloned());
            }

            // ANSI color codes
            const BLUE: &str = "\x1b[34m";
            const WHITE: &str = "\x1b[97m";
            const RESET: &str = "\x1b[0m";

            // Print the map
            for y in 0..map.len() {
                for x in 0..map[0].len() {
                    let pos = (x as i32, y as i32);
                    match map[y][x].thing {
                        Thing::Wall => print!("{BLUE}#{RESET}"),
                        Thing::Robot => print!("S"),
                        Thing::End => print!("E"),
                        Thing::Empty => {
                            if path_positions.contains(&pos) {
                                print!("{WHITE}█{RESET}");
                            } else {
                                print!(".");
                            }
                        }
                    };
                }
                println!(); // New line after each row
            }
        }
        None => {
            println!("No path found!");
        }
    }
}
