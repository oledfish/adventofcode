#[derive(PartialEq, Eq, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Outcome {
    Win,
    Lose,
    Draw
}

fn main() {
    let input = include_str!("../input/day02.input");

    // Part one
    let points_incorrect = first_puzzle(input);
    println!("Your final score would be {} points with the incorrectly decrypted guide", points_incorrect);

    // Part two
    let points_correct = second_puzzle(input);
    println!("Your final score would be {} points with the correctly decrypted guide.", points_correct);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day02.input");

    assert_eq!(first_puzzle(sample), 15);
    assert_eq!(second_puzzle(sample), 12);
}

fn first_puzzle(source: &str) -> i32 {
    source
        .lines()
        .map(parse_plays)
        .map(|(player, enemy)| points(player, play(player, enemy)))
        .sum()
}

fn second_puzzle(source: &str) -> i32 {
    source
        .lines()
        .map(parse_outcome)
        .map(|(enemy, desired_outcome)| (ensure_outcome(enemy, desired_outcome), enemy))
        .map(|(player, enemy)| points(player, play(player, enemy)))
        .sum()
}

fn parse_plays(line: &str) -> (Play, Play) {
    let mut letters = line.split(' ').take(2);

    let enemy = match letters.next().unwrap() {
        "A" => Play::Rock,
        "B" => Play::Paper,
        "C" => Play::Scissors,
        _ => unreachable!()
    };

    let player = match letters.next().unwrap() {
        "X" => Play::Rock,
        "Y" => Play::Paper,
        "Z" => Play::Scissors,
        _ => unreachable!()
    };

    (player, enemy)
}

fn parse_outcome(line: &str) -> (Play, Outcome) {
    let mut letters = line.split(' ').take(2);

    let enemy = match letters.next().unwrap() {
        "A" => Play::Rock,
        "B" => Play::Paper,
        "C" => Play::Scissors,
        _ => unreachable!()
    };

    let desired_outcome = match letters.next().unwrap() {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => unreachable!()
    };

    (enemy, desired_outcome)
}

fn ensure_outcome(enemy: Play, desired_outcome: Outcome) -> Play {
    match desired_outcome {
        Outcome::Win => wins_against(enemy),
        Outcome::Lose => wins_against(wins_against(enemy)),
        Outcome::Draw => enemy,
    }
}

fn play(player: Play, enemy: Play) -> Outcome {
    if player == wins_against(enemy) { return Outcome::Win }
    if enemy == wins_against(player) { return Outcome::Lose }

    Outcome::Draw
}

fn wins_against(play: Play) -> Play {
    match play {
        Play::Rock => Play::Paper,
        Play::Paper => Play::Scissors,
        Play::Scissors => Play::Rock
    }
}

fn points(play: Play, outcome: Outcome) -> i32 {
    let mut count = 0;

    count += match play {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3
    };

    count += match outcome {
        Outcome::Win => 6,
        Outcome::Lose => 0,
        Outcome::Draw => 3
    };

    count
}