use itertools::Itertools;

fn part1(keys: &Vec<Schematic>, locks: &Vec<Schematic>) -> usize {
    keys.iter().cartesian_product(locks).filter(|(key, lock)| {
        key.heights.iter()
            .zip(lock.heights)
            .map(|(a, b)| a+b)
            .all(|val| val <= 7)
    }).count()
}

fn main() {
    let (keys, locks) = parse_input();
    println!("part1: {}", part1(&keys, &locks));
}

#[derive(Clone, Debug)]
struct Schematic {
    heights: [usize; 5],
    typ: SchematicTyp,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
enum SchematicTyp {
    Key,
    Lock
}
fn parse_input() -> (Vec<Schematic>, Vec<Schematic>) {
    let inp = include_str!("input.txt");
    
    let schematics = inp.split("\n\n").map(|block| {
        let block: Vec<Vec<char>> = block.split("\n").map(|line| line.chars().collect()).collect();
        let mut heights = [0; 5];
        for x in 0..5 {
            for y in 0..7 {
                if block[y][x] == '#' {
                    heights[x] += 1;
                }
            }
        }
        let typ = if block[0][0] == '#' {
            SchematicTyp::Lock
        } else {
            SchematicTyp::Key
        };

        Schematic {
            heights,
            typ
        }
    });

    schematics.fold((vec![], vec![]), |mut acc, next| {
        match next.typ {
            SchematicTyp::Key => acc.0.push(next),
            SchematicTyp::Lock => acc.1.push(next),
        }
        acc
    })
}