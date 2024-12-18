use std::{collections::{HashMap, HashSet}, ops::{Shl, Shr}, str::FromStr};
use num_bigint::{BigUint, ToBigUint};

#[derive(Clone, Debug)]
struct Registers {
    a: BigUint,
    b: BigUint,
    c: BigUint,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Combo {
    Zero,
    One,
    Two,
    Three,
    A,
    B,
    C,
    Seventh
}
impl Combo {
    fn from_num(num: u128) -> Self {
        match num {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::A,
            5 => Self::B,
            6 => Self::C,
            7 => Self::Seventh,
            _ => unreachable!()
        }
    }

    fn to_val(self, registers: &Registers) -> Option<BigUint> {
        match self {
            Self::Zero => Some(0.to_biguint().unwrap()),
            Self::One => Some(1.to_biguint().unwrap()),
            Self::Two => Some(2.to_biguint().unwrap()),
            Self::Three => Some(3.to_biguint().unwrap()),
            Self::A => Some(registers.a.clone()),
            Self::B => Some(registers.b.clone()),
            Self::C => Some(registers.c.clone()),
            Self::Seventh => None
        }
    }
}

enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv
}
impl Instruction {
    fn from_digit(num: u128) -> Self {
        match num {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!()
        }
    }
}


fn part1(registers: Registers, program: &Vec<u128>) -> Vec<u128> {
    let mut registers = registers;
    let mut pc = 0;

    let mut out: Vec<u128> = vec![];

    while pc < program.len() {
        let inst = Instruction::from_digit(program[pc]);
        let literal = program[pc+1];
        let combo = Combo::from_num(literal);
        
        let val = combo.to_val(&registers).unwrap();
        // println!("{val}, {:?}", val.clone().to_u64_digits());
        // assert!(val.to_u64_digits().len() <= 1);
        // let val = 1;

        // println!("{pc}: {registers:?}");
        // println!("{}, {}", program[pc], program[pc+1]);
        pc += 2;


        match inst {
            Instruction::Adv => {
                let digs = val.to_u32_digits();
                assert!(digs.len() <= 1);
                let val = digs.get(0).cloned().unwrap_or(0);
                registers.a = registers.a.clone() / 2u128.pow(val as u32);
            },
            Instruction::Bxl => {
                registers.b = registers.b ^ literal.to_biguint().unwrap();
            },
            Instruction::Bst => {
                registers.b = val % 8.to_biguint().unwrap();
            },
            Instruction::Jnz => {
                if registers.a.clone() != 0.to_biguint().unwrap() {
                    pc = literal as usize;
                }
            },
            Instruction::Bxc => {
                registers.b = registers.b.clone() ^ registers.c.clone();
            },
            Instruction::Out => {
                let tmp = val % 8.to_biguint().unwrap();
                let digs = tmp.to_u64_digits();
                assert!(digs.len() <= 1);
                out.push(digs.get(0).cloned().unwrap_or(0) as u128);
            }
            Instruction::Bdv => {
                let digs = val.to_u32_digits();
                assert!(digs.len() <= 1);
                let val = digs.get(0).cloned().unwrap_or(0);
                registers.b = registers.a.clone() / 2u128.pow(val as u32);
            },
            Instruction::Cdv => {
                let digs = val.to_u32_digits();
                assert!(digs.len() <= 1);
                let val = digs.get(0).cloned().unwrap_or(0);
                registers.c = registers.a.clone() / 2u128.pow(val as u32);
            }
        }

    }
    // println!("{registers:?}");
    // println!("{}", out.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(","));
    out
}

#[derive(Clone, Debug, Copy)]
enum APart {
    A(usize),
    Not(usize),
    Lit(u8),
    Range(usize, usize)
}
impl APart {
    fn get_len(self) -> usize {
        match self {
            APart::A(_)|APart::Not(_)|APart::Lit(_) => 1,
            APart::Range(start, end) => end-start+1
        }
    }
}
#[derive(Clone, Debug)]
enum Values {
    Literal(u128),
    A,
    AParts(Vec<APart>),
    Div(Box<Values>, Box<Values>),
    Xor(Box<Values>, Box<Values>),
    Mod8(Box<Values>),
    Possibilities(Vec<usize>, HashMap<usize, Values>),
}
impl Values {
    fn reduce(self) -> Self {
        match self {
            Values::Literal(_) => self,
            Values::A => Values::AParts(vec![APart::Range(0, 1000000)]),
            Values::AParts(parts) => {
                if parts.len() == 1 {
                    if let APart::Range(start, end) = parts[0] {
                        if end-start+1 < 16 {
                            Values::AParts(
                                (start..=end).map(|i| APart::A(i)).collect()
                            )
                        } else {
                            Values::AParts(parts)
                        }
                    } else {
                        Values::AParts(parts)
                    }
                } else if parts.iter().all(|val| if let APart::Lit(_) = val { true } else { false } ) {
                    Values::Literal(parts.iter().rfold(0, |acc, val| {
                        if let APart::Lit(lit) = val {
                            (acc*2) + *lit as u128
                        } else {
                            unreachable!()
                        }
                    }))
                } else {
                    Values::AParts(parts)
                }
            },
            Values::Div(values, values1) => {
                let values = values.reduce();
                let values1 = values1.reduce();
                match (values, values1) {
                    (
                        Values::AParts(parts), 
                        Values::Literal(lit)
                    ) => {
                        let n_parts: Vec<APart>;
                        
                        if parts.len() > lit as usize {
                            n_parts = (&parts[lit as usize..]).to_vec();
                        } else if parts.len() == lit as usize {
                            if let APart::Range(start, end) = parts[parts.len()-1] {
                                n_parts = vec![APart::Range(
                                    start + 1,
                                    end
                                )]
                            } else {
                                n_parts = vec![];
                            }
                        } else {
                            let last = parts[parts.len()-1];
                            if let APart::Range(start, end) = last {
                                n_parts = vec![APart::Range(
                                    start + (lit as usize + 1 - parts.len()) , 
                                    end
                                )];
                            } else {
                                unreachable!()
                            }
                        }
                        Values::AParts(n_parts).reduce()
                        // if let APart::Range(start, end) = parts[0] {
                        //     Self::AParts(vec![APart::Range(
                        //         (start + lit as usize).min(end),
                        //         end
                        //     )])
                        // } else {
                        //     unreachable!()
                        // }
                    },
                    (
                        values,
                        Values::AParts(parts1)
                    ) if Values::AParts(parts1.clone()).get_unknowns().map(|a| a.len()).unwrap_or(0) != 0 => {
                        let unknowns = Values::AParts(parts1.clone()).get_unknowns().unwrap();
                        let mut unknowns = unknowns.into_iter().collect::<Vec<usize>>();
                        unknowns.sort();

                        let parts1 = Values::AParts(parts1);
                        let mut poss = HashMap::new();

                        for i in 0..((2 as usize).pow(unknowns.len() as u32)) {
                            let mut vals = HashMap::new();
                            for j in 0..unknowns.len() {
                                vals.insert(
                                    unknowns[j],
                                    (i.shr(j)&1) as u8
                                );
                            }
                            poss.insert(
                                i,
                                Values::Div(
                                    Box::new(values.clone().apply(&vals)),
                                    Box::new(parts1.clone().apply(&vals))
                                ).reduce()
                            );
                        }

                        Values::Possibilities(unknowns, poss)
                    }
                    (values, values1) => Self::Div(
                        Box::new(values),
                        Box::new(values1)
                    )
                }
                
            },
            Values::Xor(values, values1) => {
                let values = values.reduce();
                let values1 = values1.reduce();
                match (values, values1) {
                    (
                        Values::Xor(a, b),
                        Values::Literal(lit1)
                    ) if matches!(*b, Values::Literal(_))=> Values::Xor(
                        a.clone(),
                        if let Values::Literal(lit) = *b {
                            Box::new(Values::Literal(lit ^ lit1))
                        } else { unreachable!() }
                    ),

                    (
                        Values::AParts(mut parts),
                        Values::Literal(mut lit)
                    )
                    | (
                        Values::Literal(mut lit),
                        Values::AParts(mut parts)
                    ) => {
                        let mut i = 0;
                        while lit > 0 && i < parts.len()  {
                            parts[i] = match (lit&1 == 1, parts[i]) {
                                (true, APart::A(val)) => APart::Not(val),
                                (true, APart::Not(val)) => APart::A(val),
                                (true, APart::Lit(lit)) => APart::Lit(
                                    if lit == 1 {
                                        0
                                    } else {
                                        1
                                    }
                                ),
                                (_, APart::Range(_, _)) => {
                                    assert!(i == parts.len()-1);
                                    break;
                                },
                                (false, _) => parts[i]
                            };
                            i += 1;
                            lit /= 2;
                        }
                        if lit != 0 {
                            if let Some(APart::Range(mut start, end)) = parts.pop() {
                                while lit > 0 {
                                    if lit&1 == 1 {
                                        parts.push(APart::Not(start));
                                    } else {
                                        parts.push(APart::A(start));
                                    }
                                    lit /= 2;
                                    start += 1;
                                }
                                parts.push(APart::Range(start, end));
                            } else {
                                unreachable!()
                            }
                        }
                        Values::AParts(parts)
                    }
                    (
                        Values::AParts(parts),
                        Values::Possibilities(unknowns, poss)
                    ) => {
                        // println!("before: {:#?}", Values::Xor(Box::new(Values::AParts(parts.clone())), Box::new(Values::Possibilities(unknowns.clone(), poss.clone()))));

                        let after = Values::Possibilities(
                            unknowns.clone(), 
                            HashMap::from_iter(
                                poss.into_iter().map(|(val, values)| {
                                    let mut vals = HashMap::new();
                                    for j in 0..unknowns.len() {
                                        vals.insert(
                                            unknowns[j],
                                            (val.shr(j)&1) as u8
                                        );
                                    }

                                    (
                                        val,
                                        Values::Xor(
                                            Box::new(Values::AParts(parts.clone()).apply(&vals)),
                                            Box::new(values)
                                        ).reduce()
                                    )
                                })
                            )
                        );
                        // println!("after: {:?}", after);
                        after
                    }
                    
                    (values, values1) => Values::Xor(
                        Box::new(values),
                        Box::new(values1)
                    )
                }
            },
            Values::Mod8(values) => {
                let values = values.reduce();
                match values {
                    Values::Literal(a) => Values::Literal(a%8),
                    Values::AParts(parts) 
                    if parts.len() == 1 && matches!(parts[0], APart::Range(_, _)) =>  {
                        if let APart::Range(start, end) = parts[0] {
                            Values::AParts(vec![APart::Range(
                                start,
                                (start+2).min(end)
                            )]).reduce()
                        } else { 
                            unreachable!()
                        }
                    },
                    Values::Possibilities(unknowns, poss)
                    if poss.iter().all(|(_, val)| if let Values::AParts(_) = val { true } else { false }) => {
                        Values::Possibilities(
                            unknowns, 
                            HashMap::from_iter(poss.into_iter().map(|(val, values)| {
                                let mut n_parts = vec![];
                                let mut counter = 0;
                                if let Values::AParts(parts) = values {
                                    while counter < parts.len() && counter < 3 {
                                        match parts[counter] {
                                            APart::A(_)
                                            |APart::Not(_)
                                            |APart::Lit(_) => {
                                                n_parts.push(parts[counter]);
                                                counter += 1;
                                            }
                                            APart::Range(start, end) => {
                                                assert!(counter == parts.len()-1);
                                                assert!(end-start+1 >= 3-counter);
                                                for i in 0..(3-counter) {
                                                    n_parts.push(APart::A(start+i));
                                                }
                                                break;
                                            },
                                        }
                                    }
                                } else {
                                    unreachable!()
                                }
                                
                                (
                                    val,
                                    Values::AParts(n_parts).reduce()
                                )
                            }))
                        )
                    }
                    _ => Values::Mod8(Box::new(values))
                }
            },
            Values::Possibilities(_, _) => self,
        }
    }

    fn get_unknowns_helper(&self, set: &mut HashSet<usize>) -> Option<()> {
        match self {
            Values::Literal(_) => Some(()),
            Values::A => None,
            Values::AParts(vec) => {
                for item in vec {
                    match item {
                        APart::A(val)
                        | APart::Not(val) => {
                            set.insert(*val);
                        }
                        APart::Range(_, _) => return None,
                        _ => {}
                    }
                }
                Some(())
            },
            Values::Div(values, values1) => {
                values.get_unknowns_helper(set)?;
                values1.get_unknowns_helper(set)?;
                Some(())
            },
            Values::Xor(values, values1) => {
                values.get_unknowns_helper(set)?;
                values1.get_unknowns_helper(set)?;
                Some(())
            },
            Values::Mod8(values) => {
                values.get_unknowns_helper(set)?;
                Some(())
            },
            Values::Possibilities(div_unknowns, others ) => {
                todo!()
            }
        }
    }

    fn get_unknowns(&self) -> Option<HashSet<usize>> {
        let mut set = HashSet::new();

        self.get_unknowns_helper(&mut set).map(|_| set)
    }

    fn apply(self, vals: &HashMap<usize, u8>) -> Self {
        match self {
            Values::Literal(_) => self,
            Values::A => self.reduce().apply(vals),
            Values::AParts(vec) => Values::AParts(vec.into_iter().flat_map(|part| {
                match part {
                    APart::A(i) => {
                        vec![if let Some(&val) = vals.get(&i) {
                            APart::Lit(val)      
                        } else {
                            APart::A(i)
                        }]
                    },
                    APart::Not(i) => {
                        vec![if let Some(&val) = vals.get(&i) {
                            APart::Lit((val+1)%2)      
                        } else {
                            APart::Not(i)
                        }]
                    },
                    APart::Lit(_) => vec![part],
                    APart::Range(start, end) => {
                        let mut max: Option<usize> = None;
                        for (&i, _) in vals {
                            if start <= i && i <= end {
                                if let Some(m) = max {
                                    max = Some(m.max(i));
                                } else {
                                    max = Some(i);
                                }
                            }
                        }
                    
                        if let Some(max) = max {
                            let mut n_parts = vec![];
                            for i in start..=max {
                                if let Some(&val) = vals.get(&i) {
                                    n_parts.push(APart::Lit(val))
                                } else {
                                    n_parts.push(APart::A(i))
                                }
                            }
                            if max != end {
                                n_parts.push(APart::Range(max+1, end));                            }
                            n_parts
                        } else {
                            vec![APart::Range(start, end)]
                        }
                    },
                }
            }).collect::<Vec<APart>>()).reduce(),
            Values::Div(values, values1) => Values::Div(
                Box::new(values.apply(vals)),
                Box::new(values1.apply(vals))
            ).reduce(),
            Values::Xor(values, values1) => Values::Xor(
                Box::new(values.apply(vals)),
                Box::new(values1.apply(vals))
            ).reduce(),
            Values::Mod8(values) => Values::Mod8(
                Box::new(values.apply(vals))
            ).reduce(),
            Values::Possibilities(vec, hash_map) => todo!(),
        }
    }
}

#[derive(Clone, Debug)]
struct ValueRegisters {
    a: Values,
    b: Values,
    c: Values,
}

fn get_num_bits(num: u128) -> u128 {
    let mut count = 0;
    let mut num = num;
    while num != 0 {
        count += 1;
        num /= 2;
    }  
    count
}

fn part2(program: &Vec<u128>) -> BigUint {
    let mut registers = ValueRegisters {
        a: Values::A,
        b: Values::Literal(0),
        c: Values::Literal(0)
    };

    let mut pc = 0;
    let mut out: Vec<Values> = vec![];

    while pc < program.len() {
        let inst = Instruction::from_digit(program[pc]);
        let literal = Values::Literal(program[pc+1]);
        let combo = match program[pc+1] {
            0..=3 => Values::Literal(program[pc+1]),
            4 => registers.a.clone(),
            5 => registers.b.clone(),
            6 => registers.c.clone(),
            _ => unreachable!()
        };

        pc += 2;

        match inst {
            Instruction::Adv => {
                // registers.a = registers.a / 2u128.pow(val as u32);
                registers.a = Values::Div(
                    Box::new(registers.a.clone()),
                    Box::new(combo)
                );
            },
            Instruction::Bxl => {
                // registers.b = registers.b ^ literal;
                registers.b = Values::Xor(
                    Box::new(registers.b.clone()),
                    Box::new(literal)
                );
            },
            Instruction::Bst => {
                // registers.b = val % 8;
                registers.b = Values::Mod8(Box::new(combo));
            },
            Instruction::Jnz => {
                // if registers.a != 0 {
                //     pc = literal as usize;
                // }
                if out.len() != program.len() {
                    pc = 0;
                }
            },
            Instruction::Bxc => {
                // registers.b = registers.b ^ registers.c;
                registers.b = Values::Xor(
                    Box::new(registers.b.clone()),
                    Box::new(registers.c.clone())
                );
            },
            Instruction::Out => {
                // out.push(val % 8);
                out.push(Values::Mod8(Box::new(combo)));
            }
            Instruction::Bdv => {
                // registers.b = registers.a / 2u128.pow(val as u32);
                registers.b = Values::Div(
                    Box::new(registers.a.clone()),
                    Box::new(combo)
                );
            },
            Instruction::Cdv => {
                // registers.c = registers.a / 2u128.pow(val as u32);
                registers.c = Values::Div(
                    Box::new(registers.a.clone()),
                    Box::new(combo)
                );
            }
        }
    }

    // let mut out_count = 0;
    let outs: Vec<Vec<HashMap<usize, u8>>> = out.into_iter().zip(program).map(|(out, prog)| {
        // out_count += 1;
        let out = out.reduce();
        // println!("new point ===============");
        // println!("{prog}: {:#?}", out);
        let (unknowns, poss) = if let Values::Possibilities(unknowns, poss) = out {
            (unknowns, poss)
        } else {
            unreachable!()
        };

        let poss: HashMap<usize, Values> = HashMap::from_iter(poss.into_iter().map(|(val, values)| {
            (val, match values {
                Values::Literal(_)
                | Values::AParts(_) => values,
                _ => unreachable!()
            })
        }));

        poss.into_iter().filter_map(|(val, values)| {
            let mut vals = HashMap::new();
            let mut lit = val;
            for i in 0..unknowns.len() {
                vals.insert(
                    unknowns[i],
                    (lit&1) as u8
                );
                lit /= 2;
            }

            match values.clone() {
                Values::AParts(parts) => {
                    let mut prog = *prog;
                    for val in parts {
                        match val {
                            APart::A(i) => {
                                let t = vals.insert(i, (prog&1) as u8);
                                assert!(t.is_none());
                            },
                            APart::Not(i) => {
                                let t = vals.insert(i, (((prog&1)+1)%2) as u8);
                                assert!(t.is_none());
                            },
                            APart::Lit(a) => {
                                if a == (prog&1) as u8 {
        
                                } else {
                                    return None;
                                }
                            },
                            APart::Range(_, _) => todo!(),
                        }
                        prog /= 2;
                    }
                },
                Values::Literal(lit) => {
                    if lit != *prog {
                        return None;
                    }
                },
                _ => unreachable!()
            }    

            Some(vals)
        }).collect()
     
    }).collect();

    let outs = outs.into_iter().reduce(|acc, next| {
        acc.iter().flat_map(|item1| {
            next.iter().filter_map(|item2| {
                let mut r_hash = HashMap::new();

                for (&i, &val) in item1 {
                    match item2.get(&i) {
                        Some(&val2) if val != val2 => return None,
                        _ => {
                            r_hash.insert(i, val);
                        }
                    }
                }
                for (&i, &val) in item2 {
                    match item1.get(&i) {
                        Some(&val2) if val != val2 => return None,
                        _ => {
                            r_hash.insert(i, val);
                        }
                    }
                }

                Some(r_hash)
            }).collect::<Vec<HashMap<usize, u8>>>()
        }).collect()
    }).unwrap();

    outs.into_iter().map(|out| {
        out.iter().fold(BigUint::ZERO, |acc, (&i, &val)| {
            if val == 0 {
                let mask = 1.to_biguint().unwrap().shl(i);
                if acc.clone()&mask.clone() != BigUint::ZERO {
                    acc ^ mask
                } else {
                    acc
                }
            } else {
                acc | 1.to_biguint().unwrap().shl(i)
            }
        })
    }).filter(|val| {
        let outs = part1(Registers { a: val.to_biguint().unwrap(), b: BigUint::ZERO, c: BigUint::ZERO}, program);
        if outs.len() != program.len() {
            return false;
        }
        for i in 0..outs.len() {
            if outs.len() != program.len() {
                return false;
            }
        }
        true
    }).min().unwrap()
}

fn main() {
    let (registers, program) = parse_input();
    println!("part1: {}", part1(registers, &program).into_iter().map(|val| val.to_string()).collect::<Vec<String>>().join(","));
    println!("part2: {}", part2(&program));

}

fn parse_input() -> (Registers, Vec<u128>) {
    let inp = include_str!("input.txt");
    
    let parts: Vec<&str> = inp.split("\n\n").collect();

    let registers = parts[0].split("\n").map(|line| {
        BigUint::from_str(line.split(": ").skip(1).next().unwrap()).unwrap()
    }).collect::<Vec<BigUint>>();
    let registers = Registers {
        a: registers[0].clone(),
        b: registers[1].clone(),
        c: registers[2].clone(),
    };


    let program_text = parts[1].split(": ").skip(1).next().unwrap();
    let program = program_text.split(",").map(|dig| {
        dig.parse::<u128>().unwrap()
    }).collect::<Vec<u128>>();

    (registers, program)
}