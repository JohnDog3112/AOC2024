use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Registers {
    a: u128,
    b: u128,
    c: u128,
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

    fn to_val(self, registers: &Registers) -> Option<u128> {
        match self {
            Self::Zero => Some(0),
            Self::One => Some(1),
            Self::Two => Some(2),
            Self::Three => Some(3),
            Self::A => Some(registers.a),
            Self::B => Some(registers.b),
            Self::C => Some(registers.c),
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


fn part1(registers: Registers, program: &Vec<u128>) {
    let mut registers = registers;
    let mut pc = 0;

    let mut out: Vec<u128> = vec![];

    while pc < program.len() {
        let inst = Instruction::from_digit(program[pc]);
        let literal = program[pc+1];
        let combo = Combo::from_num(literal);
        
        let val = combo.to_val(&registers).unwrap();

        // println!("{pc}: {registers:?}");
        // println!("{}, {}", program[pc], program[pc+1]);
        pc += 2;


        match inst {
            Instruction::Adv => {
                registers.a = registers.a / 2u128.pow(val as u32);
            },
            Instruction::Bxl => {
                registers.b = registers.b ^ literal;
            },
            Instruction::Bst => {
                registers.b = val % 8;
            },
            Instruction::Jnz => {
                if registers.a != 0 {
                    pc = literal as usize;
                }
            },
            Instruction::Bxc => {
                registers.b = registers.b ^ registers.c;
            },
            Instruction::Out => {
                out.push(val % 8);
            }
            Instruction::Bdv => {
                registers.b = registers.a / 2u128.pow(val as u32);
            },
            Instruction::Cdv => {
                registers.c = registers.a / 2u128.pow(val as u32);
            }
        }

    }
    println!("{registers:?}");
    println!("{}", out.into_iter().map(|a| a.to_string()).collect::<Vec<String>>().join(","));
}

#[derive(Clone, Debug, Copy)]
enum APart {
    A(usize),
    Not(usize),
    Lit(u8),
    Range(usize, usize)
}
#[derive(Clone, Debug)]
enum Values {
    Literal(u128),
    A,
    AParts(Vec<APart>),
    Div(Box<Values>, Box<Values>),
    Xor(Box<Values>, Box<Values>),
    Mod8(Box<Values>),
    Possibilities(Vec<usize>, HashMap<u32, Vec<APart>>),
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
                    ) if parts.len() == 1 && matches!(parts[0], APart::Range(_, _)) => {
                        if let APart::Range(start, end) = parts[0] {
                            Self::AParts(vec![APart::Range(
                                (start + lit as usize).min(end),
                                end
                            )])
                        } else {
                            unreachable!()
                        }
                    },
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
                                    println!("{parts:?}");
                                    unreachable!()
                                },
                                (false, _) => parts[i]
                            };
                            i += 1;
                            lit /= 2;
                        }
                        Values::AParts(parts)
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
            Values::AParts(vec) => Values::AParts(vec.into_iter().map(|part| {
                match part {
                    APart::A(i) => {
                        if let Some(&val) = vals.get(&i) {
                            APart::Lit(val)      
                        } else {
                            APart::A(i)
                        }
                    },
                    APart::Not(i) => {
                        if let Some(&val) = vals.get(&i) {
                            APart::Lit((val+1)%2)      
                        } else {
                            APart::Not(i)
                        }
                    },
                    APart::Lit(_) => part,
                    APart::Range(_, _) => part,
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
            Values::Mod8(values) => todo!(),
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

fn part2(program: &Vec<u128>) {
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

    out.into_iter().zip(program).for_each(|(out, prog)| {
        println!("{prog}: {:#?}", out.reduce());
    });

}

fn main() {
    let (registers, program) = parse_input();
    part1(registers, &program);
    part2(&program);

}

fn parse_input() -> (Registers, Vec<u128>) {
    let inp = include_str!("input.txt");
    
    let parts: Vec<&str> = inp.split("\n\n").collect();

    let registers = parts[0].split("\n").map(|line| {
        line.split(": ").skip(1).next().unwrap().parse::<u128>().unwrap()
    }).collect::<Vec<u128>>();
    let registers = Registers {
        a: registers[0],
        b: registers[1],
        c: registers[2]
    };


    let program_text = parts[1].split(": ").skip(1).next().unwrap();
    let program = program_text.split(",").map(|dig| {
        dig.parse::<u128>().unwrap()
    }).collect::<Vec<u128>>();

    (registers, program)
}