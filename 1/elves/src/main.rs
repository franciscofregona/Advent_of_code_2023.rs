use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut lines = String::new();
    file.read_to_string(&mut lines)?;

    let mut current_elf: u128 = 1;
    let mut current_carrying: u128 = 0;
    let mut best_elf: u128 = 1;
    let mut best_carrying: u128 = 0;

    for line in lines.lines() {
        current_carrying = match line.trim().parse::<u128>() {
            Ok(value) => {
                if current_carrying + value > best_carrying {
                    best_carrying = current_carrying + value;
                    best_elf = current_elf;
                }
                current_carrying + value
            }
            _ => {
                current_elf += 1;
                0
            }
        }
    }
    println!("Best elf is {} with {} kg.", best_elf, best_carrying);
    Ok(())
}
