use std::fs::read_to_string;

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
    let width = lines[0].len() * 2; // Double width for expanded map
    let mut robot: Item = Item {
        x: 0,
        y: 0,
        thing: Thing::Robot,
    };

    let mut map: Vec<Vec<Item>> = Vec::with_capacity(height);

    for (y, line) in lines.iter().enumerate() {
        map.push(Vec::with_capacity(width));
        for (x, ch) in line.chars().enumerate() {
            let real_x = x * 2; // Double x for expanded map

            if ch == '@' {
                robot = Item {
                    x: real_x as i32,
                    y: y as i32,
                    thing: Thing::Robot,
                };
                map[y].push(Item {
                    x: real_x as i32,
                    y: y as i32,
                    thing: Thing::Robot,
                });
                map[y].push(Item {
                    x: (real_x + 1) as i32,
                    y: y as i32,
                    thing: Thing::Empty,
                });
            } else {
                match ch {
                    '#' => {
                        // ## for walls
                        map[y].push(Item {
                            x: real_x as i32,
                            y: y as i32,
                            thing: Thing::Wall,
                        });
                        map[y].push(Item {
                            x: (real_x + 1) as i32,
                            y: y as i32,
                            thing: Thing::Wall,
                        });
                    }
                    'O' => {
                        // Create a single Item for both halves of the box
                        let box_item = Item {
                            x: real_x as i32, // Store left coordinate
                            y: y as i32,
                            thing: Thing::Box,
                        };
                        // Both map positions point to the same Item
                        map[y].push(box_item);
                        map[y].push(box_item);
                    }
                    _ => {
                        // .. for empty space
                        map[y].push(Item {
                            x: real_x as i32,
                            y: y as i32,
                            thing: Thing::Empty,
                        });
                        map[y].push(Item {
                            x: (real_x + 1) as i32,
                            y: y as i32,
                            thing: Thing::Empty,
                        });
                    }
                }
            }
        }
    }

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

fn find_adjacent_boxes(map: &Map, start_x: i32, start_y: i32, movement: &Move) -> Vec<(i32, i32)> {
    let mut boxes = Vec::new();
    let mut to_check = vec![(start_x, start_y)];
    let mut checked = Vec::new();

    while let Some((x, y)) = to_check.pop() {
        if checked.contains(&(x, y)) {
            continue;
        }
        checked.push((x, y));

        // If this is a box, add its left coordinate
        if matches!(map[y as usize][x as usize].thing, Thing::Box) {
            let box_left_x = map[y as usize][x as usize].x; // Get stored left coordinate
            if !boxes.contains(&(box_left_x, y)) {
                boxes.push((box_left_x, y));
            }

            let next_positions = match (movement.x, movement.y) {
                (1, 0) => vec![(x + 2, y)],  // right: check right
                (-1, 0) => vec![(x - 2, y)], // left: check left
                (0, _) => {
                    let mut positions = Vec::new();
                    let check_y = y + movement.y;
                    let current_box_x = map[y as usize][x as usize].x;

                    // For vertical movement, check only positions that could actually overlap
                    // That's current_box_x-1, current_box_x, and current_box_x+1
                    for check_x in (current_box_x - 1)..=(current_box_x + 1) {
                        if check_x >= 0 {
                            let check_x = check_x as usize;
                            if matches!(map[check_y as usize][check_x].thing, Thing::Box) {
                                let box_x = map[check_y as usize][check_x].x;
                                // Only add if this box actually overlaps with our current box
                                if (box_x == current_box_x) ||  // Same position
                                   (box_x + 1 == current_box_x) ||  // Box to our left
                                   (box_x == current_box_x + 1)
                                {
                                    // Box to our right
                                    positions.push((box_x, check_y));
                                }
                            }
                        }
                    }
                    positions
                }
                _ => vec![],
            };

            for (new_x, new_y) in next_positions {
                to_check.push((new_x, new_y));
            }
        }
    }
    boxes
}

fn can_move_adjacent_boxes(map: &Map, boxes: &Vec<(i32, i32)>, movement: &Move) -> bool {
    for &(x, y) in boxes {
        let target_x = x + movement.x;
        let target_y = y + movement.y;

        println!(
            "Checking box at ({}, {}) moving to ({}, {})",
            x, y, target_x, target_y
        );

        // Check both positions the box will occupy
        for dx in 0..2 {
            let check_x = target_x + dx;
            // Check for walls or other obstacles
            match map[target_y as usize][check_x as usize].thing {
                Thing::Empty => continue,
                Thing::Wall => {
                    println!(
                        "Box at ({}, {}) blocked by wall at ({}, {})",
                        x, y, check_x, target_y
                    );
                    return false;
                }
                Thing::Box => {
                    let blocking_box_x = map[target_y as usize][check_x as usize].x;
                    // If we hit a box, check if either its left or right position is part of our moving group
                    if !boxes.contains(&(blocking_box_x, target_y)) {
                        println!(
                            "Box at ({}, {}) blocked by unconnected box at ({}, {})",
                            x, y, blocking_box_x, target_y
                        );
                        return false;
                    }
                }
                _ => {
                    println!(
                        "Box at ({}, {}) blocked by unknown obstacle at ({}, {})",
                        x, y, check_x, target_y
                    );
                    return false;
                }
            }
        }
    }
    true
}

fn move_box(map: &mut Map, x: i32, y: i32, movement: &Move) {
    let mut curr_box = map[y as usize][x as usize];

    // Clear old positions

    map[curr_box.y as usize][curr_box.x as usize].thing = Thing::Empty;
    map[curr_box.y as usize][(curr_box.x + 1) as usize] =
        map[curr_box.y as usize][curr_box.x as usize];

    // Create new box at new position
    curr_box.y += movement.y;
    curr_box.x += movement.x;

    // Set both halves to point to the same new box
    map[curr_box.y as usize][curr_box.x as usize] = curr_box;
    map[curr_box.y as usize][(curr_box.x + 1) as usize] = curr_box;
}

fn move_adjacent_boxes(map: &mut Map, boxes: &Vec<(i32, i32)>, movement: &Move) {
    // Sort boxes by distance in direction of movement (furthest first)
    let mut sorted_boxes = boxes.clone();
    sorted_boxes.sort_by_key(|&(x, y)| {
        if movement.x > 0 {
            -x // Moving right: rightmost first
        } else if movement.x < 0 {
            x // Moving left: leftmost first
        } else if movement.y > 0 {
            -y // Moving down: bottommost first
        } else {
            y // Moving up: topmost first
        }
    });

    // Move each box (boxes array already contains only left coordinates)
    for &(x, y) in &sorted_boxes {
        move_box(map, x, y, movement);
    }
}

fn move_robot(map: &mut Map, robot: &mut Item, movement: &Move) {
    let new_x = robot.x + movement.x;
    let new_y = robot.y + movement.y;

    let direction = match (movement.x, movement.y) {
        (-1, 0) => "< ",
        (1, 0) => "> ",
        (0, -1) => "^ ",
        (0, 1) => "v ",
        _ => "? ",
    };
    print!("{}", direction);

    // Check if we're moving into a box
    let is_box_here = matches!(map[new_y as usize][new_x as usize].thing, Thing::Box);

    if is_box_here {
        // Find all connected boxes
        let boxes = find_adjacent_boxes(map, new_x, new_y, movement);

        // Check if we can move all the boxes
        if can_move_adjacent_boxes(map, &boxes, movement) {
            move_adjacent_boxes(map, &boxes, movement);
            update_robot_position(map, robot, new_x, new_y);
        } else {
            print!("Can't move: Box stack is blocked!");
        }
    } else if matches!(map[new_y as usize][new_x as usize].thing, Thing::Empty) {
        update_robot_position(map, robot, new_x, new_y);
    } else if matches!(map[new_y as usize][new_x as usize].thing, Thing::Wall) {
        print!("Can't move: There's a wall in the way!");
    } else {
        print!("Can't move: Path is blocked!");
    }
}

fn draw_map(map: &Map, robot: Option<&Item>) {
    println!();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if let Some(robot) = robot {
                if x as i32 == robot.x && y as i32 == robot.y {
                    print!("@");
                    continue;
                }
            }

            match map[y][x].thing {
                Thing::Box => {
                    if x as i32 == map[y][x].x {
                        print!("(");
                    } else {
                        print!(")");
                    }
                }
                Thing::Wall => print!("#"),
                Thing::Empty => print!("."),
                Thing::Robot => print!("@"),
            }
        }
        println!();
    }
    println!();
}

fn gps_sum(map: &Map) -> i32 {
    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            // Only count boxes at their left position
            if matches!(map[y][x].thing, Thing::Box) && (x as i32 == map[y][x].x) {
                sum += (y as i32 * 100) + x as i32;
            }
        }
    }
    sum
}

fn main() {
    let content = read_to_string("data2").expect("Could not read file");
    let parts: Vec<&str> = content.split("\n\n").collect();

    let (mut map, mut robot) = parse_map(&parts[0]);
    let moves = parse_moves(&parts[1]);

    println!("Initial state:");
    draw_map(&map, Some(&robot));

    for movement in moves {
        move_robot(&mut map, &mut robot, &movement);
        draw_map(&map, Some(&robot));
    }
    println!("GPS Sum: {}", gps_sum(&map));
}
