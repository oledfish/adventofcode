use std::{collections::HashMap, vec};

#[derive(Clone, Copy)]
enum BlockReason {
    Wall,
    Floor,
    RockSide,
    RockTop
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i64,
    y: i64
}

trait Movable {
    fn move_delta(&mut self, delta: Position, map: &Map) -> Option<BlockReason>;
    fn settle(&mut self, map: &mut Map);
    fn top(&self) -> i64;
}

impl Movable for Vec<Position> {
    fn move_delta(&mut self, delta: Position, map: &Map) -> Option<BlockReason> {
        let mut reason = None;
        let can_move = self.iter().all(|pos| {
            let target = Position {
                x: pos.x + delta.x,
                y: pos.y + delta.y,
            };

            let mut pebble_reason = None;
            if target.x < map.left_wall || target.x >= map.right_wall {
                pebble_reason = Some(BlockReason::Wall);
                reason = pebble_reason;
            }

            if target.y <= map.floor {
                pebble_reason = Some(BlockReason::Floor);
                reason = pebble_reason;
            }

            if delta.x != 0 && map.rocks.contains_key(&target) {
                pebble_reason = Some(BlockReason::RockSide);
            }

            if delta.y != 0 && map.rocks.contains_key(&target) {
                pebble_reason = Some(BlockReason::RockTop);
                reason = pebble_reason;
            }

            pebble_reason.is_none()
        });

        if can_move {
            for pos in self {
                pos.x += delta.x;
                pos.y += delta.y;
            }
        }

        reason
    }

    fn settle(&mut self, map: &mut Map) {
        for pos in self {
            map.rocks.insert(*pos, true);
        }
    }

    fn top(&self) -> i64 {
        let mut max = i64::MIN;
        for pos in self {
            if pos.y > max {
                max = pos.y;
            }
        }

        max
    }
}

struct Map {
    left_wall: i64,
    right_wall: i64,
    floor: i64,
    rocks: HashMap<Position, bool>
}

fn main() {
    let input = include_str!("../input/day17.input");

    // Part one
    let height_small = solve(input, 2022);
    println!("The tower of rocks is {} units tall after 2022 rocks have stopped falling.", height_small);

    // Part two
    let height_large = solve(input, 1000000000000);
    println!("The tower of rocks is {} units tall after 1000000000000 rocks have stopped falling.", height_large);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day17.input");

    assert_eq!(solve(sample, 2022), 3068);
    assert_eq!(solve(sample, 1000000000000), 1514285714288);
}

fn get_shape(index: u64, start: &Position) -> Vec<Position> {
    let mut shape = vec![];

    match index % 5 {
        0 => { // - shape
            shape.push(Position { x: start.x, y: start.y });
            shape.push(Position { x: start.x + 1, y: start.y });
            shape.push(Position { x: start.x + 2,  y: start.y });
            shape.push(Position { x: start.x + 3, y: start.y });
        }

        1 => { // + shape
            shape.push(Position { x: start.x + 1, y: start.y });
            shape.push(Position { x: start.x, y: start.y + 1 });
            shape.push(Position { x: start.x + 1, y: start.y + 1 });
            shape.push(Position { x: start.x + 2, y: start.y + 1 });
            shape.push(Position { x: start.x + 1, y: start.y + 2 });
        }

        2 => { // ⅃ shape
            shape.push(Position { x: start.x, y: start.y });
            shape.push(Position { x: start.x + 1, y: start.y });
            shape.push(Position { x: start.x + 2, y: start.y });
            shape.push(Position { x: start.x + 2, y: start.y + 1 });
            shape.push(Position { x: start.x + 2, y: start.y + 2 });
        }

        3 => { // | shape
            shape.push(Position { x: start.x, y: start.y });
            shape.push(Position { x: start.x, y: start.y + 1 });
            shape.push(Position { x: start.x, y: start.y + 2 });
            shape.push(Position { x: start.x, y: start.y + 3 });
        }

        4 => { // # shape
            shape.push(Position { x: start.x, y: start.y });
            shape.push(Position { x: start.x + 1, y: start.y });
            shape.push(Position { x: start.x, y: start.y + 1 });
            shape.push(Position { x: start.x + 1, y: start.y + 1 });
        }

        _ => unreachable!()
    }

    shape
}

fn calc_heights(map: &Map, top: i64) -> [i64; 7] {
    let mut heights = [top.abs(); 7];

    for x in 0..7 {
        for y in (0..top).rev() {
            let pos = Position { x, y };
            if map.rocks.contains_key(&pos) {
                heights[x as usize] = (top - y).abs();
                break;
            }
        }
    }

    heights
}

fn solve(source: &str, limit: u64) -> u64 {
    let total_gusts = source.len() as u64;
    let mut wind = source.chars().cycle();

    let mut map = Map {
        left_wall: 0,
        right_wall: 7,
        floor: 0,
        rocks: HashMap::<Position, bool>::new()
    };

    let mut index = 0;
    let mut top = map.floor;
    let mut gusts = 0;
    let mut heights;

    let mut cache = HashMap::<(u64, u64, [i64; 7]), (u64, i64)>::new();

    loop {
        if index == limit {
            break;
        }

        heights = calc_heights(&map, top);

        let key = (index % 5, gusts % total_gusts, heights);
        if cache.contains_key(&key) {
            // By this point the simulation has run the non-looping part, and a single loop
            // iteration, so we must determine where the loop starts and ends.

            // What we have is the index/height at where it starts looping (the current values), 
            // and the index/height where the loop began (the cached values).

            // With the two above we calculate the start and end indexes of the loop as well
            // as the starting height and end height for the loop length and height.
            // With this we divide the remaining number of rocks by the loop length
            // and add the height by the multiplier.

            // To calculate the height after the leftover part, we find where the loop cuts off,
            // find the index/height in cache, and apply accordingly.

            let (loop_index_start, loop_height_min) = cache[&key];

            let loop_index_end = index;
            let loop_height_max = top;

            let loop_len  = loop_index_end - loop_index_start;
            let loop_height = loop_height_max - loop_height_min;

            let times = (limit - index) / loop_len;

            let leftover_rocks = (limit - index) % loop_len;
            let (_, leftover_height) = cache.iter().find(|(_, (cached_index, _))| {
                *cached_index == (loop_index_start + leftover_rocks)
            }).unwrap().1;

            top += loop_height * times as i64 + (leftover_height - loop_height_min);

            break;
        }

        cache.insert(key, (index, top));

        let start = Position { x: map.left_wall + 2, y: top + 4 };
        let mut rock = get_shape(index, &start);
        index += 1;

        loop {
            // First move by wind 
            let c = wind.next().unwrap();
            let delta = match c {
                '>' => Position { x: 1, y: 0 },
                '<' => Position { x: -1, y: 0 },
                _ => panic!("Invalid wind direction.")
            };

            gusts += 1;

            let moved = rock.move_delta(delta, &map);
            if let Some(m) = moved {
                if matches!(m, BlockReason::Floor) || matches!(m, BlockReason::RockTop) {
                    rock.settle(&mut map);
                    top = top.max(rock.top());
                    heights = calc_heights(&map, top);
                    break;
                }
            }

            // Then move down
            let delta = Position { x: 0, y: -1 };
            let moved = rock.move_delta(delta, &map);
            if let Some(m) = moved {
                if matches!(m, BlockReason::Floor) || matches!(m, BlockReason::RockTop) {
                    rock.settle(&mut map);
                    top = top.max(rock.top());
                    heights = calc_heights(&map, top);
                    break;
                }
            }
        }

        top = top.max(rock.top());
    }

    top as u64
}