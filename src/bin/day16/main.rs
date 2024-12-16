use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST
}
impl Direction {
    fn to_vec(self) -> (i32, i32) {
        match self {
            Self::NORTH => (0, -1),
            Self::EAST => (1, 0),
            Self::SOUTH => (0, 1),
            Self::WEST => (-1, 0)
        }
    }
    fn turn_left(self) -> Self {
        match self {
            Self::NORTH => Self::WEST,
            Self::WEST => Self::SOUTH,
            Self::SOUTH => Self::EAST,
            Self::EAST => Self::NORTH,
        }
    }
    fn turn_right(self) -> Self {
        match self {
            Self::NORTH => Self::EAST,
            Self::EAST => Self::SOUTH,
            Self::SOUTH => Self::WEST,
            Self::WEST => Self::NORTH
        }
    }

    fn get_costs(self, cost: i32) -> [(Self, i32); 3] {
        [
            (self, cost+1),
            (self.turn_left(), cost+1001),
            (self.turn_right(), cost+1001),
        ]
    }
}
#[derive(Clone, Debug)]
struct PathNode {
    start: (i32, i32),
    end: (i32, i32),
    direction: Direction,
    total_cost: i32,
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.total_cost == other.total_cost
    }
}
impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.total_cost.partial_cmp(&other.total_cost)
    }
}
impl Eq for PathNode {}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_cost.cmp(&other.total_cost)
    }
}


fn run_dijkstras(start: (i32, i32), map: &Vec<Vec<MapObject>>) -> HashMap<(Direction, (i32, i32)), ((i32, i32), i32)> {
    let mut heap = BinaryHeap::new();
    let mut visited: HashMap<(Direction, (i32, i32)), ((i32, i32), i32)> = HashMap::new();

    for (test_dir, cost) in Direction::EAST.get_costs(0) {
        let dir_v = test_dir.to_vec();
        let check_pos = (start.0 + dir_v.0, start.1 + dir_v.1);
        visited.insert((test_dir, start), (start, 100000000));

        if map[check_pos.1 as usize][check_pos.0 as usize] == MapObject::Empty {
            heap.push(Reverse(
                PathNode {
                    start: start,
                    end: check_pos,
                    direction: test_dir,
                    total_cost: cost
                }
            ))
        }
    }

    while let Some(Reverse(path)) = heap.pop() {
        if visited.contains_key(&(path.direction, path.end)) {
            continue;
        }
        // println!("{}", path.total_cost);
        visited.insert((path.direction, path.end), (path.start, path.total_cost));

        for (test_dir, new_cost) in path.direction.get_costs(path.total_cost) {
            let dir_v = test_dir.to_vec();
            let check_pos = (path.end.0 + dir_v.0, path.end.1 + dir_v.1);
            
            if map[check_pos.1 as usize][check_pos.0 as usize] == MapObject::Empty {
                heap.push(Reverse(
                    PathNode {
                        start: path.end,
                        end: check_pos,
                        direction: test_dir,
                        total_cost: new_cost
                    }
                ))
            }
        }
    }

    visited
}
fn part1(start: (i32, i32), map: &Vec<Vec<MapObject>>, end: (i32, i32)) -> i32 {
    let path = run_dijkstras(start, map);

    let mut min_val = 1000000000;
    for (dir, _) in Direction::EAST.get_costs(0) {
        if let Some(&(_, cost)) = path.get(&(dir, end)) {
            if cost < min_val {
                min_val = cost;
            }
        }
    }

    min_val
}

fn find_path_nodes(pos: (i32, i32), dir: Direction, paths: &HashMap<(Direction, (i32, i32)), ((i32, i32), i32)>, visited: &mut HashSet<(Direction, (i32, i32))>) {
    if visited.contains(&(dir, pos)) {
        return;
    }
    visited.insert((dir, pos));
    // println!("{dir:?}, {pos:?}");

    let &(_, cost) = paths.get(&(dir, pos)).unwrap();

    for (dir, _) in dir.get_costs(0) {
        let dir_vec = dir.to_vec();

        let check_pos = (pos.0 - dir_vec.0, pos.1 - dir_vec.1);

        for (n_dir, offset) in dir.get_costs(0) {
            let target_cost = cost - offset;
            if let Some(&(_, cost)) = paths.get(&(n_dir, check_pos)) {
                if cost == target_cost {
                    find_path_nodes(check_pos, n_dir, paths, visited);
                }
            }
        } 
    }
}
fn part2(start: (i32, i32), map: &Vec<Vec<MapObject>>, end: (i32, i32)) -> i32 {
    let path = run_dijkstras(start, map);

    let mut end_cost = 1000000;
    for (dir, _) in Direction::EAST.get_costs(0) {
        if let Some(&(_, cost)) = path.get(&(dir, end)) {
            if cost < end_cost {
                end_cost = cost;
            }
        }
    }

    let mut visited = HashSet::new();

    for (dir, _) in Direction::EAST.get_costs(0) {
        if let Some(&(_, cost)) = path.get(&(dir, end)) {
            if end_cost == cost {
                find_path_nodes(end, dir, &path, &mut visited);
            }
        }
    }
    

    let mut path_nodes = HashSet::new();
    for (_, node) in visited {
        path_nodes.insert(node);
    }

    path_nodes.len() as i32 + 1
}
fn main() {
    let (start, map, end) = parse_input();

    println!("part1: {}", part1(start, &map, end));
    println!("part2: {}", part2(start, &map, end));
}


#[derive(Clone, Copy, PartialEq, Eq)]
enum MapObject {
    Empty,
    Wall
}

fn parse_input() -> ((i32, i32), Vec<Vec<MapObject>>, (i32, i32)) {
    let inp = include_str!("input.txt");

    let mut start = (0, 0);
    let mut end = (0, 0);
    let map: Vec<Vec<MapObject>> = inp.split("\n").enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, ch)| {
            match ch {
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

    (start, map, end)
}