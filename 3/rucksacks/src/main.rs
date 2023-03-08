use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut lines = String::new();
    file.read_to_string(&mut lines)?;

    let mut sum: u128 = 0;

    for line in lines.lines() {
        let half: usize = line.len() / 2;
        let first = &line[0..half];
        let second = &line[half..];
        let mixed_char: char = find_mixed_char(first, second);
        sum += u128::from(priority(mixed_char));
    }

    println!("{sum}");

    Ok(())
}

/*
TODO: Change this method to return a Result(char,E) instead.
*/
fn find_mixed_char(_first_group: &str, _second_group: &str) -> char {
    for a_char in _first_group.chars() {
        if _second_group.contains(a_char) {
            return a_char;
        }
    }
    panic!("No repeated char found.");
}

fn priority(_c: char) -> u32 {
    let b: u32 = _c.into();
    if (97..=122).contains(&b) {
        b - 96
    } else if (65..=90).contains(&b) {
        b - 38
    } else {
        0
    }
}
