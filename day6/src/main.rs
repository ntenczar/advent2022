use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("input")?;
    for line in BufReader::new(file).lines() {
        //input file only has one line so this is silly but w/e
        let line = line.unwrap();
        let pos = first_marker_pos(&line, 4);
        println!("Location of first marker: {}", pos);
        let pos = first_marker_pos(&line, 14);
        println!("Location of first marker, slice size 14: {}", pos);
    }
    Ok(())
}

// assume that there's no way there isn't one before the string ends
fn first_marker_pos(input: &str, slice_size: usize) -> usize {
    let char_vec: Vec<char> = input.chars().collect();
    let num_chars = char_vec.len();
    for idx in 0..(num_chars - slice_size) {
        let slice: Vec<&char> = char_vec[idx..idx + slice_size].into_iter().collect();
        let any_twice: bool = slice
            .clone()
            .into_iter()
            .map(|c| does_char_appear_twice(c, slice.clone()))
            .any(|b| b);
        // if none appeared twice we have our index (plus four)
        if !any_twice {
            return idx + slice_size;
        }
    }
    panic!("no markers detected");
}

#[test]
fn test_first_marker_pos() {
    assert!(first_marker_pos("bvwbjplbgvbhsrlpgdmjqwftvncz", 4) == 5);
    assert!(first_marker_pos("nppdvjthqldpwncqszvftbrmjlhg", 4) == 6);
    assert!(first_marker_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4) == 10);
    assert!(first_marker_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4) == 11);
}

fn does_char_appear_twice(c: &char, input: Vec<&char>) -> bool {
    let count = input.into_iter().filter(|x| *x == c).count();
    return count > 1;
}

#[test]
fn test_does_char_appear_twice() {
    assert!(does_char_appear_twice(&'c', vec![&'c', &'c']));
    assert!(!does_char_appear_twice(&'c', vec![&'a', &'c']));
}
