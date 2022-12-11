struct MonkeyTurn {
    monkey_num: usize,
    starting_items: Vec<u64>,
    operator: char,
    operand: Option<u64>,
    divisor: u64,
    true_monkey_num: usize,
    false_monkey_num: usize
}

fn main() {
    let input = include_str!("../input/day11.input");

    let monkey_business = first_puzzle(input);
    println!("The level of monkey business after 20 rounds is {}.", monkey_business);

    let monkey_business_hard = second_puzzle(input);
    println!("The level of monkey business after 10000 rounds is {}.", monkey_business_hard);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day11.input");

    assert_eq!(first_puzzle(sample), 10605);
    assert_eq!(second_puzzle(sample), 2713310158);
}

fn first_puzzle(source: &str) -> u64 {
    solve(source, 20, false)
}

fn second_puzzle(source: &str) -> u64 {
    solve(source, 10000, true)
}

fn solve(source: &str, num_rounds: usize, use_supermodulo: bool) -> u64 {
    let mut turns: Vec<MonkeyTurn> = source
        .split("\n\n")
        .map(parse_monkey)
        .collect();

    let mut monkeys = vec![vec![]; turns.len()];
    let mut inspect = vec![0; turns.len()];

    for turn in turns.iter_mut() {
        monkeys[turn.monkey_num].append(&mut turn.starting_items);
    }

    // Unfortunately, needed hints for this, couldn't solve it on my own
    let mut supermodulo = None;
    if use_supermodulo {
        supermodulo = Some(turns
            .iter()
            .map(|turn| turn.divisor)
            .fold(1u64, |mut mul, num| { mul *= num; mul }))
    }

    for _ in 1..=num_rounds {
        for turn in &turns {
            monkey_turn(turn, supermodulo, &mut monkeys, &mut inspect);
        }
    }

    inspect.sort_by(|a, b| b.cmp(a));
    inspect[0] * inspect[1]
}

fn parse_monkey(source: &str) -> MonkeyTurn {
    let mut lines = source.lines();

    let monkey_num_line = lines.next().unwrap();
    if !monkey_num_line.starts_with("Monkey ") {
        panic!("Could not parse monkey number.");
    }

    let monkey_num = monkey_num_line
        .strip_prefix("Monkey ")
        .unwrap()
        .strip_suffix(':')
        .unwrap()
        .parse::<usize>().expect("Invalid monkey number.");

    let starting_items_line = lines.next().unwrap();
    if !starting_items_line.starts_with("  Starting items: ") {
        panic!("Could not parse starting items.");
    }

    let starting_items: Vec<u64> = starting_items_line
        .strip_prefix("  Starting items: ")
        .unwrap()
        .split(", ")
        .map(|num| num.parse::<u64>().expect("Invalid item."))
        .collect();

    let operation_line = lines.next().unwrap();
    if !operation_line.starts_with("  Operation: new = old ") {
        panic!("Could not parse operation.");
    }

    let expression_right_half = operation_line.strip_prefix("  Operation: new = old ").unwrap();

    let operator = expression_right_half.chars().next().unwrap();
    if operator != '+' && operator != '*' {
        panic!("Invalid operator.");
    }

    let operand_str = &expression_right_half[2..];
    let mut operand = None;
    if operand_str != "old" {
        operand = Some(operand_str.parse::<u64>().expect("Invalid number."));
    }

    let test_line = lines.next().unwrap();
    if !test_line.starts_with("  Test: divisible by ") {
        panic!("Could not parse test.");
    }

    let divisor = test_line.strip_prefix("  Test: divisible by ").unwrap().parse::<u64>().expect("Invalid division number.");

    let true_line = lines.next().unwrap();
    if !true_line.starts_with("    If true: throw to monkey ") {
        panic!("Could not parse true condition.");
    }

    let true_monkey_num = true_line.strip_prefix("    If true: throw to monkey ").unwrap().parse::<usize>().expect("Invalid monkey number.");

    let false_line = lines.next().unwrap();
    if !false_line.starts_with("    If false: throw to monkey ") {
        panic!("Could not parse false condition.");
    }

    let false_monkey_num = false_line.strip_prefix("    If false: throw to monkey ").unwrap().parse::<usize>().expect("Invalid monkey number.");

    MonkeyTurn { 
        monkey_num, 
        starting_items, 
        operator, 
        operand, 
        divisor, 
        true_monkey_num, 
        false_monkey_num 
    }

}

fn monkey_turn(turn: &MonkeyTurn, supermodulo: Option<u64>, monkeys: &mut [Vec<u64>], inspect: &mut [u64]) {
    let items = monkeys[turn.monkey_num].clone();

    for item in items {
        inspect[turn.monkey_num] += 1;

        let mut worry_level = item;
        let initial_worry_level = item;

        let operand = if turn.operand.is_none() { worry_level } else { turn.operand.unwrap() };

        match turn.operator {
            '+' => worry_level += operand,
            '*' => worry_level *= operand,
            _ => unreachable!()
        };

        if let Some(s) = supermodulo {
            worry_level %= s;
        } else {
            worry_level /= 3;
        }

        let thrown_index = if worry_level % turn.divisor == 0 { turn.true_monkey_num } else { turn.false_monkey_num };
        monkeys[thrown_index].push(worry_level);
        monkeys[turn.monkey_num].retain(|level| *level != initial_worry_level);
    }
}