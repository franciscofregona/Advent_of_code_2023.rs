//File read to string
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::Split;

fn main() -> std::io::Result<()> {
    // let mut file = File::open("try")?;
    // let mut file = File::open("input_test.txt")?;
    let mut file = File::open("input.txt")?;
    let mut lines = String::new();
    file.read_to_string(&mut lines)?;

    //Step 1: find the contents of the stacks
    // lines have the following format, some gaps may appear within crates but not within indexes
    // [W] [T] [P] [J] [C] [G] [W] [P] [J]
    // 1   2   3   4   5   6   7   8   9
    // Columns are 2, 6, 10, 14, 18, 22, 26, 30, 34...
    // Formula is, then C(x) = 4x+2, with x starting from 0.
    let mut ship: HashMap<usize, Vec<char>> = HashMap::new();
    let mut parsing_movements: bool = false;

    for line in lines.lines() {
        if parsing_movements == false {
            let parsed_line = parse_crate_contents(line);
            if parsed_line.is_empty() {
                parsing_movements = true;
            } else {
                //play parsed initial positions
                for item in parsed_line {
                    let column = index_to_column(item.0);
                    match ship.get(&column) {
                        Some(vector) => {
                            let mut vector = vector.to_owned();
                            vector.push(item.1);
                            ship.insert(column, vector);
                        }
                        _ => {
                            ship.insert(column, vec![item.1]);
                        }
                    }
                }
            }
        } else {
            match parse_move(line) {
                Some((qty, origin, destination)) => {
                    for _ in 0..qty {
                        match ship.get(&origin) {
                            Some(origin_v) => match ship.get(&destination) {
                                Some(destination_v) => {
                                    let mut destination_v = destination_v.to_owned();
                                    let mut origin_v = origin_v.to_owned();
                                    // let a_crate: char = origin_v.pop().unwrap();
                                    // destination_v.push(a_crate);
                                    destination_v.insert(0, origin_v.remove(0));
                                    ship.insert(destination, destination_v);
                                    ship.insert(origin, origin_v);
                                }
                                _ => {
                                    panic!(
                                        "Illegal move, failed destination: {qty},{origin},{destination}",
                                    );
                                }
                            },
                            _ => {
                                panic!("Illegal move, failed origin: {qty},{origin},{destination}",);
                            }
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    for k in ship.keys().into_iter().sorted() {
        print!(
            "{}",
            ship.get(k).unwrap_or(&vec!['a'; 0]).first().unwrap_or(&' ')
        );
    }
    println!();
    Ok(())
}

fn _count_brackets(line: &str) -> usize {
    line.chars().filter(|ch| *ch == '[').count()
}

// Take a string with some box contents and return the tuples with their indexes
// ie:
// parse_crate_contents("[N] [C]") -> [(2, N), (6, C)]
fn parse_crate_contents(line: &str) -> Vec<(usize, char)> {
    line.char_indices()
        .filter(|ch| ch.1.is_ascii_uppercase())
        .collect()
}

fn index_to_column(index: usize) -> usize {
    (index - 1) / 4 + 1
}

//move 1 from 2 to 1 -> qty, origin, destination
fn parse_move(line: &str) -> Option<(usize, usize, usize)> {
    if line.trim() == "" {
        return None;
    }

    let a: Split<&str> = line.split(" ");
    let b = a
        .filter(|x| *x != "move")
        .filter(|x| *x != "to")
        .filter(|x| *x != "from");
    let mut c = b.map(|x| x.parse::<usize>().unwrap());
    let mut una_tupla: (usize, usize, usize) = (0, 0, 0);
    una_tupla.0 = c.next()?;
    una_tupla.1 = c.next()?;
    una_tupla.2 = c.next()?;

    Some(una_tupla)
}
