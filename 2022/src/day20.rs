fn main() {
    let input = include_str!("../input/day20.input");

    // Part one
    let sum = first_puzzle(input);
    println!("The sum of the numbers that form the grove coordinates is {}.", sum);

    // Part two
    let sum_key = second_puzzle(input);
    println!("The sum of the numbers that form the grove coordinates (with the decryption key) is {}.", sum_key);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day20.input");

    assert_eq!(first_puzzle(sample), 3);
    assert_eq!(second_puzzle(sample), 1623178306);
}

fn first_puzzle(source: &str) -> i64 {
    let message: Vec<(i64, usize)> = source
        .lines()
        .enumerate()
        .map(|(index, num)| (num.parse::<i64>().expect("Invalid number"), index))
        .collect();

    solve(message, 1)
}

fn second_puzzle(source: &str) -> i64 {
    let decryption_key = 811589153;
    let message: Vec<(i64, usize)> = source
        .lines()
        .enumerate()
        .map(|(index, num)| (decryption_key * num.parse::<i64>().expect("Invalid number"), index))
        .collect();

    solve(message, 10)
}

fn solve(message: Vec<(i64, usize)>, rounds: usize) -> i64 {
    let mut mixed = message.clone();

    for _ in 0..rounds {
        for value in &message {
            mix(&mut mixed, value);
        }
    }

    let start_index = mixed
        .iter()
        .enumerate()
        .find(|(_, val)| val.0 == 0)
        .expect("Couldn't find 0.")
        .0;

    let first = mixed[(start_index + 1000).rem_euclid(mixed.len())].0;
    let second = mixed[(start_index + 2000).rem_euclid(mixed.len())].0;
    let third = mixed[(start_index + 3000).rem_euclid(mixed.len())].0;

    first + second + third
}

fn mix(vec: &mut Vec<(i64, usize)>, value: &(i64, usize)) {
    if value.0 == 0 {
        return;
    }

    let true_index = vec
        .iter()
        .enumerate()
        .find(|(_, val)| val.0 == value.0 && val.1 == value.1)
        .expect("Couldn't find entry.")
        .0;

    vec.remove(true_index);
    vec.insert((true_index as i64 + value.0).rem_euclid(vec.len() as i64) as usize, *value);
}