use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut lines_string = String::new();
    file.read_to_string(&mut lines_string)?;

    let mut total_score: u128 = 0;

    for line in lines_string.lines() {
        let pair: Vec<&str> = line.split(' ').collect();
        let theirs: &str = pair[0];
        let mine: &str = pair[1];

        total_score += calculate(theirs, mine);
    }

    println!("{}", total_score);

    Ok(())
}

fn calculate(theirs: &str, mine: &str) -> u128 {
    // let theirs: u128 = theirs.trim().parse().unwrap_or_default();
    // let mine: u128 = mine.trim().parse().unwrap_or_default();
    match mine {
        "X" => {
            1 + match theirs {
                "A" => 3,
                "B" => 0,
                "C" => 6,
                _ => 0,
            }
        }
        "Y" => {
            2 + match theirs {
                "A" => 6,
                "B" => 3,
                "C" => 0,
                _ => 0,
            }
        }
        "Z" => {
            3 + match theirs {
                "A" => 0,
                "B" => 6,
                "C" => 3,
                _ => 0,
            }
        }
        _ => 0,
    }
}
