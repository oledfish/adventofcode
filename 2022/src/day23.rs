use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Vector2 {
    x: i64,
    y: i64
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East
}

fn main() {
    let input = include_str!("../input/day23.input");

    // Part one
    let empty = first_puzzle(input);
    println!("In the smallest rectangle encompassing the Elves after 10 rounds, there are {} empty ground tiles.", empty);

    // Part two
    let round = second_puzzle(input);
    println!("The Elves stop moving after round {}.", round);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day23.input");

    assert_eq!(first_puzzle(sample), 110);
    assert_eq!(second_puzzle(sample), 20);
}

fn first_puzzle(source: &str) -> usize {
    let mut map = parse_map(source);
    let mut order = [Direction::North, Direction::South, Direction::West, Direction::East];
    let rounds = 10;

    for _ in 1..=rounds {
        simulate_round(&mut map, &mut order);
    }

    let mut min = Vector2 { x: i64::MAX, y: i64::MAX };
    let mut max = Vector2 { x: i64::MIN, y: i64::MIN };

    for pos in map.keys() {
        min.x = min.x.min(pos.x);
        min.y = min.y.min(pos.y);

        max.x = max.x.max(pos.x);
        max.y = max.y.max(pos.y);
    }

    let mut empty = 0;

    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let pos = Vector2 { x, y };

            if !map.contains_key(&pos) {
                empty += 1;
            }
        }
    }

    empty
}

fn second_puzzle(source: &str) -> i64 {
    let mut map = parse_map(source);
    let mut order = [Direction::North, Direction::South, Direction::West, Direction::East];
    let mut num = 0;

    loop {
        let moved = simulate_round(&mut map, &mut order);
        num += 1;

        if !moved {
            return num;
        }
    }
}

fn simulate_round(map: &mut HashMap<Vector2, bool>, order: &mut [Direction]) -> bool {
    /*for y in min.y..=max.y {
        for x in min.x..=max.x {
            let pos = Vector2 { x, y };

            if map.contains_key(&pos) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!();*/
    
    let mut proposed = vec![];
    let mut moved = false;

    for pos in map.keys() {
        if let Some(proposed_pos) = peek(pos, order, map) {
            proposed.push((*pos, proposed_pos));
        }
    }

    let mut valid = vec![];

    for proposal in &proposed {
        if proposed.iter().filter(|(_, proposed_pos)| *proposed_pos == proposal.1).count() == 1 {
            valid.push(proposal);
        }
    }

    for proposal in valid {
        let from = proposal.0;
        let to = proposal.1;

        map.remove(&from);
        map.insert(to, true);

        moved = true;
    }

    order.rotate_left(1);

    moved
}

fn peek(pos: &Vector2, order: &[Direction], map: &HashMap<Vector2, bool>) -> Option<Vector2> {
    let n = Vector2 { x: pos.x, y: pos.y - 1 };
    let s = Vector2 { x: pos.x, y: pos.y + 1 };
    let w = Vector2 { x: pos.x - 1, y: pos.y };
    let e = Vector2 { x: pos.x + 1, y: pos.y };
    let ne = Vector2 { x: pos.x + 1, y: pos.y - 1 };
    let nw = Vector2 { x: pos.x - 1, y: pos.y - 1 };
    let se = Vector2 { x: pos.x + 1, y: pos.y + 1 };
    let sw = Vector2 { x: pos.x - 1, y: pos.y + 1 };

    let all = [n, s, w, e, ne, nw, se, sw];
    if all.iter().all(|dir| !map.contains_key(dir)) {
        return None;
    }

    for next in order {
        match *next {
            Direction::North => {
                if [n, ne, nw].iter().all(|dir| !map.contains_key(dir)) {
                    return Some(n);
                }
            }

            Direction::South => {
                if [s, se, sw].iter().all(|dir| !map.contains_key(dir)) {
                    return Some(s);
                }
            }

            Direction::West => {
                if [w, nw, sw].iter().all(|dir| !map.contains_key(dir)) {
                    return Some(w);
                }
            }

            Direction::East => {
                if [e, ne, se].iter().all(|dir| !map.contains_key(dir)) {
                    return Some(e);
                }
            }
        }
    }

    None
}

fn parse_map(source: &str) -> HashMap<Vector2, bool> {
    let mut map = HashMap::<Vector2, bool>::new();

    source
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line
                .chars()
                .enumerate()
                .for_each(|(x, tile)| {
                    let pos = Vector2 { x: x as i64, y: y as i64};

                    if tile != '#' && tile != '.' {
                        panic!("Invalid tile");
                    }

                    if tile == '#' {
                        map.insert(pos, true);
                    }
                })
        });

    map
}