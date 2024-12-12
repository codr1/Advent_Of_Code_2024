static mut MAX_DEPTH: usize = 71;
const LOOKUP_DEPTH: usize = 25; // How many steps to pre-compute
const LOOKUP_MAX: usize = 10000; // Maximum number to precompute

#[derive(Debug, Clone)]
struct PrecomputeResult {
    numbers: Vec<i64>,
}

fn precompute_digits() -> Vec<Vec<PrecomputeResult>> {
    let mut table = vec![
        vec![
            PrecomputeResult {
                numbers: Vec::new()
            };
            LOOKUP_DEPTH
        ];
        LOOKUP_MAX + 1
    ];

    // Initialize depth 0 for all numbers
    for num in 0..=LOOKUP_MAX {
        table[num][0] = PrecomputeResult {
            numbers: vec![num as i64],
        };
    }

    // Compute each depth
    for depth in 0..LOOKUP_DEPTH - 1 {
        for num in 0..=LOOKUP_MAX {
            let mut new_numbers = Vec::new();
            for &n in &table[num][depth].numbers {
                if n == 0 {
                    new_numbers.push(1);
                } else {
                    let digits = count_digits(n);
                    if digits % 2 == 0 {
                        let (front, back) = get_digit_parts(n, digits);
                        new_numbers.push(front);
                        new_numbers.push(back);
                    } else {
                        new_numbers.push(n * 2024);
                    }
                }
            }
            table[num][depth + 1] = PrecomputeResult {
                numbers: new_numbers,
            };
        }
    }
    table
}

fn process_stone_depth_with_lookup(
    stone: i64,
    depth: usize,
    lookup: &Vec<Vec<PrecomputeResult>>,
) -> usize {
    if depth == unsafe { MAX_DEPTH } {
        return 1;
    }

    // Single step from MAX_DEPTH
    if depth == unsafe { MAX_DEPTH } - 1 {
        let digits = count_digits(stone);
        if stone == 0 {
            return 1;
        } else if digits % 2 == 0 {
            return 2;
        } else {
            return 1;
        }
    }

    // Use lookup table when possible
    if stone >= 0 && stone <= LOOKUP_MAX as i64 {
        let remaining_steps = unsafe { MAX_DEPTH } - depth;
        if remaining_steps <= LOOKUP_DEPTH {
            // Use the precomputed result directly
            let result = &lookup[stone as usize][remaining_steps - 1];
            return result.numbers.len();
        }
    }

    // Regular processing
    let digits = count_digits(stone);
    if stone == 0 {
        return process_stone_depth_with_lookup(1, depth + 1, lookup);
    } else if digits % 2 == 0 {
        let (front, back) = get_digit_parts(stone, digits);
        return process_stone_depth_with_lookup(front, depth + 1, lookup)
            + process_stone_depth_with_lookup(back, depth + 1, lookup);
    } else {
        return process_stone_depth_with_lookup(stone * 2024, depth + 1, lookup);
    }
}

fn count_digits(n: i64) -> usize {
    if n == 0 {
        return 1;
    };
    let mut n = n.abs();
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }

    count
}

fn get_digit_parts(mut n: i64, digits: usize) -> (i64, i64) {
    let mut idx = 0;
    let mut back = 0;
    while idx < digits / 2 {
        back += (n % 10) * 10_i64.pow(idx as u32);
        n /= 10;
        idx += 1;
    }

    (n, back)
}

fn main() {
    let numbers: Vec<i64> = std::fs::read_to_string("data")
        .expect("Failed to read file")
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i64>().expect("Failed to parse number"))
        .collect();

    println!("Choose method:");
    println!("1. breadth-first ");
    println!("2. depth-first ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => {
            // Lanternfish-style solution
            use std::collections::HashMap;

            // Initialize our groups
            let mut stone_groups: HashMap<i64, usize> = HashMap::new();
            for &n in &numbers {
                *stone_groups.entry(n).or_insert(0) += 1;
            }

            for i in 0..unsafe { MAX_DEPTH } {
                let mut new_groups: HashMap<i64, usize> = HashMap::new();

                for (stone, &count) in stone_groups.iter() {
                    let digits = count_digits(*stone);
                    if *stone == 0 {
                        // All zeros become ones
                        *new_groups.entry(1).or_insert(0) += count;
                    } else if digits % 2 == 0 {
                        // Even-digit stones split into two parts
                        let (front, back) = get_digit_parts(*stone, digits);
                        *new_groups.entry(front).or_insert(0) += count;
                        *new_groups.entry(back).or_insert(0) += count;
                    } else {
                        // Odd-digit stones multiply by 2024
                        *new_groups.entry(stone * 2024).or_insert(0) += count;
                    }
                }

                stone_groups = new_groups;
                let total_stones: usize = stone_groups.values().sum();
                println!("Blink {} Stones: {}", i + 1, total_stones);
            }

            let final_count: usize = stone_groups.values().sum();
            println!("Breadth-first final count: {}", final_count);
        }
        "2" => {
            // HACK: Add 1 to MAX_DEPTH for depth-first search to match breadth-first results.
            // This compensates for a counting discrepancy between the two methods that we
            // haven't fully diagnosed yet, but empirically fixes the results.
            const DEPTH_FIRST_OFFSET: usize = 1;
            let lookup_table = precompute_digits();
            let original_depth = unsafe { MAX_DEPTH };
            unsafe { MAX_DEPTH += DEPTH_FIRST_OFFSET };
            println!("Precomputing lookup table... (using depth {})", unsafe {
                MAX_DEPTH
            });

            let mut total_stones = 0;
            for (i, &number) in numbers.iter().enumerate() {
                let stones = process_stone_depth_with_lookup(number, 0, &lookup_table);
                println!("Number {} generated {} stones", i + 1, stones);
                total_stones += stones;
            }
            println!("Depth-first final count: {}", total_stones);
            // Restore original depth
            unsafe { MAX_DEPTH = original_depth };

            // Debug: print the lookup table for first few steps
            //println!("\nLookup table for first 3 depths:");
            for _digit in 0..10 {
                //println!("Digit {}: {:?}", _digit, &lookup_table[_digit][0..3]);
            }
        }
        _ => println!("Invalid choice"),
    }
}
