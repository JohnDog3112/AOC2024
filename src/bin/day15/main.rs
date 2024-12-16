use std::collections::{HashMap, HashSet};


#[allow(unused)]
fn print_map(map: &Vec<Vec<MapObject>>, robot: (i32, i32)) {
    let mut map = map.iter().map(|line| {
        line.iter().map(|pos| {
            match pos {
                MapObject::Empty => '.',
                MapObject::Box => 'O',
                MapObject::Wall => '#'
            }
        }).collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    map[robot.1 as usize][robot.0 as usize] = '@';

    println!(
        "{}",
        map.into_iter()
            .map(|line| String::from_iter(line))
            .collect::<Vec<String>>()
            .join("\n")
    );
}
fn part1(robot_pos: (i32, i32), map: Vec<Vec<MapObject>>, directions: Vec<Direction>) -> usize {
    let mut robot = robot_pos;
    let mut map = map;

    for dir in directions {
        // print_map(&map, robot);
        let dir = dir.to_dir();
        let new_pos = (robot.0 + dir.0, robot.1 + dir.1);
        let mut check_pos = new_pos;
        while map[check_pos.1 as usize][check_pos.0 as usize] == MapObject::Box {
            check_pos = (check_pos.0 + dir.0, check_pos.1 + dir.1);
        }

        if map[check_pos.1 as usize][check_pos.0 as usize] == MapObject::Wall {
            continue;
        }
        let tmp = map[new_pos.1 as usize][new_pos.0 as usize];
        map[check_pos.1 as usize][check_pos.0 as usize] = tmp;
        map[new_pos.1 as usize][new_pos.0 as usize] = MapObject::Empty;

        robot = new_pos;
    }
    // print_map(&map, robot);

    map.into_iter().enumerate().fold(0, |acc, (y, line)| {
        line.into_iter().enumerate().fold(0, |acc, (x, obj)| {
            acc + match obj {
                MapObject::Box => 100 * y + x,
                _ => 0
            }
        }) + acc
    })
}

#[allow(unused)]
fn print_map2(map: &Vec<Vec<MapObject2>>, robot: (i32, i32)) {
    let mut map = map.iter().map(|line| {
        line.iter().map(|pos| {
            match pos {
                MapObject2::Empty => '.',
                MapObject2::LBox => '[',
                MapObject2::RBox => ']',
                MapObject2::Wall => '#'
            }
        }).collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    map[robot.1 as usize][robot.0 as usize] = '@';

    println!(
        "{}",
        map.into_iter()
            .map(|line| String::from_iter(line))
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn find_empty(check_pos: (i32, i32), dir: (i32, i32), map: &Vec<Vec<MapObject2>>) -> Option<(i32, i32)> {
    let mut x: usize = check_pos.0 as usize;
    let mut y: usize = check_pos.1 as usize;
    while map[y][x] == MapObject2::LBox || map[y][x] == MapObject2::RBox  {
        x = (x as i32 + dir.0) as usize;
        y = (y as i32 + dir.1) as usize;
    }

    if map[y][x] == MapObject2::Wall {
        None
    } else {
        Some((x as i32, y as i32))
    }
}

fn search_up(
    check_pos: (i32, i32),
    last_tile: Option<MapObject2>,
    y_dir: i32, 
    map: &Vec<Vec<MapObject2>>, 
    sections: &mut HashMap<i32, (i32, i32)>,
    visited: &mut HashSet<(i32, i32)>
) -> Option<()> {
    let tile = map[check_pos.1 as usize][check_pos.0 as usize];
    if tile == MapObject2::Empty {
        return Some(());
    } else if tile == MapObject2::Wall {
        return None;
    }

    if visited.contains(&check_pos) {
        return Some(());
    }
    visited.insert(check_pos);

    let (min_y, max_y) = sections.get(&check_pos.0).cloned().unwrap_or((10000000, -1000000));

    let updated_section = (
        min_y.min(check_pos.1),
        max_y.max(check_pos.1)
    );
    sections.insert(check_pos.0, updated_section);


    let other_x = if tile == MapObject2::LBox {
        check_pos.0 + 1
    } else {
        check_pos.0 - 1
    };
    let other_tile = if tile == MapObject2::LBox {
        MapObject2::RBox
    } else {
        MapObject2::LBox
    };


    search_up(
        (check_pos.0, check_pos.1 + y_dir),
        Some(tile),
        y_dir,
        map,
        sections,
        visited
    )?;

    if last_tile.unwrap_or(other_tile) != tile {
        // println!("{check_pos:?}, {other_x}");
        search_up(
            (other_x, check_pos.1),
            Some(other_tile),
            y_dir,
            map,
            sections,
            visited
        )?;
    } 



    return Some(())
}
fn part2(robot_pos: (i32, i32), map: Vec<Vec<MapObject>>, directions: Vec<Direction>) -> usize {
    let mut robot = (robot_pos.0 * 2, robot_pos.1);
    let mut map = map.into_iter().map(|line| {
        line.into_iter().flat_map(|obj| {
            match obj {
                MapObject::Empty => vec![MapObject2::Empty; 2],
                MapObject::Wall => vec![MapObject2::Wall; 2],
                MapObject::Box => vec![MapObject2::LBox, MapObject2::RBox]
            }
        }).collect::<Vec<MapObject2>>()
    }).collect::<Vec<Vec<MapObject2>>>();

    for direction in directions {
        // print_map2(&map, robot);
        let dir = direction.to_dir();
        let new_pos = (robot.0 + dir.0, robot.1 + dir.1);

        let new_tile = map[new_pos.1 as usize][new_pos.0 as usize];
        if new_tile == MapObject2::Empty {
            robot = new_pos;
            continue;
        } else if new_tile == MapObject2::Wall {
            continue;
        }

        match direction {
            Direction::UP
            | Direction::DOWN => {
                let mut sections = HashMap::new();
                let mut visited = HashSet::new();
                let res = search_up(new_pos, None, dir.1, &map, &mut sections, &mut visited);
                if let None = res {
                    continue;
                }

                // println!("{sections:#?}");
                for (x, (y_min, y_max)) in sections {
                    if dir.1 == -1 {
                        for y in (y_min-1)..=(y_max-1) {
                            map[y as usize][x as usize] = map[(y+1) as usize][x as usize];
                        }
                        map[y_max as usize][x as usize] = MapObject2::Empty;
                    } else {
                        for y in ((y_min+1)..=(y_max+1)).rev() {
                            map[y as usize][x as usize] = map[(y-1) as usize][x as usize];
                        }
                        map[y_min as usize][x as usize] = MapObject2::Empty;
                    }
                }
                robot = new_pos;
            }
            
            Direction::RIGHT
            | Direction::LEFT => {
                let check_pos = find_empty(new_pos, dir, &map);
                if let Some(mut pos) = check_pos {
                    while pos != new_pos {
                        map[pos.1 as usize][pos.0 as usize]
                            = map[(pos.1 - dir.1) as usize][(pos.0 - dir.0) as usize];
                        
                        pos = (pos.0 - dir.0, pos.1 - dir.1);
                    }
                    map[new_pos.1 as usize][new_pos.0 as usize] = MapObject2::Empty;

                    robot = new_pos;
                } else {
                    continue;
                }
            },
        }
    }

    // print_map2(&map, robot);

    map.into_iter().enumerate().fold(0, |acc, (y, line)| {
        line.into_iter().enumerate().fold(0, |acc, (x, obj)| {
            acc + match obj {
                MapObject2::LBox => 100 * y + x,
                _ => 0
            }
        }) + acc
    })
}

fn main() {
    let (robot_pos, map, directions) = parse_input();

    println!("part1: {}", part1(robot_pos, map.clone(), directions.clone()));

    println!("part2: {}", part2(robot_pos, map, directions));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MapObject {
    Empty,
    Box,
    Wall
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MapObject2 {
    Empty,
    LBox,
    RBox,
    Wall
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}
impl Direction {
    fn to_dir(self) -> (i32, i32) {
        match self {
            Self::UP => (0, -1),
            Self::RIGHT => (1, 0),
            Self::DOWN => (0, 1),
            Self::LEFT => (-1, 0),
        }
    }

}
fn parse_input() -> ((i32, i32), Vec<Vec<MapObject>>, Vec<Direction>) {
    let inp = include_str!("input.txt");

    let parts: Vec<&str> = inp.split("\n\n").collect();

    let map = parts[0];
    let mut robot_pos = (-1, -1);
    let map: Vec<Vec<MapObject>> = map.split("\n").enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, obj)| {
            match obj {
                '.' => MapObject::Empty,
                '#' => MapObject::Wall,
                'O' => MapObject::Box,
                '@' => {
                    robot_pos = (x as i32, y as i32);
                    MapObject::Empty
                },
                _ => unreachable!()
            }
        }).collect::<Vec<MapObject>>()
    }).collect();

    assert!(robot_pos != (-1, -1));

    let moves: String = parts[1].split("\n").map(|line| line.to_string()).collect::<Vec<String>>().join("");
    let moves: Vec<Direction> = moves.chars().map(|mv| {
        match mv {
            '^' => Direction::UP,
            '>' => Direction::RIGHT,
            'v' => Direction::DOWN,
            '<' => Direction::LEFT,
            _ => {
                println!("{mv}");
                unreachable!()
            }
        }
    }).collect();

    (robot_pos, map, moves)

}