use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("input")?;
    let mut score = 0;
    let mut second_run_score = 0;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let split: Vec<&str> = line.split(" ").collect();
        score = score + determine_points(split[0], split[1]);
        let alternate_winner = choose_winner(split[0], split[1]);
        second_run_score = second_run_score + determine_points(split[0], alternate_winner);
    }
    println!("Final score: {}", score);
    println!("Second run score: {}", second_run_score);
    Ok(())
}

// A/X rock, B/Y paper, C/Z scissors
fn determine_points(them: &str, you: &str) -> u32 {
    match (them, you) {
        ("A", "X") => 4,
        ("A", "Y") => 8,
        ("A", "Z") => 3,
        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,
        ("C", "X") => 7,
        ("C", "Y") => 2,
        ("C", "Z") => 6,
        (_, _) => panic!("unknown"),
    }
}

// X lose, Y draw, Z win
fn choose_winner<'a>(them: &'a str, outcome: &'a str) -> &'a str {
    match (them, outcome) {
        ("A", "X") => "Z",
        ("A", "Y") => "X",
        ("A", "Z") => "Y",
        ("B", "X") => "X",
        ("B", "Y") => "Y",
        ("B", "Z") => "Z",
        ("C", "X") => "Y",
        ("C", "Y") => "Z",
        ("C", "Z") => "X",
        (_, _) => panic!("unknown"),
    }
}
