use std::collections::{HashMap, HashSet};


const DIRECTIONS: [(i32, i32); 4] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1)
];
//returns number of 9's found
fn breadth_helper(map: &Vec<Vec<u8>>, node: (usize, usize), visited: &mut HashSet<(usize, usize)>) -> usize {
    if visited.contains(&node) {
        return 0;
    }
    visited.insert(node);

    let val = map[node.1][node.0];
    if val == 9 {
        return 1;
    }
    let mut sum = 0;
    for dir in DIRECTIONS {
        if
            node.0 as i32+ dir.0 < 0 || node.0 as i32 + dir.0 >= map[0].len() as i32
            || node.1 as i32 + dir.1 < 0 || node.1 as i32 + dir.1 >= map.len() as i32
        {
            continue;
        }
        let new_node = (
            (node.0 as i32 + dir.0) as usize,
            (node.1 as i32 + dir.1) as usize
        );
        let new_val = map[new_node.1][new_node.0];
        if new_val == val+1 {
            sum += breadth_helper(map, new_node, visited);
        }
    }
    return sum;
}
//returns number of 9's found
fn breadth_first_search(map: &Vec<Vec<u8>>, node: (usize, usize)) -> usize {
    return breadth_helper(map, node, &mut HashSet::new());
}

fn part1(inp: Vec<Vec<u8>>) -> usize {
    inp.iter().enumerate().map(|(y, line)| {
        line.iter().enumerate().map(|(x, pnt)| {
            if *pnt == 0 {
                breadth_first_search(&inp, (x, y))
            } else {
                0
            }
        }).sum::<usize>()
    }).sum()
}


fn second_helper(map: &Vec<Vec<u8>>, node: (usize, usize), paths: &mut HashMap<(usize, usize), usize>) -> usize {
    let val = map[node.1][node.0];
    if val == 9 {
        return 1;
    }
    if let Some(amt) = paths.get(&node) {
        return *amt;
    }

    let mut sum = 0;
    for dir in DIRECTIONS {
        if
            node.0 as i32+ dir.0 < 0 || node.0 as i32 + dir.0 >= map[0].len() as i32
            || node.1 as i32 + dir.1 < 0 || node.1 as i32 + dir.1 >= map.len() as i32
        {
            continue;
        }
        let new_node = (
            (node.0 as i32 + dir.0) as usize,
            (node.1 as i32 + dir.1) as usize
        );
        let new_val = map[new_node.1][new_node.0];
        if new_val == val+1 {
            sum += second_helper(map, new_node, paths);
        }
    }
    paths.insert(node, sum);
    return sum;
}
fn second_breadth_search(map: &Vec<Vec<u8>>, node: (usize, usize)) -> usize {
    second_helper(map, node, &mut HashMap::new())
}

fn part2(inp: Vec<Vec<u8>>) -> usize {
    inp.iter().enumerate().map(|(y, line)| {
        line.iter().enumerate().map(|(x, pnt)| {
            if *pnt == 0 {
                second_breadth_search(&inp, (x, y))
            } else {
                0
            }
        }).sum::<usize>()
    }).sum()
}
fn main() {
    let inp = get_input();
    println!("part1: {}", part1(inp.clone()));
    println!("part2: {}", part2(inp));

}

pub fn get_input() -> Vec<Vec<u8>> {
    let inp = include_str!("input.txt");

    inp.split('\n').map(|line| {
        line.chars().map(|char| {
            match char.to_digit(10) {
                Some(dig) => dig as u8,
                None => 20u8 //just some large number to ignore with
            }
        }).collect::<Vec<u8>>()
    }).collect::<Vec<Vec<u8>>>()
}