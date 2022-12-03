use std::collections::HashSet;

fn main() {
    let input = include_str!("../input/day03.input");

    // Part one
    let total = first_puzzle(input);
    println!("The sum of all priorities is {}.", total);

    // Part two
    let total = second_puzzle(input);
    println!("The sum of the priorities of all badges is {}.", total);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day03.input");

    assert_eq!(first_puzzle(sample), 157);
    assert_eq!(second_puzzle(sample), 70);
}

fn first_puzzle(source: &str) -> u64 {
    source
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(first, second)| priority(find_match(first, second)))
        .sum()
}

fn second_puzzle(source: &str) -> u64 {
    let mut lines = source.lines().peekable();
    let mut count = 0;

    loop {
        if lines.peek().is_none() {
            break;
        }

        let line1 = lines.next().unwrap();
        let _ = lines.next().unwrap();
        let line3 = lines.next().unwrap();

        // Take the first line slice and the third line slice to form a slice that contains all three lines
        let start = line1.as_ptr() as usize - source.as_ptr() as usize;
        let end = line3.as_ptr() as usize - source.as_ptr() as usize + line3.len();

        count += priority(find_badge(&source[start..end]));
    }

    count
}
 
fn find_match(line1: &str, line2: &str) -> char {
    assert_eq!(line1.len(), line2.len(), "Rucksacks must be of the same size");
    assert!(line1.is_ascii(), "Invalid format");
    assert!(line2.is_ascii(), "Invalid format");

    let mut result = None;
    line1
        .chars()
        .for_each(|c| {
            if line2.contains(c) {
                result = Some(c);
            }
        });

    if result.is_none() {
        panic!("No match found");
    }

    result.unwrap()
}

fn find_badge(group: &str) -> char {
    assert!(group.is_ascii(), "Invalid format");

    let mut lines = group.lines();

    let line1 = lines.next().unwrap();
    let line2 = lines.next().unwrap();
    let line3 = lines.next().unwrap();

    let mut result = None;
    line1
        .chars()
        .for_each(|c| {
            if line2.contains(c) && line3.contains(c) {
                result = Some(c);
            }
        });

    if result.is_none() {
        panic!("No badge found");
    }

    result.unwrap()
}

fn priority(character: char) -> u64 {
    match character {
        character @ 'a'..='z' => character as u64 - 96,
        character @ 'A'..='Z' => character as u64 - 38,
        _ => panic!("Invalid character.")
    }
}