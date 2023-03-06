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
    match mine {
        "X" => {
            // loose!
            0 + match theirs {
                "A" => 3,
                "B" => 1,
                "C" => 2,
                _ => 0,
            }
        }
        "Y" => {
            // draw
            3 + match theirs {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => 0,
            }
        }
        "Z" => {
            // win
            6 + match theirs {
                "A" => 2,
                "B" => 3,
                "C" => 1,
                _ => 0,
            }
        }
        _ => 0,
    }
}
