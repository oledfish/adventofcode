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
 
fn find_match(first: &str, second: &str) -> char {
    assert_eq!(first.len(), second.len(), "Rucksacks must be of the same size");
    assert!(first.is_ascii(), "Invalid format");
    assert!(second.is_ascii(), "Invalid format");

    let first_chars: HashSet<char> = HashSet::from_iter(first.chars());
    let second_chars: HashSet<char> = HashSet::from_iter(second.chars());

    let mut matching = first_chars.intersection(&second_chars);

    *matching.next().unwrap()
}

fn find_badge(group: &str) -> char {
    assert!(group.is_ascii(), "Invalid format");

    let mut lines = group.lines();

    let line1 = lines.next().unwrap();
    let sack1: HashSet<char> = HashSet::from_iter(line1.chars());

    let line2 = lines.next().unwrap();
    let sack2: HashSet<char> = HashSet::from_iter(line2.chars());

    let line3 = lines.next().unwrap();
    let sack3: HashSet<char> = HashSet::from_iter(line3.chars());

    // Three-way intersection, I don't actually know how it works
    *sack1.iter().filter(|k| sack2.contains(k)).find(|k| sack3.contains(k)).unwrap()
}

fn priority(character: char) -> u64 {
    match character {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => panic!("Invalid character.")
    }
}