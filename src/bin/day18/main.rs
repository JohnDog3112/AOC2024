use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};

#[derive(Clone, Debug)]
struct PathNode {
    start: (i32, i32),
    end: (i32, i32),
    predicted_distance: i32,
    cost: i32,
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.predicted_distance == other.predicted_distance
    }
}
impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.predicted_distance.partial_cmp(&other.predicted_distance)
    }
}
impl Eq for PathNode {}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.predicted_distance.cmp(&other.predicted_distance)
    }
}

fn find_path(filled: &HashSet<(i32, i32)>, start: (i32, i32), end: (i32, i32), break_early: bool) -> Option<i32> {
    let mut paths = HashMap::new();
    let mut heap = BinaryHeap::new();

    heap.push(Reverse(PathNode {
        start: start,
        end: start,
        predicted_distance: (end.0 - start.0).pow(2) + (end.1 - start.1).pow(2),
        cost: 0,
    }));

    while let Some(Reverse(node)) = heap.pop() {
        if let Some((_, cost)) = paths.get(&node.end) {
            if *cost <= node.cost {
                continue;
            }
        }
        paths.insert(node.end, (node.start, node.cost));
        if node.end == end && break_early {
            break;
        }

        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let check = (node.end.0 + dir.0, node.end.1 + dir.1);
            
            if check.0 < 0 || check.0 > end.0 || check.1 < 0 || check.1 > end.1 {
                continue;
            }
            if filled.contains(&check) {
                continue;
            }

            if let Some((_, cost)) = paths.get(&check) {
                if *cost <= node.cost+1 {
                    continue;
                }
            }

            heap.push(Reverse(PathNode {
                start: node.end,
                end: check,
                predicted_distance: (check.0 - end.0).pow(2) + (check.1 - end.1).pow(2),
                cost: node.cost + 1
            }));
        }
    }

    paths.get(&end).map(|(_, cost)| *cost)
}

const NUM_TAKE: usize = 1024;
const START: (i32, i32) = (0, 0);
const END: (i32, i32) = (70, 70);

fn part1(bytes: &Vec<(i32, i32)>) -> usize {
    
    let mut filled = HashSet::new();

    for i in 0..NUM_TAKE {
        filled.insert(bytes[i]);
    }

    find_path(&filled, START, END, false).unwrap() as usize
}
fn part2(bytes: &Vec<(i32, i32)>) -> (i32, i32) {
    let mut filled = HashSet::new();
    for i in 0..bytes.len() {
        filled.insert(bytes[i]);
        if let None = find_path(&filled, START, END, true) {
            return bytes[i];
        }
    }
    unreachable!()
}
fn main() {
    let bytes = parse_input();
    println!("part1: {}", part1(&bytes));
    println!("part2: {:?}", part2(&bytes));
}

fn parse_input() -> Vec<(i32, i32)> {
    let inp = include_str!("input.txt");
    inp.split("\n").map(|line| {
        let parts = line.split(",").map(|num| num.parse().unwrap()).collect::<Vec<i32>>();
        (parts[0], parts[1])
    }).collect()
}