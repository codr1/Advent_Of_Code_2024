use std::fs::read_to_string;

//const GRID_X: i32 = 11;
//const GRID_Y: i32 = 7;
const GRID_X: i32 = 101;
const GRID_Y: i32 = 103;

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    x_v: i32,
    y_v: i32,
}

fn parse_line(line: &str) -> Option<Robot> {
    let parts: Vec<&str> = line.split(" ").collect();
    if parts.len() != 2 {
        return None;
    }

    let coords = parts[0].trim();
    let x_y: Vec<&str> = coords.split(",").collect();
    if x_y.len() != 2 {
        return None;
    }

    let x = x_y[0].trim().trim_start_matches("p=").parse::<i32>().ok()?;
    let y = x_y[1].trim().parse::<i32>().ok()?;

    let velocity = parts[1].trim();
    let v_parts: Vec<&str> = velocity.split(',').collect();
    if v_parts.len() != 2 {
        return None;
    }

    let x_v = v_parts[0]
        .trim()
        .trim_start_matches("v=")
        .parse::<i32>()
        .ok()?;
    let y_v = v_parts[1].trim().parse::<i32>().ok()?;

    Some(Robot { x, y, x_v, y_v })
}

fn parse_robots(content: &str) -> Vec<Robot> {
    let mut robots = Vec::new();

    for line in content.lines() {
        robots.push(parse_line(line).expect("Failed to parse robot"));
    }

    robots
}

fn move_robots(robots: &mut Vec<Robot>) {
    for robot in robots.iter_mut() {
        // Handle negative coordinates by adding grid size before modulo
        robot.x = ((robot.x + robot.x_v) % GRID_X + GRID_X) % GRID_X;
        robot.y = ((robot.y + robot.y_v) % GRID_Y + GRID_Y) % GRID_Y;
    }
}

fn draw_map(robots: &Vec<Robot>) {
    // Initialize grid with zeros
    let mut grid = vec![vec![0; GRID_Y as usize]; GRID_X as usize];

    // Increment grid positions for each robot
    for robot in robots {
        grid[robot.x as usize][robot.y as usize] += 1;
    }

    // Print the grid
    for y in 0..GRID_Y {
        for x in 0..GRID_X {
            let count = grid[x as usize][y as usize];
            if count == 0 {
                print!(".");
            } else {
                print!("{}", count);
            }
        }
        println!(); // New line after each row
    }
    println!(); // Empty line after the grid
}

fn find_symmetry(robots: &Vec<Robot>) -> bool {
    let mid_x = GRID_X / 2;
    let mid_y = GRID_Y / 2;

    // Check each robot in left half
    for robot in robots {
        // Skip robots on or below middle line
        if robot.y >= mid_y || robot.x >= mid_x {
            continue;
        }

        // Calculate mirror position
        let mirror_x = GRID_X - 1 - robot.x;

        // Look for matching robot at mirror position
        let has_mirror = robots
            .iter()
            .any(|other| other.x == mirror_x && other.y == robot.y);

        if !has_mirror {
            return false;
        }
    }

    true
}

fn calc_risk(robots: &Vec<Robot>) -> i32 {
    let mid_x = GRID_X / 2;
    let mid_y = GRID_Y / 2;
    let mut quadrant_counts = [0; 4]; // [top_left, top_right, bottom_left, bottom_right]

    for robot in robots {
        // Skip robots on the dividing lines
        if robot.x == mid_x || robot.y == mid_y {
            continue;
        }

        // Determine which quadrant the robot is in
        let quadrant = match (robot.x < mid_x, robot.y < mid_y) {
            (true, true) => 0,   // top-left
            (false, true) => 1,  // top-right
            (true, false) => 2,  // bottom-left
            (false, false) => 3, // bottom-right
        };

        quadrant_counts[quadrant] += 1;
    }

    // Multiply all quadrant counts together
    quadrant_counts.iter().product()
}

fn main() {
    let num_steps = 100;
    let content = read_to_string("data2").expect("Could not read file");
    let mut robots = parse_robots(&content);

    for _ in 0..num_steps {
        move_robots(&mut robots);
        draw_map(&robots);
    }

    let risk = calc_risk(&robots);
    println!("Risk factor: {}", risk);

    // let mut map = vec![vec![Vec::new(); GRID_Y as usize]; GRID_X as usize];
}
