use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Elf {
    number: u128,
    carrying: u128,
}

impl Elf {
    fn new(number: u128, carrying: u128) -> Self {
        Elf { number, carrying }
    }
}

#[derive(Debug)]
struct Podium {
    places: [Elf; 4],
}

impl Podium {
    fn new() -> Self {
        Podium {
            places: [
                Elf::new(0, 0),
                Elf::new(0, 0),
                Elf::new(0, 0),
                Elf::new(0, 0),
            ],
        }
    }

    fn submit(&mut self, e: Elf) {
        self.places[0] = e;
        self.places.sort_by_key(|x| x.carrying)
    }

    fn sum(&self) -> u128 {
        self.places[1].carrying + self.places[2].carrying + self.places[3].carrying
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut lines = String::new();
    file.read_to_string(&mut lines)?;

    let mut current_elf: u128 = 1;
    let mut current_carrying: u128 = 0;
    let mut podium = Podium::new();

    let mut it = lines.lines().peekable();
    while let Some(line) = it.next() {
        current_carrying = match line.trim().parse::<u128>() {
            Ok(value) => {
                if it.peek().is_none() {
                    podium.submit(Elf::new(current_elf, current_carrying + value));
                }
                current_carrying + value
            }
            _ => {
                podium.submit(Elf::new(current_elf, current_carrying));
                current_elf += 1;
                0
            }
        }
    }
    println!("{}", podium.sum());
    Ok(())
}
