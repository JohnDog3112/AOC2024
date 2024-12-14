use std::collections::HashSet;


#[derive(Clone, Debug)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32)
}
impl Robot {
    fn new(text: &str) -> Self {
        let parts: Vec<(i32, i32)> = text.split(" ").map(|parts| {
            let nums = parts.split("=").skip(1).next().unwrap();
            let nums: Vec<i32> = nums.split(",").map(|num| num.parse::<i32>().unwrap()).collect();
            (nums[0], nums[1])
        }).collect();

        Self {
            pos: parts[0],
            vel: parts[1]
        }
    }

    fn step(&mut self, bounds: (i32, i32)) {
        self.pos = (
            (self.pos.0 + self.vel.0).rem_euclid(bounds.0),
            (self.pos.1 + self.vel.1).rem_euclid(bounds.1)
        )
    }
}

const DIRECTIONS: [(i32, i32); 4] = [
    (0, -1), (1, 0), (0, 1), (-1, 0)
];

fn get_max_connected_area(map: &HashSet<(i32, i32)>) -> i32 {
    let mut max = 0;
    let mut map = map.clone();
    while let Some(&next) = map.iter().next() {
        let mut area = 0;
        let mut queue = vec![next];
        map.remove(&next);
        while let Some(next) = queue.pop() {
            area += 1;
            for dir in DIRECTIONS {
                let next_pos = (
                    next.0 + dir.0,
                    next.1 + dir.1
                );
                if map.contains(&next_pos) {
                    queue.push(next_pos);
                    map.remove(&next_pos);
                }
            }
        }
        if area > max {
            max = area;
        }
    }

    max
}
fn part2(robots: Vec<Robot>) {
    let mut robots = robots.clone();
    let mut steps: Vec<(HashSet<(i32, i32)>, i32, i32)> = vec![];
    let bounds = (101, 103);

    for i in 1..10000 {
        robots.iter_mut().for_each(|robot| robot.step(bounds));

        let map: HashSet<(i32, i32)> = HashSet::from_iter(
            robots.iter().map(|robot| robot.pos)
        );
        let map_max_area = get_max_connected_area(&map);
        
        steps.push((map, map_max_area, i));

        if i%100 == 0 {
            println!("{i}");
        }
    }

    //highest to lowest
    steps.sort_by(|(_, a, _), (_, b, _)| b.cmp(a));

    //assume highest amount is the answer (worked in testing on my input)
    let mut map = vec![vec!['.'; bounds.0 as usize]; bounds.1 as usize];
    for &(x, y) in &steps[0].0 {
        map[y as usize][x as usize] = '#';
    }
    println!("\n\nindex: {}, area: {}\n{}", steps[0].2, steps[0].1, map.into_iter().map(|line| String::from_iter(line)).collect::<Vec<String>>().join("\n"));

    // for i in 0..10 {
    //     let mut map = vec![vec!['.'; bounds.0 as usize]; bounds.1 as usize];
    //     for &(x, y) in &steps[i].0 {
    //         map[y as usize][x as usize] = '#';
    //     }
    //     println!("\n\n{}\n{}", steps[i].1, map.into_iter().map(|line| String::from_iter(line)).collect::<Vec<String>>().join("\n"));
    // }
}

fn main() {
    part2(parse());
}

fn parse() -> Vec<Robot> {
    let inp = include_str!("input.txt");
    inp.split("\n").map(Robot::new).collect()
}

