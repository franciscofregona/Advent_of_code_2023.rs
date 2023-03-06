use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut lines_string = String::new();
    file.read_to_string(&mut lines_string)?;

    let mut sum: u32 = 0;

    for line in lines_string.lines() {
        let a: Vec<&str> = line.trim().split(',').collect();
        if a.len() != 2 {
            continue;
        }
        let first: Range = Range::new(a[0]);
        let second: Range = Range::new(a[1]);

        if first.contains(&second) || second.contains(&first) {
            println!("One contains the other: {} - {}", a[0], a[1]);
            sum += 1;
        }
    }
    // let mut r = Range::new(3, 4);
    println!("{}", sum);

    Ok(())
}

#[derive(Debug)]
struct Range {
    min: i32,
    max: i32,
}

impl Range {
    fn new(min_max: &str) -> Range {
        let a: Vec<&str> = min_max.trim().split('-').collect();
        if a.len() != 2 {
            return Range { min: 0, max: 0 };
        }
        Range {
            min: a[0].trim().parse().unwrap(),
            max: a[1].trim().parse().unwrap(),
        }
    }

    fn contains(&self, other: &Range) -> bool {
        self.min <= other.min && self.max >= other.max
    }
}
