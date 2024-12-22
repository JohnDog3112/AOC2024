use std::collections::{HashMap, HashSet, VecDeque};

fn part1(buyers: Vec<i128>) -> i128 {
    buyers.into_iter().map(|mut buyer| {
        for _ in 0..2000 {
            buyer = (buyer ^ (buyer*64)) % 16777216;
            buyer = (buyer ^ (buyer/32)) % 16777216;
            buyer = (buyer ^ (buyer*2048)) % 16777216;
        }
        buyer
    }).sum()
}

fn part2(buyers: Vec<i128>) -> i128 {
    let buyers = buyers.into_iter().map(|mut buyer| {
        (0..2000).map(|_| {
            buyer = (buyer ^ (buyer*64)) % 16777216;
            buyer = (buyer ^ (buyer/32)) % 16777216;
            buyer = (buyer ^ (buyer*2048)) % 16777216;
            buyer % 10
        }).collect()
    }).collect::<Vec<Vec<i128>>>();

    let mut seqs = HashMap::new();
    for buyer in buyers {
        let mut seen_seqs = HashSet::new();
        let mut window = VecDeque::new();
        let mut seq = VecDeque::new();
        for price in buyer {
            window.push_back(price);
            if window.len() < 2 {
                continue;
            }
            seq.push_back(price - window[window.len() - 2]);
            if window.len() == 6 {
                seq.pop_front();
                window.pop_front();
            } else if window.len() < 5 {
                continue;
            }
            // println!("{window:?}, {seq:?}");
            let t_seq = seq.clone().into_iter().collect::<Vec<i128>>();
            if seen_seqs.contains(&t_seq) {
                continue;
            }
            seen_seqs.insert(t_seq.clone());

            if let Some(val) = seqs.get_mut(&t_seq) {
                *val += price;
            } else {
                seqs.insert(t_seq, price);
            }
        }
    }
    let max = seqs.into_iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    // println!("max seq: {:?}", max.0);
    max.1
}
fn main() {
    let buyers = parse_input();
    println!("part1: {}", part1(buyers.clone()));
    println!("part2: {}", part2(buyers));
}

fn parse_input() -> Vec<i128> {
    let inp = include_str!("input.txt");

    inp.split("\n").map(|line| {
        line.parse::<i128>().unwrap()
    }).collect()
}