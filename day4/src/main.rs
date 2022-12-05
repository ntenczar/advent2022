use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("input")?;
    let mut full_overlap = 0;
    let mut any_overlap = 0;
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split(",").collect();
        if do_sections_fully_overlap(split[0], split[1]) {
            full_overlap = full_overlap + 1;
        }
        if do_sections_overlap(split[0], split[1]) {
            any_overlap = any_overlap + 1;
        }
    }
    println!("Num fully overlapping sections: {}", full_overlap);
    println!("Num any overlap: {}", any_overlap);
    Ok(())
}

fn parse_section(section: &str) -> (usize, usize) {
    let split: Vec<&str> = section.split("-").collect();
    let start: usize = split[0].parse::<usize>().unwrap();
    let end: usize = split[1].parse::<usize>().unwrap();
    return (start, end);
}

fn do_sections_fully_overlap(first_section: &str, second_section: &str) -> bool {
    let (first_start, first_end) = parse_section(first_section);
    let (second_start, second_end) = parse_section(second_section);

    if first_start <= second_start && first_end >= second_end {
        return true;
    }
    if second_start <= first_start && second_end >= first_end {
        return true;
    }
    return false;
}

fn do_sections_overlap(first_section: &str, second_section: &str) -> bool {
    let (first_start, first_end) = parse_section(first_section);
    let (second_start, second_end) = parse_section(second_section);

    if first_start <= second_start && first_end >= second_start {
        return true;
    }
    if second_start <= first_start && second_end >= first_start {
        return true;
    }
    return false;
}

#[test]
fn test_parse_section() {
    let (start, end) = parse_section("2-5");
    assert!(start == 2);
    assert!(end == 5);
    let (start, end) = parse_section("8-40");
    assert!(start == 8);
    assert!(end == 40);
}

#[test]
fn test_overlap_sections() {
    assert!(do_sections_fully_overlap("2-4", "2-4"));
    assert!(do_sections_fully_overlap("1-5", "2-4"));
    assert!(do_sections_fully_overlap("2-8", "3-5"));
}

#[test]
fn test_any_overlap() {
    assert!(do_sections_overlap("2-4", "2-4"));
    assert!(do_sections_overlap("1-4", "2-4"));
    assert!(do_sections_overlap("1-4", "4-8"));
    assert!(!do_sections_overlap("1-2", "3-4"));
    assert!(!do_sections_overlap("0-1", "2-3"));
    assert!(do_sections_overlap("0-1", "1-3"));
}
