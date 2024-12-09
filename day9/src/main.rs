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

fn compactify2(memory_map: &mut Vec<i32>) {
    let mut back = memory_map.len() - 1;

    while back > 0 {
        // Skip -1s at the end
        while back > 0 && memory_map[back] == -1 {
            back -= 1;
        }

        let value = memory_map[back];
        let mut block_len = 0;
        let mut pos = back;

        // Get block size
        while pos >= 0 && memory_map[pos] == value {
            block_len += 1;
            if pos == 0 {
                break;
            }

            pos -= 1;
        }

        // Look for first gap of sufficient size
        let mut found_gap = false;
        let mut front = 0;
        while front < back {
            if memory_map[front] == -1 {
                // Count consecutive -1s
                let mut gap_len = 0;
                let mut i = front;
                while i < memory_map.len() && memory_map[i] == -1 {
                    gap_len += 1;
                    i += 1;
                }

                // gap is big enough
                if gap_len >= block_len {
                    // Copy to the front
                    for i in 0..block_len {
                        memory_map[front + i] = memory_map[back - i];
                        memory_map[back - i] = -1;
                    }
                    found_gap = true;
                    break;
                }
            }
            front += 1;
        }

        // Move back to before the current block
        if found_gap {
            back = back - block_len;
        } else {
            // If no gap found
            back = pos;
        }
    }
}

fn main() -> std::io::Result<()> {
    //match get_data("https://adventofcode.com/2024/day/9/input") {
    // Read the grid once
    let content = fs::read_to_string("data")?;
    //let content = "2333133121414131402";

    let (_, memory_map) = parse_memory_layout(&content);
    let mut one_memory_map = memory_map.clone();
    let mut two_memory_map = memory_map.clone();

    println!("Downloaded: {} bytes.", content.len());
    //println!("Blocks: {:?}", blocks);
    //println!("Memory map: {:?}", memory_map)
    //
    for entry in &memory_map {
        println!("{}", entry);
    }
    println!("Started with {}", one_memory_map.len());
    compactify(&mut one_memory_map);
    println!("Ended with {}", one_memory_map.len());

    println!("Started with {}", two_memory_map.len());
    compactify2(&mut two_memory_map);
    println!("Ended with {}", two_memory_map.len());
    let mut sum: u128 = 0;
    for (pos, &value) in one_memory_map.iter().enumerate() {
        if value == -1 {
            panic!("Found -1 at position {}", pos);
        }
        sum += (pos as u128) * (value as u128);
    }
    println!("Part 1 Sum: {}", sum);

    sum = 0;
    for (pos, &value) in two_memory_map.iter().enumerate() {
        if value == -1 {
            continue;
            //panic!("Found -1 at position {}", pos);
        }
        sum += (pos as u128) * (value as u128);
    }
    println!("Part 2 Sum: {}", sum);

    /*    let original_formatted: String = memory_map
            .iter()
            .map(|&x| {
                if x == -1 {
                    '.'
                } else {
                    char::from_digit(x as u32, 10).unwrap_or('.')
                }
            })
            .collect();
        println!("Original map: {}", original_formatted);

        let formatted: String = two_memory_map
            .iter()
            .map(|&x| {
                if x == -1 {
                    '.'
                } else {
                    char::from_digit(x as u32, 10).unwrap_or('.')
                }
            })
            .collect();
        println!("Memory map:   {}", formatted);
    */
    Ok(())
}
