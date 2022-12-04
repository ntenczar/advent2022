use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("input")?;
    let mut sum = 0;
    let mut lines: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let (first_half, second_half) = halve_string(line.clone());
        let shared = shared_char(first_half, second_half);
        let pos = position_in_alphabet(shared);
        sum = sum + pos;
        lines.push(line);
    }
    let groups_of_three = lines.chunks(3);
    let mut groups_of_three_sum = 0;
    for group in groups_of_three {
        let shared =
            shared_char_group_of_three(group[0].clone(), group[1].clone(), group[2].clone());
        groups_of_three_sum = groups_of_three_sum + position_in_alphabet(shared);
    }
    println!("Sum of shared chars {}", sum);
    println!("Sum of grouped shared chars {}", groups_of_three_sum);
    Ok(())
}

fn halve_string(full_str: String) -> (String, String) {
    let length = full_str.len();
    (
        full_str[0..(length / 2)].to_string(),
        full_str[(length / 2)..length].to_string(),
    )
}

fn shared_char(first_half: String, second_half: String) -> char {
    let first_hash: HashSet<char> = first_half.chars().collect();
    for c in second_half.chars() {
        if first_hash.contains(&c) {
            return c;
        }
    }
    panic!("no shared char");
}

fn shared_char_group_of_three(first: String, second: String, third: String) -> char {
    let first_hash: HashSet<char> = first.chars().collect();
    let second_hash: HashSet<char> = second.chars().collect();
    for c in third.chars() {
        if first_hash.contains(&c) && second_hash.contains(&c) {
            return c;
        }
    }
    panic!("no shared char");
}

fn position_in_alphabet(c: char) -> usize {
    let mut alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    return alphabet.position(|a| a == c).unwrap() + 1;
}

#[test]
fn pos_in_alphabet_test() {
    assert!(position_in_alphabet('a') == 1);
    assert!(position_in_alphabet('A') == 27);
    assert!(position_in_alphabet('Z') == 52);
}
