use std::fs;

#[derive(Debug)]
struct Block {
    id: u32,
    starting_address: u32,
    length: u32,
}

fn parse_memory_layout(input: &str) -> (Vec<Block>, Vec<i32>) {
    let mut blocks = Vec::new();
    let mut memory_map = Vec::new();
    let mut current_position = 0;
    let mut current_id = 0;
    let mut is_block = true; // alternating flag

    let numbers: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();

    for &length in numbers.iter() {
        if is_block {
            // Create new block
            let block = Block {
                id: current_id,
                starting_address: current_position,
                length,
            };
            blocks.push(block);
            // Fill memory map
            for _ in 0..length {
                memory_map.push(current_id as i32);
            }
            current_id += 1;
        } else {
            // Fill free space
            for _ in 0..length {
                memory_map.push(-1);
            }
        }
        current_position += length;
        is_block = !is_block; // toggle flag
    }
    (blocks, memory_map)
}

fn get_data(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get(url)?;
    response.text()
}

fn compactify(memory_map: &mut Vec<i32>) {
    let mut front = 0;
    let mut back = memory_map.len() - 1;

    while front < back {
        while memory_map[back] == -1 {
            back -= 1;
            memory_map.pop();
        }
        if memory_map[front] == -1 {
            memory_map[front] = memory_map[back];
            back -= 1;
            memory_map.pop();
        }
        front += 1;
    }
}

fn main() -> std::io::Result<()> {
    //match get_data("https://adventofcode.com/2024/day/9/input") {
    // Read the grid once
    let content = fs::read_to_string("data")?;

    let (_, mut memory_map) = parse_memory_layout(&content);
    println!("Downloaded: {} bytes.", content.len());
    //println!("Blocks: {:?}", blocks);
    //println!("Memory map: {:?}", memory_map)
    for entry in &memory_map {
        println!("{}", entry);
    }
    println!("Started with {}", memory_map.len());
    compactify(&mut memory_map);
    println!("Ended with {}", memory_map.len());

    let mut sum: u128 = 0;
    for (pos, &value) in memory_map.iter().enumerate() {
        if value == -1 {
            panic!("Found -1 at position {}", pos);
        }
        sum += (pos as u128) * (value as u128);
    }
    println!("Sum: {}", sum);

    Ok(())
}
