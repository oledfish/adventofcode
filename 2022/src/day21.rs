use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone)]
struct Monkey {
    operation: Option<Operation>,
    result: Option<i64>
}

#[derive(Debug, Clone)]
struct Operation {
    a: String,
    b: String,
    operator: char
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.operation, &self.result) {
            (Some(_), Some(r)) => write!(f, "{}", r),
            (Some(op), None) => write!(f, "{} {} {}", op.a, op.operator, op.b),
            (None, Some(r)) => write!(f, "{}", r),
            (None, None) => write!(f, "x")
        }
    }
}

impl Operation {
    fn eval(&self, monkeys: &HashMap<String, Monkey>) -> Option<i64> {
        if !monkeys.contains_key(&self.a) || !monkeys.contains_key(&self.b) {
            return None;
        }

        let a_monkey = monkeys.get(&self.a).unwrap();
        let b_monkey = monkeys.get(&self.b).unwrap();

        if a_monkey.result.is_some() && b_monkey.result.is_some() {
            let a = a_monkey.result.unwrap();
            let b = b_monkey.result.unwrap();

            return match self.operator {
                '+' => Some(a + b),
                '-' => Some(a - b),
                '*' => Some(a * b),
                '/' => Some(a / b),
                '=' => Some((a == b) as i64),
                _ => unreachable!()
            }
        }

        None
    }
}

fn main() {
    let input = include_str!("../input/day21.input");

    let root = first_puzzle(input);
    println!("The monkey named \"root\" will yell {}.", root);

    let humn = second_puzzle(input);
    println!("You must yell the number {} to pass the equality test.", humn);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day21.input");

    assert_eq!(first_puzzle(sample), 152);
    assert_eq!(second_puzzle(sample), 301.0);
}

fn first_puzzle(source: &str) -> i64 {
    let mut monkeys = parse_monkeys(source);
    eval_unsolved(&mut monkeys);

    monkeys.get("root").unwrap().result.unwrap()
}

fn second_puzzle(source: &str) -> f64 {
    let mut monkeys = parse_monkeys(source);
    monkeys.get_mut("root").unwrap().operation.as_mut().unwrap().operator = '=';
    monkeys.get_mut("humn").unwrap().result = None;

    eval_unsolved(&mut monkeys);
    solve_equation("root", &monkeys)
}

fn solve_equation(start: &str, monkeys: &HashMap<String, Monkey>) -> f64 {
    let monkey = monkeys.get(start).unwrap();
    let monkey_left = monkeys.get(&monkey.operation.as_ref().unwrap().a).unwrap();
    let monkey_right = monkeys.get(&monkey.operation.as_ref().unwrap().b).unwrap();

    let eval_left = monkey_left.operation.as_ref().unwrap().eval(monkeys);
    let eval_right = monkey_right.operation.as_ref().unwrap().eval(monkeys);

    let mut result = eval_left.or(eval_right).unwrap() as f64;
    let first = if eval_left.is_some() && eval_right.is_none() { monkey_right } else { monkey_left };

    let mut stack = vec![];
    stack.push(first);

    loop {
        if stack.is_empty() {
            break;
        }

        let monkey = stack.pop().unwrap();

        if monkey.operation.is_none() && monkey.result.is_none() {
            break;
        }

        let operation = monkey.operation.as_ref().unwrap();

        let monkey_left = monkeys.get(&operation.a).unwrap();
        let monkey_right = monkeys.get(&operation.b).unwrap();

        let (left_undefined, right_undefined) = {
            if monkey_left.result.is_some() && monkey_right.result.is_none() {
                (false, true)
            } else {
                (true, false)
            }
        };

        match operation.operator {
            '+' => {
                if left_undefined { result -= monkey_right.result.unwrap() as f64; }
                if right_undefined { result -= monkey_left.result.unwrap() as f64; }
            }

            '*' => {
                if left_undefined { result /= monkey_right.result.unwrap() as f64; }
                if right_undefined { result /= monkey_left.result.unwrap() as f64; }
            }

            '-' => {
                if left_undefined { result += monkey_right.result.unwrap() as f64; }
                if right_undefined {  // (a - (x) = b)  == (-a + (x) = -b)
                    result = -result;
                    result += monkey_left.result.unwrap() as f64;
                }
            }

            '/' => {
                if left_undefined { result *= monkey_right.result.unwrap() as f64; }
                if right_undefined { // (a / (x) = b)  == (1/a * (x) = 1/b)
                    result = 1.0 / (result as f64);
                    result /= 1.0 / (monkey_left.result.unwrap() as f64);
                }
            }

            _ => unimplemented!()
        }

        if left_undefined {
            stack.push(monkey_left);
        }

        if right_undefined {
            stack.push(monkey_right);
        }
    }
    result
}

fn eval_unsolved(monkeys: &mut HashMap<String, Monkey>) {
    loop {
        let mut results = HashMap::<String, i64>::new();
        for (code, monkey) in monkeys.iter().filter(|(_, monkey)| monkey.operation.is_some() && monkey.result.is_none()) {
            if let Some(r) = monkey.operation.as_ref().unwrap().eval(monkeys) {
                results.insert(code.to_owned(), r);
            }
        }

        for (code, result) in &results {
            monkeys.get_mut(code).unwrap().result = Some(*result);
        }

        if results.is_empty() {
            break;
        }
    }
}

fn parse_monkeys(source: &str) -> HashMap<String, Monkey> {
    let mut monkeys = HashMap::<String, Monkey>::new();

    for line in source.lines() {
        let (code, monkey) = parse_line(line);
        monkeys.insert(code, monkey);
    }

    monkeys
}

fn parse_line(source: &str) -> (String, Monkey) {
    let mut split = source.split(": ");

    let code = split
        .next()
        .expect("String ended early.")
        .trim()
        .to_owned();

    let expr = split
        .next()
        .expect("String ended early.")
        .trim();

    let mut operation = None;
    let mut result = None;

    if expr.starts_with(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']) {
        result = Some(expr.parse::<i64>().expect("Invalid number"));
    } else {
        let mut expr_split = expr.split_ascii_whitespace();

        let a = expr_split.next().expect("Couldn't find left operand.").to_owned();
        let operator = expr_split.next().expect("Couldn't find operator.").chars().next().unwrap();
        let b = expr_split.next().expect("Couldn't find right operand.").to_owned();

        operation = Some(Operation { a, b, operator });
    }

    (code, Monkey { operation, result })
}

/*
fn full_equation(start: &str, monkeys: &HashMap<String, Monkey>) -> String {
    let monkey = monkeys.get(start).unwrap();

    match (&monkey.operation, &monkey.result) {
        (Some(_), Some(r)) => format!("{}", r),
        (None, Some(r)) => format!("{}", r),
        (None, None) => "x".to_string(),
        (Some(op), None) => { 
            if let Some(r) = op.eval(monkeys) {
                format!("{}", r)
            } else {
                if op.operator != '=' {
                    format!("({} {} {})", full_equation(&op.a, monkeys), op.operator, full_equation(&op.b, monkeys))
                } else {
                    format!("({} - {} = 0)", full_equation(&op.a, monkeys), full_equation(&op.b, monkeys))
                }
            }
        },
    }
}
*/