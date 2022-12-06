#![allow(unused_imports)]
use std::{collections::VecDeque, iter::zip};

fn main() {
    let input = include_str!("../input/day06.input");

    // Part one
    let first_packet_marker = first_puzzle(input);
    println!("First start-of-packet marker detected after {} characters.", first_packet_marker);

    // Part two
    let first_message_marker = second_puzzle(input);
    println!("First start-of-message marker detected after {} characters.", first_message_marker);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day06.input");

    let results = [(7, 19), (5, 23), (6, 23), (10, 29), (11, 26)];
    let lines = sample.lines();

    zip(results.iter(), lines)
        .for_each(|(results, line)| {
            assert_eq!(first_puzzle(line), results.0);
            assert_eq!(second_puzzle(line), results.1);
        });
}

fn first_puzzle(source: &str) -> usize {
    solve(source, 4)
}

fn second_puzzle(source: &str) -> usize {
    solve(source, 14)
}

fn solve(source: &str, marker_size: usize) -> usize {
    let mut different = VecDeque::with_capacity(4);

    for (index, character) in source.chars().enumerate() {
        if different.contains(&character) {
            loop {
                let c = different.pop_front().expect("No more elements in queue.");

                if c == character {
                    break;
                }
            }

            different.push_back(character);
            continue;
        } else {
            different.push_back(character);
        }

        if different.len() == marker_size {
            return index + 1;
        }
    }

    0
}