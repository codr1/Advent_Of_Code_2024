use std::fs::read_to_string;
use std::sync::atomic::{AtomicI32, Ordering};

static GRID_X: AtomicI32 = AtomicI32::new(0);
static GRID_Y: AtomicI32 = AtomicI32::new(0);

#[derive(Debug, Copy, Clone)]
enum Thing {
    Box,
    Wall,
    Robot,
    Empty,
}

#[derive(Debug, Copy, Clone)]
struct Item {
    x: i32,
    y: i32,
    thing: Thing,
}

#[derive(Debug, Copy, Clone)]
struct Move {
    x: i32,
    y: i32,
}

type Map = Vec<Vec<Item>>;

fn parse_map(content: &str) -> (Map, Item) {
    let lines: Vec<&str> = content.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut robot: Item = Item {
        x: 0,
        y: 0,
        thing: Thing::Robot,
    };

    let mut map: Vec<Vec<Item>> = Vec::with_capacity(height);

    for (y, line) in lines.iter().enumerate() {
        map.push(Vec::with_capacity(width));
        for (x, ch) in line.chars().enumerate() {
            if ch == '@' {
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
                    'O' => Thing::Box,
                    '@' => Thing::Robot,
                    _ => Thing::Empty,
                },
            })
        }
    }

    GRID_Y.store(map.len() as i32, Ordering::SeqCst);
    GRID_X.store(map[0].len() as i32, Ordering::SeqCst);

    (map, robot)
}

fn parse_moves(content: &str) -> Vec<Move> {
    content
        .chars()
        .filter_map(|ch| match ch {
            '^' => Some(Move { x: 0, y: -1 }),
            'v' => Some(Move { x: 0, y: 1 }),
            '<' => Some(Move { x: -1, y: 0 }),
            '>' => Some(Move { x: 1, y: 0 }),
            _ => None,
        })
        .collect()
}

fn update_robot_position(map: &mut Map, robot: &mut Item, new_x: i32, new_y: i32) {
    // Clear old position
    map[robot.y as usize][robot.x as usize].thing = Thing::Empty;
    // Update robot position
    robot.x = new_x;
    robot.y = new_y;
    // Set new position
    map[new_y as usize][new_x as usize].thing = Thing::Robot;
}

fn move_robot(map: &mut Map, robot: &mut Item, movement: &Move) {
    let new_x = robot.x + movement.x;
    let new_y = robot.y + movement.y;

    // Check bounds
    if new_x < 0
        || new_y < 0
        || new_x >= GRID_X.load(Ordering::SeqCst)
        || new_y >= GRID_Y.load(Ordering::SeqCst)
    {
        return;
    }

    match map[new_y as usize][new_x as usize].thing {
        Thing::Empty => {
            update_robot_position(map, robot, new_x, new_y);
        }
        Thing::Box => {
            let mut box_count = 0;
            let mut curr_x = new_x;
            let mut curr_y = new_y;

            // Count boxes and find end of stack
            loop {
                if curr_x < 0
                    || curr_y < 0
                    || curr_x >= GRID_X.load(Ordering::SeqCst)
                    || curr_y >= GRID_Y.load(Ordering::SeqCst)
                {
                    return; // Out of bounds
                }

                match map[curr_y as usize][curr_x as usize].thing {
                    Thing::Box => {
                        box_count += 1;
                        curr_x += movement.x;
                        curr_y += movement.y;
                    }
                    Thing::Empty => {
                        // Found empty space after boxes, can move everything
                        // Start from the last box and move backwards
                        curr_x -= movement.x;
                        curr_y -= movement.y;

                        // Move each box one space forward
                        for _ in 0..box_count {
                            map[(curr_y + movement.y) as usize][(curr_x + movement.x) as usize]
                                .thing = Thing::Box;
                            map[curr_y as usize][curr_x as usize].thing = Thing::Empty;
                            curr_x -= movement.x;
                            curr_y -= movement.y;
                        }

                        // Finally move the robot
                        update_robot_position(map, robot, new_x, new_y);
                        break;
                    }
                    Thing::Wall => return,  // Can't move anything
                    Thing::Robot => return, // Shouldn't happen
                }
            }
        }
        _ => {} // Can't move into walls
    }
}

fn gps_sum(map: &Map) -> i32 {
    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if matches!(map[y][x].thing, Thing::Box) {
                sum += (y as i32 * 100) + x as i32;
            }
        }
    }
    sum
}

fn draw_map(map: &Map, robot: &Item) {
    println!();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if x as i32 == robot.x && y as i32 == robot.y {
                print!("@");
            } else {
                let symbol = match map[y][x].thing {
                    Thing::Wall => '#',
                    Thing::Box => 'O',
                    Thing::Empty => '.',
                    Thing::Robot => '@', // shouldn't happen
                };
                print!("{}", symbol);
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let content = read_to_string("data2").expect("Could not read file");
    let parts: Vec<&str> = content.split("\n\n").collect();

    let (mut map, mut robot) = parse_map(&parts[0]);
    let moves = parse_moves(&parts[1]);

    println!("Initial state:");
    draw_map(&map, &robot);

    for movement in moves {
        move_robot(&mut map, &mut robot, &movement);
        draw_map(&map, &robot);
    }

    println!("GPS Sum: {}", gps_sum(&map));
}
