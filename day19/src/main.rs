use std::collections::HashMap;
use std::fs::read_to_string;

fn can_assemble_string(tokens: &[String], target: &str) -> u64 {
    if target.is_empty() {
        return 1;
    }

    let min_token_len = tokens.iter().map(|t| t.len()).min().unwrap();

    if target.len() < min_token_len {
        return 0;
    }

    let first_char = target.chars().next().unwrap();
    if !tokens.iter().any(|t| t.starts_with(first_char)) {
        return 0;
    }

    let mut memo: HashMap<usize, u64> = HashMap::new();

    fn build_string(
        tokens: &[String],
        target: &str,
        current_pos: usize,

        min_len: usize,

        memo: &mut HashMap<usize, u64>,
    ) -> u64 {
        if let Some(&cached_ways) = memo.get(&current_pos) {
            return cached_ways;
        }

        if current_pos == target.len() {
            return 1;
        }

        if current_pos > target.len() {
            return 0;
        }

        let remaining_len = target.len() - current_pos;
        if remaining_len < min_len {
            return 0;
        }

        let next_char = target.chars().nth(current_pos).unwrap();
        let mut total_ways = 0;

        for token in tokens.iter().filter(|t| {
            t.starts_with(next_char)
                && t.len() <= remaining_len
                && (t.len() == remaining_len || remaining_len - t.len() >= min_len)
        }) {
            if target[current_pos..].starts_with(token) {
                total_ways +=
                    build_string(tokens, target, current_pos + token.len(), min_len, memo);
            }
        }

        memo.insert(current_pos, total_ways);
        total_ways
    }

    build_string(tokens, target, 0, min_token_len, &mut memo)
}

fn main() -> std::io::Result<()> {
    let contents = read_to_string("data2")?;
    let lines: Vec<&str> = contents.lines().collect();

    let tokens: Vec<String> = lines[0].split(", ").map(String::from).collect();

    let blank_line_idx = lines.iter().position(|&line| line.is_empty()).unwrap();
    let strings_to_check = &lines[blank_line_idx + 1..];

    let total_ways: u64 = strings_to_check
        .iter()
        .map(|s| {
            let ways = can_assemble_string(&tokens, s);
            println!("Processed string: {} ({} ways)", s, ways);
            ways
        })
        .sum();

    println!("Total number of ways across all strings: {}", total_ways);

    Ok(())
}
