use std::{collections::HashMap, hash::Hash};

use itertools::Itertools;

#[allow(unused)]
fn part1(codes: &Vec<Vec<KeyPadCode>>) -> i128 {
    codes.into_iter().map(|code| {
        let (_, path) = code.into_iter().fold((KeyPadCode::A.get_pos(), vec![]), |acc, next| {
            let (n_pos, path) = next.find_path(acc.0);
            (
                n_pos,
                if acc.1.len() == 0 {
                    path
                } else {
                    acc.1.into_iter().cartesian_product(path)
                        .map(|(a,b)| {
                            [a, b].concat()
                        }).collect::<Vec<Vec<MovePadCode>>>()
                }
            )
        });

        let path= path.into_iter().flat_map(|opt| {
            let (_, path) = opt.into_iter().fold((MovePadCode::A, vec![]), |acc, next| {
                let to_concat = if acc.0 == next {
                    vec![vec![MovePadCode::A]]
                } else {
                    let (_, path) = next.find_path(acc.0.get_pos());
                    path
                };

                (
                    next,
                    if acc.1.len() == 0 {
                        to_concat
                    } else {
                        acc.1.into_iter().cartesian_product(to_concat).map(|(a, b)| [a, b].concat()).collect()
                    }
                )
            });
            path
        }).collect::<Vec<Vec<MovePadCode>>>();

        let min = path.iter().map(|a| a.len()).min().unwrap();
        let path = path.into_iter().filter(|a| a.len() == min).collect::<Vec<Vec<MovePadCode>>>();

        let path= path.into_iter().flat_map(|opt| {
            let (_, path) = opt.into_iter().fold((MovePadCode::A, vec![]), |acc, next| {
                let to_concat = if acc.0 == next {
                    vec![vec![MovePadCode::A]]
                } else {
                    let (_, path) = next.find_path(acc.0.get_pos());
                    path
                };

                (
                    next,
                    if acc.1.len() == 0 {
                        to_concat
                    } else {
                        acc.1.into_iter().unique().cartesian_product(to_concat).map(|(a, b)| [a, b].concat()).collect()
                    }
                )
            });
            path
        }).collect::<Vec<Vec<MovePadCode>>>();
        
        let n_code = code.into_iter().filter_map(|code| code.get_num()).fold(0, |acc, next| acc*10 + next);
        let min_len = path.into_iter().map(|a| a.len()).min().unwrap();

        n_code * min_len as i128
    }).sum()
}

fn to_hashmap(seq: &[MovePadCode]) -> HashMap<(MovePadCode, MovePadCode), usize> {
    let mut map = HashMap::new();
    for (i, &code) in seq.iter().enumerate() {
        let prev = if i == 0 {
            MovePadCode::A
        } else {
            seq[i - 1]
        };
        if let Some(prev) = map.get_mut(&(prev, code)) {
            *prev += 1;
        } else {
            map.insert((prev, code), 1);
        }
    }
    map
}

fn gen_expansion() -> Vec<HashMap<(MovePadCode, MovePadCode), Vec<MovePadCode>>> {
    let move_codes = [
        MovePadCode::A,
        MovePadCode::Up,
        MovePadCode::Down,
        MovePadCode::Left,
        MovePadCode::Right
    ];

    let mut finished = 0;
    //brute force for the best moveset between any given move pad key and any other key
    let move_pad_expansion: HashMap<(MovePadCode, MovePadCode), Vec<Vec<MovePadCode>>> = HashMap::from_iter(move_codes.clone().into_iter().cartesian_product(move_codes).map(|(a, b)| {
        let (_, paths) = b.find_path(a.get_pos());
        (
            (a, b),
            if paths.len() == 1 || paths[0].len() < paths[1].len() || paths[0].len() == 1 || paths[0] == paths[1] {
                finished += 1;
                vec![paths[0].clone()]
            } else if paths[0].len() > paths[1].len() {
                finished += 1;
                vec![paths[1].clone()]
            } else {
                paths
            }
        )
    }));

    // loop {
    //     // println!("finished: {finished}");
    //     let mut maps: HashMap<(usize, (MovePadCode, MovePadCode)), Vec<HashMap<(MovePadCode, MovePadCode), usize>>> = HashMap::new();
    //     let expansions = move_pad_expansion.iter()
    //         .fold(vec![HashMap::new()], |acc: Vec<HashMap<(MovePadCode, MovePadCode), Vec<MovePadCode>>>, (key, paths)| {
    //             acc.into_iter().cartesian_product(paths.clone()).map(|(mut h, val)| {
    //                 h.insert(key.clone(), val);
    //                 h
    //             }).collect()
    //         }).into_iter().map(|a| HashMap::from_iter(a.into_iter().map(|(key, val)| (key, to_hashmap(&val)))))
    //         .collect::<Vec<HashMap<(MovePadCode, MovePadCode), HashMap<(MovePadCode, MovePadCode), usize>>>>();
    //     'loop2: loop {
    //         println!("step!");
    //         let ext_expansion = move_pad_expansion.clone();
    //         for (key, paths) in ext_expansion {
    //             if paths.len() == 1 {
    //                 continue;
    //             }
    //             let paths = paths.into_iter().map(|a| to_hashmap(&a)).collect::<Vec<HashMap<(MovePadCode, MovePadCode), usize>>>();
                
    //             for (i, expansion) in expansions.iter().enumerate() {
                    
    //             }
    //         }
    //     }
    // }

    move_pad_expansion.iter()
        .fold(vec![HashMap::new()], |acc: Vec<HashMap<(MovePadCode, MovePadCode), Vec<MovePadCode>>>, (key, paths)| {
            acc.into_iter().cartesian_product(paths.clone()).map(|(mut h, val)| {
                h.insert(key.clone(), val);
                h
            }).collect()
        })
}
fn part2(codes: &Vec<Vec<KeyPadCode>>, expansions: &Vec<HashMap<(MovePadCode, MovePadCode), Vec<MovePadCode>>>, robots: i128) -> usize {

    let mut total = 0;
    for code in codes {
        let (_, path) = code.into_iter().fold((KeyPadCode::A.get_pos(), vec![]), |acc, next| {
            let (n_pos, path) = next.find_path(acc.0);
            (
                n_pos,
                if acc.1.len() == 0 {
                    path
                } else {
                    acc.1.into_iter().cartesian_product(path)
                        .map(|(a,b)| {
                            [a, b].concat()
                        }).collect::<Vec<Vec<MovePadCode>>>()
                }
            )
        });
        let min: usize = expansions.iter().map(|expansion| {
            path.iter().map(|path| {
                let mut path = to_hashmap(&path);
                for _ in 0..(robots-1) {
                    let mut n_path = HashMap::new();
                    for ((prev, next), count) in path {
                        for (key, count2) in to_hashmap(expansion.get(&(prev, next)).unwrap()) {
                            if let Some(val) = n_path.get_mut(&key) {
                                *val += count*count2;
                            } else {
                                n_path.insert(key, count*count2);
                            }
                        }
                    }
                    path = n_path;
                }
                path.into_iter().map(|(_, count)| count).sum()
            }).min().unwrap()
        }).min().unwrap();
         

        let n_code = code.into_iter().filter_map(|code| code.get_num()).fold(0, |acc, next| acc*10 + next);
        total += n_code * min as i128;
    }
    // println!("{total}");
    total as usize
}
fn main() {
    let codes = parse_input();
    
    let expansions = gen_expansion();
    println!("part1: {}", part2(&codes, &expansions, 3));
    println!("part2: {}", part2(&codes, &expansions, 26))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum MovePadCode {
    Up,
    Right,
    Down,
    Left,
    A
}
impl MovePadCode {
    #[allow(unused)]
    fn to_string(self) -> String {
        match self {
            MovePadCode::Up => "^",
            MovePadCode::Right => ">",
            MovePadCode::Down => "v",
            MovePadCode::Left => "<",
            MovePadCode::A => "A",
        }.to_string()
    }
    fn get_pos(self) -> (i128, i128) {
        match self {
            MovePadCode::Up => (1, 0),
            MovePadCode::Right => (2, 1),
            MovePadCode::Down => (1, 1),
            MovePadCode::Left => (0, 1),
            MovePadCode::A => (2, 0),
        }
    }

    fn find_path(self, pos: (i128, i128)) -> ((i128, i128), Vec<Vec<MovePadCode>>) {
        let target = self.get_pos();
        let offset = (target.0 - pos.0, target.1 - pos.1);

        let horizontal = vec![if offset.0 > 0 { MovePadCode::Right } else { MovePadCode::Left }; offset.0.abs() as usize];
        let vertical = vec![if offset.1 > 0 { MovePadCode::Down } else { MovePadCode::Up }; offset.1.abs() as usize];

        let mut paths = if pos.1 == 0 && target.1 != 0 && target.0 == 0 {
            vec![[vertical, horizontal].concat()]
        } else if pos.1 != 0 && target.1 == 0 && pos.0 == 0 {
            vec![[horizontal, vertical].concat()]
        } else {
            vec![
                [vertical.clone(), horizontal.clone()].concat(),
                [horizontal, vertical].concat()
            ]
        };
        for path in &mut paths {
            path.push(MovePadCode::A);
        }

        (target, paths)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KeyPadCode {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A
}
impl KeyPadCode {
    #[allow(unused)]
    fn to_string(self) -> String {
        match self {
            KeyPadCode::Zero => "0",
            KeyPadCode::One => "1",
            KeyPadCode::Two => "2",
            KeyPadCode::Three => "3",
            KeyPadCode::Four => "4",
            KeyPadCode::Five => "5",
            KeyPadCode::Six => "6",
            KeyPadCode::Seven => "7",
            KeyPadCode::Eight => "8",
            KeyPadCode::Nine => "9",
            KeyPadCode::A => "A",
        }.to_string()
    }
    fn get_num(self) -> Option<i128> {
        match self {
            KeyPadCode::Zero => Some(0),
            KeyPadCode::One => Some(1),
            KeyPadCode::Two => Some(2),
            KeyPadCode::Three => Some(3),
            KeyPadCode::Four => Some(4),
            KeyPadCode::Five => Some(5),
            KeyPadCode::Six => Some(6),
            KeyPadCode::Seven => Some(7),
            KeyPadCode::Eight => Some(8),
            KeyPadCode::Nine => Some(9),
            KeyPadCode::A => None,
        }
    }
    fn get_pos(self) -> (i128, i128) {
        match self {
            KeyPadCode::Zero => (1, 3),
            KeyPadCode::One => (0, 2),
            KeyPadCode::Two => (1, 2),
            KeyPadCode::Three => (2, 2),
            KeyPadCode::Four => (0, 1),
            KeyPadCode::Five => (1, 1),
            KeyPadCode::Six => (2, 1),
            KeyPadCode::Seven => (0, 0),
            KeyPadCode::Eight => (1, 0),
            KeyPadCode::Nine => (2, 0),
            KeyPadCode::A => (2, 3),
        }
    }

    fn find_path(self, pos: (i128, i128)) -> ((i128, i128), Vec<Vec<MovePadCode>>) {
        let target = self.get_pos();
        let offset = (target.0 - pos.0, target.1 - pos.1);

        let horizontal = vec![if offset.0 > 0 { MovePadCode::Right } else { MovePadCode::Left }; offset.0.abs() as usize];
        let vertical = vec![if offset.1 > 0 { MovePadCode::Down } else { MovePadCode::Up }; offset.1.abs() as usize];

        let mut movements = if pos.1 == 3 && target.1 != 3 && target.0 == 0 {
            vec![[vertical, horizontal].concat()]
        } else if pos.1 != 3 && target.1 == 3 && pos.0 == 0 {
            vec![[horizontal, vertical].concat()]
        } else {
            vec![
                [horizontal.clone(), vertical.clone()].concat(),
                [vertical, horizontal].concat()
            ]
        };

        for movement in &mut movements {
            movement.push(MovePadCode::A);
        }
        

        (target, movements)
    }
}
fn parse_input() -> Vec<Vec<KeyPadCode>> {
    let inp = include_str!("input.txt");

    inp.split("\n").map(|line| {
        line.chars().map(|key| {
            match key {
                '0' => KeyPadCode::Zero,
                '1' => KeyPadCode::One,
                '2' => KeyPadCode::Two,
                '3' => KeyPadCode::Three,
                '4' => KeyPadCode::Four,
                '5' => KeyPadCode::Five,
                '6' => KeyPadCode::Six,
                '7' => KeyPadCode::Seven,
                '8' => KeyPadCode::Eight,
                '9' => KeyPadCode::Nine,
                'A' => KeyPadCode::A,
                _ => unreachable!()
            }
        }).collect()
    }).collect()
}