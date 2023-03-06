use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut lines = String::new();
    file.read_to_string(&mut lines)?;

    let mut sum: u128 = 0;

    // desugar the -for- into a loop over an iterator and save it so we can call next()
    let mut it = lines.lines().into_iter();
    while let Some(line) = it.next() {
        let first = line;
        let second = it.next().unwrap();
        let third = it.next().unwrap();
        let mixed_char: char = find_mixed_char(first, second, third);
        sum += u128::from(priority(mixed_char));
    }

    println!("{sum}");

    Ok(())
}

/*
TODO: Change this method to return a Result(char,Er) instead.
*/
fn find_mixed_char(first_group: &str, second_group: &str, third_group: &str) -> char {
    for a_char in first_group.chars() {
        if second_group.contains(a_char) {
            if third_group.contains(a_char) {
                return a_char;
            }
        }
    }
    panic!("No repeated char found.");
}

fn priority(_c: char) -> u32 {
    let b: u32 = _c.into();
    if b >= 97 && b <= 122 {
        return b - 96;
    } else if b >= 65 && b <= 90 {
        return b - 38;
    } else {
        return 0;
    }
}
