
#[derive(Clone, Copy)]
enum Slot {
    Filled(i16),
    Empty
}

fn part1(inp: Vec<u8>) -> i64 {
    let mut drive: Vec<Slot> = vec![];
    let mut next_id = 0;
    for (i, len) in inp.into_iter().enumerate() {
        if i%2 == 0 {
            let id = next_id;
            next_id += 1;
            for _ in 0..len {
                drive.push(Slot::Filled(id));
            }
        } else {
            for _ in 0..len {
                drive.push(Slot::Empty);
            }
        }
    }

    let mut l_ptr = 0;
    let mut r_ptr = drive.len() - 1;

    loop {
        while let Slot::Filled(_) = &drive[l_ptr] {
            l_ptr += 1;
        }
        while let Slot::Empty = drive[r_ptr] {
            r_ptr -= 1;
        }
        if l_ptr > r_ptr {
            break;
        }

        drive[l_ptr] = drive[r_ptr];
        drive[r_ptr] = Slot::Empty;
    }
    

    // println!("{}", drive.iter().map(|d| 
    //     match d {
    //         Slot::Filled(d) => d.to_string(),
    //         Slot::Empty => ".".to_string()
    //     }
    // ).collect::<Vec<String>>().join(""));

    drive.into_iter().enumerate().map(|(i, slot)| {
        match slot {
            Slot::Filled(id) => id as usize*i,
            Slot::Empty => 0,
        }
    }).sum::<usize>() as i64
}

fn part2(inp: Vec<u8>) -> i64 {
    let mut drive: Vec<Slot> = vec![];
    let mut next_id = 0;
    for (i, len) in inp.into_iter().enumerate() {
        if i%2 == 0 {
            let id = next_id;
            next_id += 1;
            for _ in 0..len {
                drive.push(Slot::Filled(id));
            }
        } else {
            for _ in 0..len {
                drive.push(Slot::Empty);
            }
        }
    }

    let mut file_ptr = (drive.len() - 1) as i64;
    while 0 <= file_ptr {
        let id = match drive[file_ptr as usize] {
            Slot::Filled(id) => id,
            Slot::Empty => {
                file_ptr -= 1;
                continue;
            }
        };
        let mut file_len = 0;
        while (file_ptr - file_len) >= 0 {
            match drive[file_ptr as usize - file_len as usize] {
                Slot::Filled(tmp_id) if id == tmp_id => {
                    file_len += 1;
                },
                _ => break
            }
        }
        file_ptr -= file_len - 1;

        let mut l_ptr = 0;
        let mut consecutive_empty = 0;
        while l_ptr < file_ptr {
            match drive[l_ptr as usize] {
                Slot::Empty => {
                    consecutive_empty += 1;
                    if consecutive_empty >= file_len {
                        let empty_start = l_ptr - consecutive_empty + 1;
                        for i in empty_start..(empty_start + consecutive_empty) {
                            drive[i as usize] = Slot::Filled(id);
                        }
                        for i in file_ptr..(file_ptr + file_len) {
                            drive[i as usize] = Slot::Empty;
                        }
                        break;
                    }
                },
                _ => {
                    consecutive_empty = 0;
                }
            }

            l_ptr += 1;
        }
        file_ptr -= 1;
    }

    // println!("{}", drive.iter().map(|d| 
    //     match d {
    //         Slot::Filled(d) => d.to_string(),
    //         Slot::Empty => ".".to_string()
    //     }
    // ).collect::<Vec<String>>().join(""));
    

    drive.into_iter().enumerate().map(|(i, slot)| {
        match slot {
            Slot::Filled(id) => id as usize*i,
            Slot::Empty => 0,
        }
    }).sum::<usize>() as i64
}
fn main() {
    let inp = get_input();
    println!("part1: {}", part1(inp.clone()));
    println!("part2: {}", part2(inp));

}

pub fn get_input() -> Vec<u8> {
    let inp = include_str!("input.txt");

    inp.split('\n').next().unwrap().chars().map(|ch| {
        ch.to_digit(10).unwrap() as u8
    }).collect()
}