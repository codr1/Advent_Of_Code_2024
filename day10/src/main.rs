use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct TrailHead {
    x: usize,
    y: usize,
    score: u32,
    score_nv: u32,
}

fn search_path_with_visit(
    map: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
    current_value: u32,
    visited: &mut Vec<Vec<bool>>,
) -> u32 {
    if visited[y][x] {
        return 0;
    }
    visited[y][x] = true;

    // If we found a 9, increment score and return
    if map[y][x] == 9 {
        return 1;
    }

    let mut score = 0;
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)]; // up, right, down, left

    for (dx, dy) in directions {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;

        // Check bounds
        if new_x >= 0 && new_x < map[0].len() as i32 && new_y >= 0 && new_y < map.len() as i32 {
            let new_x = new_x as usize;
            let new_y = new_y as usize;

            // Check if the adjacent tile is exactly one bigger
            if map[new_y][new_x] == current_value + 1 {
                score += search_path_with_visit(map, new_x, new_y, map[new_y][new_x], visited);
            }
        }
    }

    score
}

fn search_path_no_visit(map: &Vec<Vec<u32>>, x: usize, y: usize, current_value: u32) -> u32 {
    // If we found a 9, increment score and return
    if map[y][x] == 9 {
        return 1;
    }

    let mut score = 0;
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)]; // up, right, down, left

    for (dx, dy) in directions {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;

        // Check bounds
        if new_x >= 0 && new_x < map[0].len() as i32 && new_y >= 0 && new_y < map.len() as i32 {
            let new_x = new_x as usize;
            let new_y = new_y as usize;

            // Check if the adjacent tile is exactly one bigger
            if map[new_y][new_x] == current_value + 1 {
                score += search_path_no_visit(map, new_x, new_y, map[new_y][new_x]);
            }
        }
    }

    score
}

fn main() {
    let path = Path::new("data2");
    let mut map: Vec<Vec<u32>> = Vec::new();

    if let Ok(file) = File::open(path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                map.push(line.chars().filter_map(|c| c.to_digit(10)).collect());
            }
        }
    }

    // Find all trailheads (positions with value 0)
    let mut trailheads = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 0 {
                let mut visited = vec![vec![false; map[0].len()]; map.len()];
                let score_with_visit = search_path_with_visit(&map, x, y, 0, &mut visited);
                let score_no_visit = search_path_no_visit(&map, x, y, 0);
                trailheads.push(TrailHead {
                    x,
                    y,
                    score: score_with_visit,
                    score_nv: score_no_visit,
                });
                println!(
                    "Trailhead at ({}, {}) scores: with_visit={}, no_visit={}",
                    x, y, score_with_visit, score_no_visit
                );
            }
        }
    }

    // Sum up all scores
    let total_score: u32 = trailheads.iter().map(|head| head.score).sum();
    let total_score_nv: u32 = trailheads.iter().map(|head| head.score_nv).sum();
    println!("\nTotal score: {}", total_score);
    println!("\nTotal score: {}", total_score_nv);
}
