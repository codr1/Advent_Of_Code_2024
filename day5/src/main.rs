use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_to_array(filename: &str) -> (Vec<String>, Vec<String>) {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    
    let mut rules = Vec::new();
    let mut pages = Vec::new();
    let mut is_rules = true;  // Flag to track which section we're in
    
    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                if line_content.trim().is_empty() {
                    is_rules = false;  // Switch to pages section after empty line
                    continue;  // Skip the empty line itself
                }
                
                if is_rules {
                    rules.push(line_content);
                } else {
                    pages.push(line_content);
                }
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
    
    (rules, pages)
}

fn parse_rules(rules_lines: Vec<String>) -> Vec<Vec<i32>> {
    let mut rules = Vec::new();
    
    for line in rules_lines {
        let numbers: Vec<i32> = line
            .split('|')
            .map(|s| s.trim().parse().expect("Failed to parse number"))
            .collect();
    rules.push(numbers);
    }
    
    rules
}

fn parse_pages(pages_lines: Vec<String>) -> Vec<Vec<i32>> {
    let mut pages = Vec::new();
    
    for line in pages_lines {
        let numbers: Vec<i32> = line
            .split(',')
            .map(|s| s.trim().parse().expect("Failed to parse number"))
            .collect();
        pages.push(numbers);
    }
    
    pages
}

fn check_page_numbers(numbers: &Vec<i32>, rules: &Vec<Vec<i32>>) -> (bool, usize, i32, i32) {
    // Iterate through each number in the list
    for (i, &curr_page) in numbers.iter().enumerate() {
        // Look at all subsequent numbers
        for &checked_page in numbers.iter().skip(i + 1) {
            // Check against each rule
            for (rule_idx, rule) in rules.iter().enumerate() {
                if rule[0] == checked_page && rule[1] == curr_page {
                    return (false, rule_idx + 1, curr_page, checked_page);
                }
            }
        }
    }
    
    (true, 0, 0, 0)
}

fn main() {
    println!("Hello, world!");

    let filename = "data";
    let (rules_lines, pages_lines) = read_to_array(filename);
    let rules = parse_rules(rules_lines);
    let pages = parse_pages(pages_lines);
    let mut sum = 0;

    for (idx, page_list) in pages.iter().enumerate() {
        let(pass, rule_index, curr_page, checked_page) = check_page_numbers(&page_list, &rules);
        if !pass {
            println!("failed line {}, failed rule {} - {}, {}", idx + 1, rule_index + 1, curr_page, checked_page );
        } else {
            let middle_index = page_list.len() / 2;
            println!("mid {}", page_list[middle_index]);
            sum += page_list[middle_index];
        }
            
    }
    println!("Sum {}", sum);

}
