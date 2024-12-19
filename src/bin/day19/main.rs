use std::collections::{HashMap, HashSet};


fn is_possible_helper(patterns: &Vec<String>, design: &str, index: usize, failed: &mut HashSet<usize>) -> bool {
    if failed.contains(&index) {
        return false;
    }
    if index == design.len() {
        return true;
    }

    for pattern in patterns {
        if design[index..].starts_with(pattern) {
            if is_possible_helper(patterns, design, index+pattern.len(), failed) {
                return true;
            }
        }
    }
    failed.insert(index);
    false
}

fn is_possible(patterns: &Vec<String>, design: &str) -> bool {
    let mut failed = HashSet::new();
    
    is_possible_helper(patterns, design, 0, &mut failed)
}
fn part1(patterns: &Vec<String>, designs: &Vec<String>) -> usize {
    designs.into_iter().filter(|design| {
        is_possible(patterns, design)
    }).count()
}

fn count_possible_helper(patterns: &Vec<String>, design: &str, index: usize, possible: &mut HashMap<usize, usize>) -> usize {
    if let Some(possible) = possible.get(&index) {
        return *possible;
    }
    if index == design.len() {
        return 1;
    }

    let mut total_count = 0;
    for pattern in patterns {
        if design[index..].starts_with(pattern) {
            total_count += count_possible_helper(patterns, design, index+pattern.len(), possible);
        }
    }
    possible.insert(index, total_count);
    total_count
}

fn count_possible(patterns: &Vec<String>, design: &str) -> usize {
    let mut possible = HashMap::new();
    
    count_possible_helper(patterns, design, 0, &mut possible)
}
fn part2(patterns: &Vec<String>, designs: &Vec<String>) -> usize {
    designs.into_iter().map(|design| {
        count_possible(patterns, design)
    }).sum()
}
fn main() {
    let (patterns, designs) = parse_input();

    println!("part1: {}", part1(&patterns, &designs));
    println!("part2: {}", part2(&patterns, &designs));
}

fn parse_input() -> (Vec<String>, Vec<String>) {
    let inp = include_str!("input.txt");
    
    let parts: Vec<&str> = inp.split("\n\n").collect();

    let patterns: Vec<String> = parts[0].split(", ").map(|st| st.to_string()).collect();

    let designs: Vec<String> = parts[1].split("\n").map(|st| st.to_string()).collect();

    (patterns, designs)
}