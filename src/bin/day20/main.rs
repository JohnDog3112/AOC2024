use std::collections::{HashMap, HashSet, VecDeque};

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn get_costs(map: &Vec<Vec<MapObject>>, start: (i32, i32), end: (i32, i32)) -> (HashSet<(i32, i32)>, HashMap<(i32, i32), i32>, HashMap<(i32, i32), i32>) {
    let mut walls = HashSet::new();
    let mut from_start = HashMap::new();
    let mut from_end = HashMap::new();
    from_start.insert(start, 0);
    from_end.insert(end, 0);

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    
    while let Some((node, cost)) = queue.pop_front() {
        for dir in DIRECTIONS {
            let next = (node.0 + dir.0, node.1 + dir.1);
            if next.0 < 0 || next.0 >= map[0].len() as i32 || next.1 < 0 || next.1 >= map.len() as i32 {
                continue;
            }
            if MapObject::Wall == map[next.1 as usize][next.0 as usize] {
                walls.insert(next);
            } else if let None = from_start.get(&next) {
                from_start.insert(next, cost+1);
                queue.push_back((next, cost+1));
            }
        }
    }
    queue.push_back((end, 0));
    while let Some((node, cost)) = queue.pop_front() {
        for dir in DIRECTIONS {
            let next = (node.0 + dir.0, node.1 + dir.1);
            if next.0 < 0 || next.0 >= map[0].len() as i32 || next.1 < 0 || next.1 >= map.len() as i32 {
                continue;
            }
            if MapObject::Wall == map[next.1 as usize][next.0 as usize] {
                // walls.insert(next);
            } else if let None = from_end.get(&next) {
                from_end.insert(next, cost+1);
                queue.push_back((next, cost+1));
            }
        }
    }

    (walls, from_start, from_end)
}
fn part1(map: &Vec<Vec<MapObject>>, start: (i32, i32), end: (i32, i32)) -> i32 {
    
    let (walls, from_start, from_end) = get_costs(map, start, end);

    let mut shortcuts = HashMap::new();
    let base_cost = *from_start.get(&end).unwrap();

    // println!("{base_cost:?}");

    for wall in walls {
        for dir in DIRECTIONS {
            let start_pos = (wall.0 + dir.0, wall.1 + dir.1);
            if let Some(start) = from_start.get(&start_pos) {
                // add_shortcuts(start_pos, wall, false, *start, &from_end, base_cost, &mut shortcuts);
                for dir in DIRECTIONS {
                    let end_pos = (wall.0 + dir.0, wall.1 + dir.1);
                    if end_pos.0 == start_pos.0 && end_pos.1 == start_pos.1 {
                        continue;
                    }
                    if let Some(end) = from_end.get(&end_pos) {
                        let savings = base_cost - (start + end + 2);
                        if savings <= 0 {
                            continue;
                        }
                        if let Some(val) = shortcuts.get_mut(&(start_pos, end_pos)) {
                            if savings < *val {
                                *val = savings;
                            } 
                        } else {
                            shortcuts.insert((start_pos, end_pos), savings);
                        }
                    }
                }
            }
        }
    }
    // println!("{:?}", shortcuts);
    let mut shortcut_count = HashMap::new();
    for (_, savings) in shortcuts {
        if let Some(val) = shortcut_count.get_mut(&savings) {
            *val += 1;
        } else {
            shortcut_count.insert(savings, 1);
        }
    }
    let shortcuts = shortcut_count.into_iter().collect::<Vec<(i32, i32)>>();
    // shortcuts.sort_by(|a, b| a.0.cmp(&b.0));
    // println!("{:?}", shortcuts);
    shortcuts.into_iter().filter_map(|(savings, count)| {
        if savings >= 100 {
            Some(count)
        } else {
            None
        }
    }).sum()
}

fn part2(map: &Vec<Vec<MapObject>>, start: (i32, i32), end: (i32, i32)) -> i32 {
    let (_, from_start, from_end) = get_costs(map, start, end);
    let base_cost = *from_start.get(&end).unwrap();

    let mut shortcuts = HashMap::new();

    for (start_pos, start_cost) in from_start {
        let mut visited = HashSet::new();
        visited.insert(start_pos);
        let mut queue: VecDeque<((i32, _), i32)> = VecDeque::new();
        queue.push_back((start_pos, 0));
        while let Some((node, steps)) = queue.pop_front() {
            for dir in DIRECTIONS {
                let next = (node.0 + dir.0, node.1 + dir.1);
                if visited.contains(&next) {
                    continue;
                }
                visited.insert(next);
                if let Some(end) = from_end.get(&next) {
                    let savings = base_cost - (start_cost + *end + steps + 1);
                    if savings > 0 {
                        shortcuts.insert((start_pos, next), savings);
                    }
                }
                if steps < 19 {
                    queue.push_back((next, steps+1));
                }
            }
        }
    }

    let mut shortcut_counts = HashMap::new();
    for (_, savings) in shortcuts {
        if let Some(val) = shortcut_counts.get_mut(&savings) {
            *val += 1;
        } else {
            shortcut_counts.insert(savings, 1);
        }
    }
    let shortcut_counts = shortcut_counts.into_iter().collect::<Vec<(i32, i32)>>();
    shortcut_counts.into_iter().filter_map(|(savings, count)| {
        if savings >= 100 {
            Some(count)
        } else {
            None
        }
    }).sum()
}
fn main() {
    let (map, start, end) = parse_input();
    println!("part1: {}", part1(&map, start, end));
    println!("part2: {}", part2(&map, start, end));
}


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum MapObject {
    Empty,
    Wall
}

fn parse_input() -> (Vec<Vec<MapObject>>, (i32, i32), (i32, i32)) {
    let inp = include_str!("input.txt");

    let mut start = (-1, -1);
    let mut end = (-1, -1);
    let map = inp.split("\n").enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, obj)| {
            match obj {
                '#' => MapObject::Wall,
                '.' => MapObject::Empty,
                'S' => {
                    start = (x as i32, y as i32);
                    MapObject::Empty
                },
                'E' => {
                    end = (x as i32, y as i32);
                    MapObject::Empty
                },
                _ => unreachable!()
            }
        }).collect()
    }).collect();

    assert!(start.0 != -1 && start.1 != -1);
    assert!(end.0 != -1 && end.1 != -1);

    (map, start, end)
}