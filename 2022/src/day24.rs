use std::{collections::{HashMap, HashSet, VecDeque}, hash::Hash};

const STEPS: u64 = 16;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
struct Vector2 {
    x: i64,
    y: i64
}

enum Tile {
    Ground,
    Wall
}

#[derive(Default)]
struct Map {
    layout: HashMap<Vector2, Tile>,
    blizzards: Vec<(Vector2, Vector2)>, // left: position, right: direction
    open_ground: HashMap<Vector2, HashSet<u64>>, // for each tile, at which minutes it was uncovered
    simulated_moves: u64,

    width: i64,
    height: i64,
    lcm: u64,
    
    start: Vector2,
    goal: Vector2,
}

fn main() {
    let input = include_str!("../input/day24.input");

    let steps = first_puzzle(input);
    println!("Avoiding the blizzards, the goal can be reached in {} steps.", steps);

    let round = second_puzzle(input);
    println!("A trip to the goal, back to the start and again to the goal can be done in {} steps.", round);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day24.input");

    assert_eq!(first_puzzle(sample), 18);
    assert_eq!(second_puzzle(sample), 54);
}

fn first_puzzle(source: &str) -> u64 {
    let mut map = parse_map(source);

    for _ in 1..=map.lcm {
        simulate_blizzards(&mut map);
    }

    let start = map.start;
    let goal = map.goal;

    search_iter(&mut map, start, goal, 0)
}

fn second_puzzle(source: &str) -> u64 {
    let mut map = parse_map(source);

    for _ in 1..=map.lcm {
        simulate_blizzards(&mut map);
    }

    let start = map.start;
    let goal = map.goal;
    
    let first = search_iter(&mut map, start, goal, 1);
    let second = search_iter(&mut map, goal, start, first + 1);
    let third = search_iter(&mut map, start, goal, second + 1);

    third
}

fn search_iter(map: &mut Map, start: Vector2, goal: Vector2, starting_minutes: u64) -> u64 {
    let mut visited = HashSet::<(Vector2, u64)>::new();
    let mut queue = VecDeque::<(Vector2, u64)>::new();
    visited.insert((start, starting_minutes));
    queue.push_front((start, starting_minutes));

    loop {
        if queue.is_empty() {
            return u64::MAX;
        }

        let (pos, minutes) = queue.pop_back().unwrap();

        if pos == goal {
            return minutes;
        }

        let up = Vector2 { x: pos.x, y: pos.y - 1 };
        let down = Vector2 { x: pos.x, y: pos.y + 1 };
        let left = Vector2 { x: pos.x - 1, y: pos.y };
        let right = Vector2 { x: pos.x + 1, y: pos.y };

        let adjacent = [pos, up, down, left, right];

        for dir in adjacent {
            if let Some(tile) = map.layout.get(&dir) {
                let next = (dir, (minutes + 1));
                let test = (dir, (minutes + 1) % map.lcm);

                if matches!(tile, Tile::Ground) && !visited.contains(&test) && map.open_ground.get(&dir).unwrap().contains(&((minutes + 1) % map.lcm)) {
                    visited.insert(test);
                    queue.push_front(next);
                }
            }
        }
    }
}

fn search(map: &mut Map, cache: &mut HashMap<(Vector2, u64), u64>, minima: &mut u64, pos: Vector2, minutes: u64) -> u64 {
    if minutes > *minima {
        return u64::MAX;
    }

    if pos.x == map.goal.x && pos.y == map.goal.y {
        if minutes < *minima {
            *minima = minutes;
        }

        return minutes - 1;
    }

    let mut min = u64::MAX;

    let up = Vector2 { x: pos.x, y: pos.y - 1 };
    let down = Vector2 { x: pos.x, y: pos.y + 1 };
    let left = Vector2 { x: pos.x - 1, y: pos.y };
    let right = Vector2 { x: pos.x + 1, y: pos.y };

    let adjacent = [up, down, left, right];

    let mut stay = true;
    for dir in adjacent {
        if dir == map.start {
            continue;
        }

        if let Some(tile) = map.layout.get(&dir) {
            if matches!(tile, Tile::Ground) && map.open_ground.get(&dir).unwrap().contains(&(minutes % map.lcm)) {
                let key = (dir, (minutes + 1) % map.lcm as u64);

                let path = if cache.contains_key(&key) {
                    *cache.get(&key).unwrap()
                } else {
                    search(map, cache, minima, dir, minutes + 1)
                };

                min = min.min(path);
                stay = false;
            }
        }
    }

    if stay {
        if !map.open_ground.get(&pos).unwrap().contains(&(minutes % map.lcm)) {
            return u64::MAX;
        }

        let key = (pos, (minutes + 1) % map.lcm as u64);

        let path = if cache.contains_key(&key) {
            *cache.get(&key).unwrap()
        } else {
            search(map, cache, minima, pos, minutes + 1)
        };

        min = min.min(path);
    }

    let key = (pos, minutes % map.lcm as u64);
    cache.insert(key, min);
    
    min
}

fn simulate_blizzards(map: &mut Map) {
    map.simulated_moves += 1;

    for (pos, dir) in map.blizzards.iter_mut() {
        let target = Vector2 { x: pos.x + dir.x, y: pos.y + dir.y };

        // Or at least it SHOULDN'T happen
        if (target.x == map.start.x && target.y == map.start.y) || (target.x == map.goal.x && target.y == map.goal.y) {
            unreachable!();
        }

        if dir.x == -1 && target.x == 0 {
            pos.x = map.width - 2;
            continue;
        }

        if dir.x == 1 && target.x == map.width - 1 {
            pos.x = 1;
            continue;
        }

        if dir.y == -1 && target.y == 0 {
            pos.y = map.height - 2;
            continue;
        }

        if dir.y == 1 && target.y == map.height - 1 {
            pos.y = 1;
            continue;
        }

        *pos = target;
    }

    for y in 0..=map.height {
        for x in 0..=map.width {
            let pos = Vector2 { x: x as i64, y: y as i64};

            if map.layout.contains_key(&pos) && !map.blizzards.iter().any(|(b_pos, _)| b_pos.x == pos.x && b_pos.y == pos.y) {
                let tile = map.layout.get(&pos).unwrap();

                if matches!(tile, Tile::Ground) {
                    map.open_ground.get_mut(&pos).unwrap().insert(map.simulated_moves);
                }
            }
        }
    }
}

fn parse_map(source: &str) -> Map {
    let mut map = Map {
        width: source.lines().next().unwrap().chars().count() as i64,
        height: source.lines().count() as i64,
        ..Default::default()
    };

    map.lcm = lcm(map.width - 2, map.height - 2) as u64;

    source
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line
                .chars()
                .enumerate()
                .for_each(|(x, tile)| {
                    let pos = Vector2 { x: x as i64, y: y as i64};

                    match tile {
                        '.' => { map.layout.insert(pos, Tile::Ground); },
                        '#' => { map.layout.insert(pos, Tile::Wall); },
                        '^' => { map.layout.insert(pos, Tile::Ground); map.blizzards.push((pos, Vector2 { x:  0,  y: -1})); },
                        'v' => { map.layout.insert(pos, Tile::Ground); map.blizzards.push((pos, Vector2 { x:  0,  y:  1})); },
                        '<' => { map.layout.insert(pos, Tile::Ground); map.blizzards.push((pos, Vector2 { x: -1,  y:  0})); },
                        '>' => { map.layout.insert(pos, Tile::Ground); map.blizzards.push((pos, Vector2 { x:  1,  y:  0})); },
                        _ => panic!("Invalid tile")
                    };

                    if tile == '.' && y == 0 {
                        map.start = pos;
                    }

                    if tile == '.' && y as i64 == map.height - 1 {
                        map.goal = pos;
                    }

                    if tile != '#' {
                        map.open_ground.insert(pos, HashSet::<u64>::new());
                    }
                })
        });

    for y in 0..=map.height {
        for x in 0..=map.width {
            let pos = Vector2 { x: x as i64, y: y as i64};

            if map.layout.contains_key(&pos) && !map.blizzards.iter().any(|(b_pos, _)| *b_pos == pos) {
                let tile = map.layout.get(&pos).unwrap();

                if matches!(tile, Tile::Ground) {
                    map.open_ground.get_mut(&pos).unwrap().insert(0);
                }
            }
        }
    }

    map
}

fn lcm(first: i64, second: i64) -> i64 {
    first * second / gcd(first, second)
}

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}