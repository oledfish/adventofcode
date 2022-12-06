struct Move {
    amount: usize,
    from: usize,
    to: usize
}

fn main() {
    let input = include_str!("../input/day05.input");

    // Part one
    let word9000 = first_puzzle(input);
    println!("The word formed by the stacks (CrateMover 9000) is {}.", word9000);

    // Part two
    let word9001 = second_puzzle(input);
    println!("The word formed by the stacks (CrateMover 9001) is {}.", word9001);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day05.input");

    assert_eq!(first_puzzle(sample), "CMZ");
    assert_eq!(second_puzzle(sample), "MCD");
}

fn first_puzzle(source: &str) -> String {
    solve(source, false)
}

fn second_puzzle(source: &str) -> String {
    solve(source, true)
}

fn solve(source: &str, upgraded: bool) -> String {
    let mut parts = source.split("\n\n").peekable();

    let stacks_num = (parts.peek().unwrap().len() + 1) / 4;
    let mut stacks = vec![vec![]; stacks_num];

    parts
        .next()
        .unwrap()
        .lines()
        .rev()
        .filter(|line| !line.starts_with(" 1"))
        .for_each(|row| {
            let crates_len = (row.len() + 1) / 4;

            for i in 0..crates_len {
                let correction = if i > 0 {1} else {0};
                let slice = &row[(i*(3 + correction))..(i*(3 + correction) + 3)];

                if slice.starts_with('[') && slice.ends_with(']') {
                    let letter = &slice[1..slice.len() - 1].parse::<char>().unwrap();
                    stacks[i].push(*letter);
                } 
            }
        });

    parts
        .next()
        .unwrap()
        .lines()
        .map(parse_command)
        .for_each(|command| execute_command(command, &mut stacks, upgraded));

    let mut word = String::default();

    for stack in stacks {
        if stack.first().is_some() {
            word.push(*stack.last().unwrap());
        }
    }
    
    word
}

fn execute_command(command: Move, stacks: &mut [Vec<char>], upgraded: bool) {
    if !upgraded {
        for _ in 0..command.amount {
            let letter = stacks[command.from - 1].pop().unwrap();
            stacks[command.to - 1].push(letter);
        }
    } else  {
        let from = &mut stacks[command.from - 1];
        let mut letters: Vec<char> = from
            .drain((from.len() - command.amount)..)
            .collect();
        
        stacks[command.to - 1].append(&mut letters);
    }
}

fn parse_command(command_str: &str) -> Move {
    let mut words = command_str.split(' ');

    let move_str = words.next().expect("Expected \"move\".");
    assert_eq!(move_str, "move");

    let amount = words.next().unwrap().parse::<usize>().expect("Number was not in a valid format");

    let from_str = words.next().expect("Expected \"from\".");
    assert_eq!(from_str, "from");

    let from = words.next().unwrap().parse::<usize>().expect("Number was not in a valid format");

    let to_str = words.next().expect("Expected \"to\".");
    assert_eq!(to_str, "to");

    let to = words.next().unwrap().parse::<usize>().expect("Number was not in a valid format");

    Move { amount, from, to }
}