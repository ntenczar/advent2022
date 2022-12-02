use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Debug)]
struct Elf {
    cals: Vec<u32>,
}

fn main() -> Result<()> {
    let file = File::open("input")?;
    let mut elves: Vec<Elf> = Vec::new();
    let mut current_elf = Elf { cals: Vec::new() };
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        // break here
        if line == "" {
            elves.push(current_elf);
            current_elf = Elf { cals: Vec::new() };
        } else {
            let parsed = line.parse::<u32>().unwrap();
            current_elf.cals.push(parsed);
        }
    }
    elves.push(current_elf);
    elves.sort_by(|a, b| sum_elf(b).partial_cmp(&sum_elf(a)).unwrap());
    println!("Max elf: {}", sum_elf(&elves[0]));
    let top_three = &elves[0..3];
    let sum_top_three: u32 = top_three.into_iter().map(|e| sum_elf(&e)).sum();
    println!("Sum of the top three: {}", sum_top_three);
    Ok(())
}

fn sum_elf(elf: &Elf) -> u32 {
    elf.cals.iter().sum()
}
