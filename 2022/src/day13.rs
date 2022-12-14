use std::fmt::{Display, Formatter, Result};
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
enum Item {
    Integer(u64),
    List(Vec<Item>)
}

impl Item {
    fn push(&mut self, item: Item) {
        match self {
            Self::List(vec) => vec.push(item),
            Self::Integer(_) => panic!("Cannot push an item to a non-list.")
        }
    }

    fn cmp(&self, other: &Item) -> Ordering {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => a.cmp(b),
            (Self::List(a), Self::List(b)) => {
                let mut ordering = Ordering::Equal;

                if a.is_empty() && b.is_empty() {
                    return Ordering::Equal;
                }

                if a.is_empty() {
                    return Ordering::Less;
                }

                if b.is_empty() {
                    return Ordering::Greater;
                }

                for i in 0..a.len().min(b.len()) {
                    ordering = a[i].cmp(&b[i]);

                    match ordering {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => {
                            if i == a.len() - 1 {
                                return Ordering::Less;
                            }

                            if i == b.len() - 1 {
                                return Ordering::Greater;
                            }
                        }
                    }
                }

                ordering
            },
            (Self::Integer(a), Self::List(_)) => Self::List(vec![Self::Integer(*a)]).cmp(other),
            (Self::List(_), Self::Integer(b)) => self.cmp(&Self::List(vec![Self::Integer(*b)])),
        }
    }

    fn rec_print(&self, fmt: &mut Formatter<'_>) -> Result {
        match self {
            Item::Integer(number) => write!(fmt, "{}", number),
            Item::List(items) => {
                items
                    .iter()
                    .enumerate()
                    .fold(write!(fmt, "["), |result, (index, item)| {
                        result
                            .and(item.rec_print(fmt))
                            .and(write!(fmt, "{}", if index < items.len() - 1 { "," } else { "" }))
                    })
                    .and(write!(fmt, "]"))
            }
        }
    }
}

impl Display for Item {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        self.rec_print(fmt)
    }
}

fn main() {
    let input = include_str!("../input/day13.input");

    let sum = first_puzzle(input);
    println!("The sum of the indices of the well-ordered pairs is {}.", sum);

    let decoder_key = second_puzzle(input);
    println!("The decoder key for the distress signal is {}.", decoder_key);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day13.input");

    assert_eq!(first_puzzle(sample), 13);
    assert_eq!(second_puzzle(sample), 140);
}

#[test]
fn equality() {
    let a = Item::List(vec![Item::List(vec![Item::Integer(2)])]);
    let b = Item::List(vec![Item::List(vec![Item::Integer(2)])]);
    assert!(a.eq(&b));
}

fn first_puzzle(source: &str) -> usize {
    let mut count = 0;

    source
        .split("\n\n")
        .enumerate()
        .for_each(|(pair_index, lines)| {
            let mut iter = lines.lines();

            let first = parse_line(iter.next().unwrap());
            let second = parse_line(iter.next().unwrap());

            if first.cmp(&second).is_le() {
                count += pair_index + 1;
            }
        });

    count
}

fn second_puzzle(source: &str) -> usize {
    let mut packets: Vec<Item> = source
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect();

    packets.push(divider(2));
    packets.push(divider(6));
    
    packets.sort_by(|a, b| a.cmp(b));
    let (divider2_index, _) = packets.iter().enumerate().find(|(_, packet)| divider(2).eq(packet)).expect("Could not find divider packet.");
    let (divider6_index, _) = packets.iter().enumerate().find(|(_, packet)| divider(6).eq(packet)).expect("Could not find divider packet.");
    
    (divider2_index + 1) * (divider6_index + 1)
}

fn divider(value: u64) -> Item {
    Item::List(vec![Item::List(vec![Item::Integer(value)])])
}

fn parse_line(source: &str) -> Item {
    let mut chars = source.chars().enumerate().peekable();
    let mut stack = vec![];

    loop {
        if chars.peek().is_none() {
            break;
        }

        let (i, c) = chars.peek().unwrap();

        if *c == '[' {
            stack.push(Item::List(vec![]));
            chars.next();
            continue;
        }

        if *c == ']' {
            if stack.len() == 1 {
                break;
            }

            let top = stack.pop().unwrap();
            stack.last_mut().unwrap().push(top);

            chars.next();
            continue;
        }

        if c.is_ascii_digit() {
            let start_index = *i;
            let mut end_index = start_index;

            loop {
                let (_, next_c) = chars.peek().unwrap();

                if !next_c.is_ascii_digit() {
                    break;
                }

                end_index += 1;
                chars.next();
            }

            let number_str = &source[start_index..end_index];
            let number = number_str.parse::<u64>().expect("Invalid number.");
            let top = stack.last_mut().unwrap();
            
            top.push(Item::Integer(number));
            //println!("{}", top);
            continue;
        }

        if c.is_ascii_whitespace() || *c == ',' {
            chars.next();
            continue;
        }

        panic!("Found invalid character: {}", c);
    }

    stack.pop().unwrap()
}