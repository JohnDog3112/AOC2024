use std::collections::{HashMap, HashSet, VecDeque};


fn part1(wires: HashMap<String, usize>, gates: Vec<Gate>) -> usize {
    let mut mapped_gates: HashMap<String, Vec<Gate>> = HashMap::new();
    for gate in gates {

        if let Some(val) = mapped_gates.get_mut(&gate.a) {
            val.push(gate.clone());
        } else {
            mapped_gates.insert(gate.a.clone(), vec![gate.clone()]);
        }
        if let Some(val) = mapped_gates.get_mut(&gate.b) {
            val.push(gate.clone());
        } else {
            mapped_gates.insert(gate.b.clone(), vec![gate.clone()]);
        }

    }

    let mut wires = wires;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    for (wire, _) in &wires {
        queue.push_back(wire.clone());
    }

    while let Some(wire) = queue.pop_front() {
        if visited.contains(&wire) {
            continue;
        }
        visited.insert(wire.clone());

        let gates = match mapped_gates.get(&wire) {
            Some(gates) => gates,
            None => continue
        };

        for gate in gates {
            let a = wires.get(&gate.a);
            let b = wires.get(&gate.b);
            let out = wires.get(&gate.out);

            if let (Some(&a), Some(&b), None) = (a, b, out) {
                let out = match gate.typ {
                    GateType::Xor => a ^ b,
                    GateType::And => a & b,
                    GateType::Or => a | b,
                };
                wires.insert(gate.out.clone(), out);
                queue.push_back(gate.out.clone());
            }
        }
    }
    let mut out = 0;
    for (wire, val) in wires {
        if wire.starts_with("z") {
            let index = wire[1..].parse::<usize>().unwrap();
            out += val << index;
        }
    }
    out
}

fn swap_outputs(mut gates: Vec<Gate>, a: String, b: String) -> Vec<String> {
    // println!("swap: {a}, {b}");
    for gate in &mut gates {
        if gate.out == a {
            gate.out = b.to_string();
        } else if gate.out == b {
            gate.out = a.to_string();
        }
    }
    let mut out = part2(gates);
    out.push(a);
    out.push(b);
    out
}
fn part2(top_gates: Vec<Gate>) -> Vec<String> {
    let mut mapped_gates: HashMap<String, Vec<Gate>> = HashMap::new();
    for gate in &top_gates {

        if let Some(val) = mapped_gates.get_mut(&gate.a) {
            val.push(gate.clone());
        } else {
            mapped_gates.insert(gate.a.clone(), vec![gate.clone()]);
        }
        if let Some(val) = mapped_gates.get_mut(&gate.b) {
            val.push(gate.clone());
        } else {
            mapped_gates.insert(gate.b.clone(), vec![gate.clone()]);
        }
    }
    let mut reverse_mapped_gates: HashMap<String, Gate> = HashMap::new();
    for gate in &top_gates {
        if let Some(_) = reverse_mapped_gates.get_mut(&gate.out) {
            assert!(false);
        } else {
            reverse_mapped_gates.insert(gate.out.clone(), gate.clone());
        }
    }
    
    let start = mapped_gates.get("x00").unwrap();
    assert!(start.len() == 2);
    assert!((start[0].a == "x00" && start[0].b == "y00") || (start[0].b == "x00" && start[0].a == "y00"));
    assert!((start[1].a == "x00" && start[1].b == "y00") || (start[1].b == "x00" && start[1].a == "y00"));
    let mut carries_map = HashMap::new();
    let mut carries = vec![];
    match (start[0].typ, start[1].typ) {
        (GateType::Xor, GateType::And) => {
            assert!(start[0].out == "z00");
            carries_map.insert(start[1].out.clone(), 0);
            carries.push(start[1].out.clone());
        },
        (GateType::And, GateType::Xor) => {
            assert!(start[1].out == "z00");
            carries_map.insert(start[0].out.clone(), 0);
            carries.push(start[0].out.clone());
        },
        _ => unimplemented!()
    }

    let mut x = 1;
    while let Some(gates) = mapped_gates.get(&format!("x{x:02}")) {
        assert!(gates.len() == 2);
        let x_str = format!("x{x:02}");
        let y_str = format!("y{x:02}");
        assert!((gates[0].a == x_str && gates[0].b == y_str) || (gates[0].b == x_str && gates[0].a == y_str));
        assert!((gates[1].a == x_str && gates[1].b == y_str) || (gates[1].b == x_str && gates[1].a == y_str));

        let (and, xor) = match (gates[0].typ, gates[1].typ) {
            (GateType::Xor, GateType::And) => (&gates[1], &gates[0]),
            (GateType::And, GateType::Xor) => (&gates[0], &gates[1]),
            _ => unimplemented!()
        };
        
        let from_xor = mapped_gates.get(&xor.out).unwrap();
        if from_xor.len() != 2 {
            let out = reverse_mapped_gates.get(&format!("z{x:02}")).unwrap();
            assert!(out.typ == GateType::Xor);
            // println!("carries: {carries_map:?}");
            // println!("out: {out:?}");
            let shared = match (carries_map.get(&out.a), carries_map.get(&out.b)) {
                (Some(_), None) => &out.b,
                (None, Some(_)) => &out.a,
                _ => unreachable!()
            };
            // let shared_gate = reverse_mapped_gates.get(shared).unwrap();
            assert!(mapped_gates.get(shared).unwrap().len() == 2); 
            return swap_outputs(top_gates, xor.out.clone(), shared.clone());
        }
        assert!(from_xor.len() == 2);

        let (from_xor_and, from_xor_xor) = match (from_xor[0].typ, from_xor[1].typ) {
            (GateType::Xor, GateType::And) => (&from_xor[1], &from_xor[0]),
            (GateType::And, GateType::Xor) => (&from_xor[0], &from_xor[1]),
            _ => unimplemented!()
        };
        assert!(carries[x-1] == from_xor_xor.a || carries[x-1] == from_xor_xor.b);
        if from_xor_xor.out != format!("z{x:02}") {
            return swap_outputs(top_gates, format!("z{x:02}"), from_xor_xor.out.clone());
        }
        assert!(from_xor_xor.out == format!("z{x:02}"));

        let from_and = mapped_gates.get(&and.out).unwrap();
        assert!(from_and.len() == 1);
        assert!(from_and[0].typ == GateType::Or);
        assert!(from_and[0].a == from_xor_and.out || from_and[0].b == from_xor_and.out);
        carries.push(from_and[0].out.clone());
        carries_map.insert(from_and[0].out.clone(), x);


        x += 1;
    }

    vec![]

}
fn main() {
    let (wires, gates) = parse_input();
    println!("part1: {}", part1(wires, gates.clone()));
    let mut switched = part2(gates);
    switched.sort();
    println!("part2: {}", switched.join(","));
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Gate {
    a: String,
    b: String,
    out: String,
    typ: GateType
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum GateType {
    Xor,
    And,
    Or
}
fn parse_input() -> (HashMap<String, usize>, Vec<Gate>) {
    let inp = include_str!("input.txt");

    let parts: Vec<&str> = inp.split("\n\n").collect();
    
    let initial_wire_vals: HashMap<String, usize> = HashMap::from_iter(parts[0].split("\n").map(|line| {
        let sections: Vec<&str> = line.split(": ").collect();
        (
            sections[0].to_string(),
            sections[1].parse::<usize>().unwrap()
        )
    }));

    let gates: Vec<Gate> = parts[1].split("\n").map(|line| {
        let parts: Vec<&str> = line.split(" ").collect();
        // println!("{parts:?}");
        let a = parts[0].to_string();
        let b = parts[2].to_string();
        let out = parts[4].to_string();
        let typ = match parts[1] {
            "XOR" => GateType::Xor,
            "AND" => GateType::And,
            "OR" => GateType::Or,
            _ => unreachable!()
        };

        Gate {
            a,
            b,
            out,
            typ
        }
    }).collect();

    (initial_wire_vals, gates)
}