use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let mut stacks: Vec<Vec<&str>> = vec![
        vec!["V", "N", "F", "S", "M", "P", "H", "J"],
        vec!["Q", "D", "J", "M", "L", "R", "S"],
        vec!["B", "W", "S", "C", "H", "D", "Q", "N"],
        vec!["L", "C", "S", "R"],
        vec!["B", "F", "P", "T", "V", "M"],
        vec!["C", "N", "Q", "R", "T"],
        vec!["R", "V", "G"],
        vec!["R", "L", "D", "P", "S", "Z", "C"],
        vec!["F", "B", "P", "G", "V", "J", "S", "D"],
    ];
    // reverse our stacks so that the "top" is the end of the list (working
    // with the end of the list is much easier)
    stacks = stacks
        .into_iter()
        .map(|s| s.into_iter().rev().collect())
        .collect();
    part_one(stacks.clone());
    return part_two(stacks);
}

fn part_one(stacks: Vec<Vec<&str>>) -> Result<()> {
    let file = File::open("input")?;
    let mut stacks = stacks.clone();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let (number, from, to) = parse_move(&line);
        for _i in 0..number {
            let popped = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(popped);
        }
    }
    let mut top: String = String::new();
    for s in stacks {
        top.push_str(s.last().unwrap());
    }
    println!("Top of the stacks: {:?}", top);
    Ok(())
}

fn part_two(stacks: Vec<Vec<&str>>) -> Result<()> {
    let file = File::open("input")?;
    let mut stacks = stacks.clone();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let (number, from, to) = parse_move(&line);
        let stack_height = stacks[from - 1].len();
        let popped: Vec<&str> = stacks[from - 1]
            .drain(stack_height - number..stack_height)
            .collect();
        stacks[to - 1].append(&mut popped.clone());
    }
    let mut top: String = String::new();
    for s in stacks {
        top.push_str(s.last().unwrap());
    }
    println!("Top of the stacks: {:?}", top);
    Ok(())
}

fn parse_move(move_str: &str) -> (usize, usize, usize) {
    let split: Vec<&str> = move_str.split(" ").collect();
    let number = split[1].parse::<usize>().unwrap();
    let from = split[3].parse::<usize>().unwrap();
    let to = split[5].parse::<usize>().unwrap();
    return (number, from, to);
}

#[test]
fn test_parse_move() {
    assert!(parse_move("move 1 from 8 to 4") == (1, 8, 4));
}
