use std::vec;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct Knot {
    x: i32,
    y: i32
}

impl Knot {
    fn diff(&self, other: &Knot) -> Knot {
        Knot {
            x: (self.x - other.x),
            y: (self.y - other.y)
        }
    }

    fn drag(&self, other: &mut Knot) {
        let diff = self.diff(other);

        if diff.x.abs() > 1 || diff.y.abs() > 1 {
            if diff.x.abs() == diff.y.abs() {
                other.x += if diff.x > 0 { 1 } else { -1 };
                other.y += if diff.y > 0 { 1 } else { -1 };
                return;
            }

            if diff.x.abs() > diff.y.abs() {
                other.x += if diff.x > 0 { 1 } else { -1 };
                other.y += diff.y;
                return;
            } else {
                other.x += diff.x;
                other.y += if diff.y > 0 { 1 } else { -1 };
                return;
            }
        }

        panic!("Invalid operation (moved knots in more steps than one).");
    }
}

#[derive(Clone, Copy)]
enum Move {
    Left(i32),
    Up(i32),
    Right(i32),
    Down(i32)
}

fn main() {
    let input = include_str!("../input/day09.input");
    
    // Part one
    let visited_short = first_puzzle(input);
    println!("The tail of the short rope visited {} positions at least once.", visited_short);

    // Part two
    let visited_long = second_puzzle(input);
    println!("The tail of the long rope visited {} positions at least once.", visited_long);
}

#[test]
fn sample() {
    let input = include_str!("../sample/day09.input");

    assert_eq!(first_puzzle(input), 13);
    assert_eq!(second_puzzle(input), 1);
}

fn first_puzzle(source: &str) -> usize {
    solve(source, &mut [Knot::default(); 2])
}

fn second_puzzle(source: &str) -> usize {
    solve(source, &mut [Knot::default(); 10])
}

fn solve(source: &str, rope: &mut [Knot]) -> usize {
    let mut visited = vec![Knot::default()];

    source
        .lines()
        .map(parse_move)
        .for_each(|direction| move_rope(direction, &mut visited, rope));

    visited.len()
}

fn parse_move(source: &str) -> Move {
    let mut parts = source.split(' ');
    let letter = parts.next().expect("Could not find a valid direction.");
    let amount = parts.next().expect("Could not find a valid amount.").parse::<i32>().expect("Invalid number");

    match letter {
        "L" => Move::Left(amount),
        "U" => Move::Up(amount),
        "R" => Move::Right(amount),
        "D" => Move::Down(amount),
        _ => panic!("Could not find a valid direction.")
    }
}

fn move_rope(direction: Move, visited: &mut Vec<Knot>, rope: &mut [Knot]) {
    match direction {
        Move::Left(amount) => {
            for _ in 0..amount {
                rope[0].x -= 1;
                drag_rope(visited, rope);
            }
        },
        Move::Up(amount) => {
            for _ in 0..amount {
                rope[0].y -= 1;
                drag_rope(visited, rope);
            }
        },
        Move::Right(amount) => {
            for _ in 0..amount {
                rope[0].x += 1;
                drag_rope(visited, rope);
            }
        },
        Move::Down(amount) => {
            for _ in 0..amount {
                rope[0].y += 1;
                drag_rope(visited, rope);
            }
        },
    }
}

fn drag_rope(visited: &mut Vec<Knot>, rope: &mut [Knot]) {
    for i in 1..rope.len() {
        let first = rope[i-1];
        let second = rope[i];

        if first == second {
            continue;
        }

        let distance = first.diff(&second);
        if distance.x.abs() > 1 || distance.y.abs() > 1 {
            first.drag(&mut rope[i]);
            //rope[i] = second;

            if i == rope.len() - 1 && !visited.contains(&rope[i]) {
                visited.push(rope[i]);
            }
        }
    }
}