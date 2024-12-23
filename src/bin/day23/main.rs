use std::collections::{HashMap, HashSet};

fn part1(connections: &HashMap<String, HashSet<String>>) -> usize {
    let mut sets = HashSet::new();
    for (base, connected_comps) in connections {
        for connected in connected_comps {
            if connected == base {
                continue;
            }
            let second_connected = connections.get(connected).unwrap();
            connected_comps.intersection(second_connected).for_each(|similar| {
                if similar != base && similar != connected {
                    let mut key = [base, connected, similar];
                    key.sort();
                    sets.insert(key);
                }
            });
        }
    }
    
    sets.into_iter().filter(|list| list.into_iter().any(|a| a.starts_with("t"))).count()
}


fn find_group(computer: &str, prev_similar: &HashSet<String>, visited: &mut HashSet<String>, skip: &mut HashSet<String>, connections: &HashMap<String, HashSet<String>>) -> Vec<String> {
    if visited.contains(computer) {
        return vec![]
    }
    visited.insert(computer.to_string());

    let mut as_list = visited.clone().into_iter().collect::<Vec<String>>();
    as_list.sort();
    let as_list = as_list.join(",");
    if skip.contains(&as_list) {
        return vec![]
    }
    skip.insert(as_list.clone());

    let con = connections.get(computer).unwrap();
    let similar: HashSet<String> = HashSet::from_iter(con.intersection(prev_similar).cloned());

    for vis in visited.iter() {
        let con = connections.get(vis).unwrap();
        for vis2 in visited.iter() {
            if vis == vis2 {
                continue;
            }
            if !con.contains(vis2) {
                visited.remove(computer);
                return vec![];
            }
        }
    }

    if similar.len() == 0 {
        visited.remove(computer);
        return vec![as_list];
    }
    let mut out = HashSet::new();
    for sim in &similar {
        for new in find_group(sim, &similar, visited, skip, connections) {
            out.insert(new);
        }
    }

    visited.remove(computer);
    out.into_iter().collect()
}
fn part2(connections: &HashMap<String, HashSet<String>>) -> String {
    let mut groups = HashSet::new();
    let mut skip = HashSet::new();
    for (computer, _) in connections {
        let similar = connections.get(computer).unwrap().clone();
        for new in find_group(computer, &similar, &mut HashSet::new(), &mut skip, connections) {
            groups.insert(new);
        }
    } 
    
    let max = groups.into_iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap();
    max
}
fn main() {
    let inp = parse_input();
    println!("part1: {}", part1(&inp));
    println!("part2: {}", part2(&inp));
}

fn parse_input() -> HashMap<String, HashSet<String>> {
    let inp = include_str!("input.txt");

    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    for line in inp.split("\n") {
        let parts = line.split("-").collect::<Vec<&str>>();
        let a = parts[0].to_string();
        let b = parts[1].to_string();

        if let Some(val) = connections.get_mut(&a) {
            val.insert(b.clone());
        } else {
            let mut set = HashSet::new();
            set.insert(b.clone());
            connections.insert(a.clone(), set);
        }

        if let Some(val) = connections.get_mut(&b) {
            val.insert(a.clone());
        } else {
            let mut set = HashSet::new();
            set.insert(a.clone());
            connections.insert(b.clone(), set);
        }
        assert!(connections.get(&a).unwrap().contains(&b));
        assert!(connections.get(&b).unwrap().contains(&a));
    }
    connections
}